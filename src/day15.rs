use std::collections::HashSet;

use itertools::Itertools;
use regex::Regex;

type Pos = (isize, isize);

#[derive(Debug, Clone, Copy)]
struct Sensor {
    pos: Pos,
    beacon: Pos,
    dist: isize,
}

#[aoc_generator(day15)]
fn generator(input: &str) -> Vec<Sensor> {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let pos = (caps[1].parse().unwrap(), caps[2].parse().unwrap());
            let beacon = (caps[3].parse().unwrap(), caps[4].parse().unwrap());
            let dist = distance(pos, beacon);
            Sensor { pos, beacon, dist }
        })
        .collect_vec()
}

#[aoc(day15, part1)]
fn part_1(input: &[Sensor]) -> usize {
    solve_part_1(input, 2000000)
}

fn distance((p1, p2): (isize, isize), (q1, q2): (isize, isize)) -> isize {
    (p1 - q1).abs() + (p2 - q2).abs()
}

fn solve_part_1(input: &[Sensor], row: isize) -> usize {
    let known_beacons: HashSet<Pos> = input.iter().map(|sensor| sensor.beacon).collect();
    input
        .iter()
        .flat_map(|sensor| {
            let mut points = vec![];
            let (sx, sy) = sensor.pos;
            for x in (sx - sensor.dist)..(sx + sensor.dist) {
                if distance((x, row), (sx, sy)) <= sensor.dist && !known_beacons.contains(&(x, row))
                {
                    points.push((x, row));
                }
            }
            points
        })
        .unique()
        .count()
}

#[aoc(day15, part2)]
fn part_2(input: &[Sensor]) -> isize {
    solve_part_2(input, 4000000)
}

fn find_ranges(x: isize, sensors: &[Sensor], max: isize) -> Vec<(isize, isize)> {
    sensors
        .iter()
        .filter_map(|sensor| {
            let d = sensor.dist;
            let f = (d - (sensor.pos.0 - x).abs()).abs();
            if d < f {
                None
            } else {
                Some(((sensor.pos.1 - f).max(0), (sensor.pos.1 + f).min(max)))
            }
        })
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .collect_vec()
}

/* Merge ranges until a gap is found */
fn find_range_gap(ranges: &[(isize, isize)], max: isize) -> Option<isize> {
    let mut stack = ranges[0];

    #[allow(clippy::needless_range_loop)]
    for i in 1..ranges.len() {
        let (from, to) = ranges[i];
        if stack.0 == 0 && stack.1 == max {
            return None;
        }

        if (from - 1) > stack.1 {
            println!(
                "Found gap trying to merge stack={:?} with {:?}",
                stack,
                (from, to)
            );
            return Some(from - 1);
        } else {
            stack = (stack.0, stack.1.max(to));
        }
    }
    None
}

fn solve_part_2(input: &[Sensor], max: isize) -> isize {
    for x in 0..=max {
        let ranges = find_ranges(x, input, max);
        if let Some(y) = find_range_gap(&ranges, max) {
            return (x * 4000000) + y;
        }
    }
    unreachable!("answer not found after {max} iterations!");
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part_1() {
        let input =
            fs::read_to_string("test_input/day15.txt").expect("Error reading test input file");
        let input = generator(&input);
        assert_eq!(solve_part_1(&input, 10), 26);
    }

    #[test]
    fn test_part_2() {
        let input =
            fs::read_to_string("test_input/day15.txt").expect("Error reading test input file");
        let input = generator(&input);
        assert_eq!(solve_part_2(&input, 20), 56000011);
    }
}
