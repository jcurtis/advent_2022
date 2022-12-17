use std::collections::HashMap;

use itertools::Itertools;
use pathfinding::prelude::bfs;
use regex::Regex;

#[derive(Debug)]
struct Valve {
    flow_rate: u32,
    leads_to: Vec<V>,
}

type Distances = HashMap<V, u32>;

#[derive(Debug)]
struct ValveWithDistances {
    flow_rate: u32,
    // leads_to: Vec<V>,
    distances: Distances,
}

impl From<(&Valve, Distances)> for ValveWithDistances {
    fn from((valve, distances): (&Valve, Distances)) -> Self {
        Self {
            flow_rate: valve.flow_rate,
            // leads_to: valve.leads_to.clone(),
            distances,
        }
    }
}

type V = String;

type Input = HashMap<V, ValveWithDistances>;

#[aoc_generator(day16)]
fn generator(input: &str) -> Input {
    let re =
        Regex::new(r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();
    let valves: HashMap<V, Valve> = input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (
                V::from(&caps[1]),
                Valve {
                    flow_rate: caps[2].parse().unwrap(),
                    leads_to: caps[3].split(',').map(|v| v.trim().to_string()).collect(),
                },
            )
        })
        .collect();

    // Preload each valve with a distance to all other valves
    valves
        .iter()
        .map(|(from_v, valve)| {
            let distances = valves
                .keys()
                .map(|goal_v| {
                    let path = bfs(
                        from_v,
                        |v| {
                            let valve = valves.get(v).unwrap();
                            valve.leads_to.clone()
                        },
                        |v| v == goal_v,
                    )
                    .expect("Error building bfs path to {v}");
                    (goal_v.to_owned(), (path.len() - 1) as u32)
                })
                .collect();
            (
                from_v.to_owned(),
                ValveWithDistances {
                    flow_rate: valve.flow_rate,
                    // leads_to: valve.leads_to.clone(),
                    distances,
                },
            )
        })
        .collect()
}

#[aoc(day16, part1)]
fn part_1(input: &Input) -> u32 {
    let mut closed_valves = input.keys().collect_vec();

    let mut total_flow = 0;
    let mut total_flow_rate = 0;

    let mut current_v = "AA".to_string();
    let mut cycle = 0;
    while cycle < 30 {
        let pre = total_flow;
        total_flow += total_flow_rate;
        println!("CYCLE: {cycle} - Flowing at rate of {total_flow_rate}: {pre} -> {total_flow}\nClosed valves: {closed_valves:?}");

        if closed_valves.is_empty() {
            total_flow += total_flow_rate * (30 - cycle);
            println!(
                "All valves open, adding remaining flow for {} cycles",
                30 - cycle
            );
            break;
        }

        let current_valve = input.get(&current_v).unwrap();
        let most_value = find_most_value(input, &closed_valves, &current_v, 29 - cycle);

        // move to valve
        let dist = current_valve.distances.get(&most_value).unwrap();
        let actual_dist = (30 - cycle).min(*dist);
        cycle += actual_dist;
        let pre = total_flow;
        total_flow += total_flow_rate * actual_dist;
        println!("Using {dist} cycles to move from {current_v:?} to {most_value:?} - Flowing at rate of {total_flow_rate}: {pre} -> {total_flow}");

        // open valve
        let error_msg = format!(
            "Error trying to remove {:?} from closed valves {:?}",
            current_v, closed_valves
        );
        closed_valves.remove(
            closed_valves
                .iter()
                .position(|&v| v == &most_value)
                .expect(&error_msg),
        );
        let opened_valve = input.get(&most_value).unwrap();
        total_flow_rate += opened_valve.flow_rate;
        cycle += 1;
        println!(
            "Opened {most_value:?} with flow rate {}",
            opened_valve.flow_rate
        );

        current_v = most_value;
    }
    total_flow
}

fn find_most_value(valves: &Input, target_valves: &Vec<&V>, current_v: &V, rem_cycles: u32) -> V {
    let mut best_value = (target_valves[0], 0);
    for closed_v in target_valves {
        let expected = expected_flow_rate(valves, current_v, closed_v, rem_cycles);
        if expected >= best_value.1 {
            best_value = (*closed_v, expected);
        }
    }
    println!("Found best value {:?}", best_value);
    best_value.0.to_owned()
}

fn expected_flow_rate(valves: &Input, current_v: &V, target_v: &V, rem_cycles: u32) -> u32 {
    let current = valves.get(current_v).unwrap();
    let target = valves.get(target_v).unwrap();
    let dist = *current.distances.get(target_v).unwrap() as u32;
    let value = if dist > rem_cycles {
        0
    } else if dist == 0 {
        target.flow_rate
    } else {
        (rem_cycles - dist) * target.flow_rate
    };
    println!("From {current_v:?} moving {dist} to {target_v:?} (rate={}) with {rem_cycles} cycles remaining you'd get {value} value", target.flow_rate);
    value
}

#[aoc(day16, part2)]
fn part_2(_input: &Input) -> u32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    #[ignore] // still fails
    fn test_part_1() {
        let input =
            fs::read_to_string("test_input/day16.txt").expect("Error reading test input file");
        let input = generator(&input);
        assert_eq!(part_1(&input), 1651);
    }

    #[test]
    #[ignore] // not implemented
    fn test_part_2() {
        let input =
            fs::read_to_string("test_input/day16.txt").expect("Error reading test input file");
        let input = generator(&input);
        assert_eq!(part_2(&input), 0);
    }
}
