use core::cmp::Ordering;
use itertools::Itertools;
use pathfinding::prelude::Grid;
use regex::Regex;
use std::ops::RangeInclusive;

fn get_range(a: usize, b: usize) -> RangeInclusive<usize> {
    match a.cmp(&b) {
        Ordering::Greater => b..=a,
        Ordering::Less => a..=b,
        Ordering::Equal => a..=b,
    }
}

#[aoc_generator(day14)]
fn generator(input: &str) -> Grid {
    let re = Regex::new(r"(\d+),(\d+)").unwrap();
    input
        .lines()
        .flat_map(|line| {
            re.captures_iter(line)
                .map(|cap| {
                    (
                        cap[1].parse::<usize>().unwrap(),
                        cap[2].parse::<usize>().unwrap(),
                    )
                })
                .tuple_windows()
                .flat_map(|(a, b)| {
                    if a.0 == b.0 {
                        let range = get_range(a.1, b.1);
                        return range.map(|y| (a.0, y)).collect_vec();
                    }
                    get_range(a.0, b.0).map(|x| (x, a.1)).collect_vec()
                })
                .collect_vec()
        })
        .collect()
}

fn check_below(input: &Grid, pos: (usize, usize)) -> Option<(usize, usize)> {
    let down = (pos.0, pos.1 + 1);
    if !input.has_vertex(down) {
        return Some(down);
    }

    let left = (pos.0 - 1, pos.1 + 1);
    if !input.has_vertex(left) {
        return Some(left);
    }

    let right = (pos.0 + 1, pos.1 + 1);
    if !input.has_vertex(right) {
        return Some(right);
    }

    None
}

#[derive(Debug, PartialEq)]
enum State {
    Start,
    Stopped,
    Dropped,
}

const START_POS: (usize, usize) = (500, 0);

fn drop_sand(input: &mut Grid, max_height: usize) -> State {
    let mut sand = START_POS;
    while let Some(next_sand) = check_below(input, sand) {
        if next_sand.1 > max_height {
            return State::Dropped;
        }
        sand = next_sand;
    }
    input.add_vertex(sand);
    State::Stopped
}

#[aoc(day14, part1)]
fn part_1(input: &Grid) -> usize {
    let mut input = input.to_owned();
    let max_height = input.height;

    let mut res = State::Start;
    let mut count = 0;
    while res != State::Dropped {
        res = drop_sand(&mut input, max_height);
        count += 1;
    }
    count - 1
}

#[aoc(day14, part2)]
fn part_2(input: &Grid) -> usize {
    let mut input = input.to_owned();
    let max_height = input.height + 1;
    input.resize(1000, max_height + 1);
    for x in 0..1000 {
        if !input.add_vertex((x, max_height)) {
            panic!("Failed to add floor! ({x}, {max_height})");
        }
    }

    for i in 1..100000 {
        let mut sand = START_POS;
        while let Some(next_sand) = check_below(&input, sand) {
            sand = next_sand;
        }
        if sand == START_POS {
            // println!("Reached top!");
            return i;
        }
        input.add_vertex(sand);
    }

    panic!("Not enough iterations!");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part_2() {
        let input = generator(SAMPLE);
        assert_eq!(part_2(&input), 93);
    }

    #[test]
    fn test_part_1() {
        let input = generator(SAMPLE);
        assert_eq!(part_1(&input), 24);
    }
}
