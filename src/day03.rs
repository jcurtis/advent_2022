use std::str::{self};

use itertools::Itertools;

#[aoc(day3, part1)]
fn part_1(input: &str) -> u32 {
    input.trim().lines().map(solve_line).sum()
}

fn solve_line(line: &str) -> u32 {
    let half = line.len() / 2;
    let first = line[0..half].as_bytes();
    let second = line[half..].as_bytes();

    for c in first {
        let found = second.iter().position(|sc| sc == c);
        if found.is_some() {
            return priority_from_byte(*c) as u32;
        }
    }
    0
}

fn priority_from_byte(input: u8) -> u8 {
    if input <= 90 {
        input - 38
    } else {
        input - 96
    }
}

#[aoc(day3, part2)]
fn part_2(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .collect_vec()
        .chunks(3)
        .map(|chunk| {
            let first = chunk[0].as_bytes();
            let second = chunk[1].as_bytes();
            let third = chunk[2].as_bytes();

            for c in first {
                let found_second = second.iter().position(|sc| sc == c);
                if found_second.is_some() {
                    let found_third = third.iter().position(|sc| sc == c);
                    if found_third.is_some() {
                        return priority_from_byte(*c) as u32;
                    }
                }
            }

            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("test_input/day03.txt").expect("error reading file");
        assert_eq!(part_1(&input), 157);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("test_input/day03.txt").expect("error reading file");
        assert_eq!(part_2(&input), 70);
    }
}
