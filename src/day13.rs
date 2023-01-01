use std::{
    cmp::Ordering::{Equal, Greater, Less},
    ops::ControlFlow::{self, Break, Continue},
};

use itertools::{
    EitherOrBoth::{Both, Left, Right},
    Itertools,
};
use serde_json::{json, Value};

use crate::double_line_ending;

#[aoc(day13, part1)]
fn part_1(input: &str) -> usize {
    let delimiter = double_line_ending();
    input
        .trim()
        .split(&delimiter)
        .enumerate()
        .filter_map(|(i, pairs)| {
            let pairs = pairs.split_whitespace().collect_vec();
            let left: &Vec<Value> = &from_str(pairs[0]);
            let right: &Vec<Value> = &from_str(pairs[1]);
            if solve_pair(left, right) {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

fn test(left: &Value, right: &Value) -> ControlFlow<bool> {
    // println!("Test {:?} vs {:?}", left, right);
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => {
            let left = left.as_u64().unwrap();
            let right = right.as_u64().unwrap();

            match left.cmp(&right) {
                Less => Break(true),
                Greater => Break(false),
                Equal => Continue(()),
            }
        }
        (Value::Array(left), Value::Array(right)) => solve_pair_rec(left, right),
        (Value::Number(left), Value::Array(right)) => {
            let left = vec![json!(left.as_u64().unwrap())];
            solve_pair_rec(&left, right)
        }
        (Value::Array(left), Value::Number(right)) => {
            let right = vec![json!(right.as_u64().unwrap())];
            solve_pair_rec(left, &right)
        }
        _ => unreachable!(),
    }
}

fn solve_pair(input_left: &[Value], input_right: &[Value]) -> bool {
    match solve_pair_rec(input_left, input_right) {
        Break(val) => val,
        Continue(_) => unreachable!(),
    }
}

fn solve_pair_rec(input_left: &[Value], input_right: &[Value]) -> ControlFlow<bool> {
    input_left
        .iter()
        .zip_longest(input_right.iter())
        .try_for_each(|pair| match pair {
            Both(left, right) => test(left, right),
            // actually right
            Left(_) => Break(false),
            // actually left
            Right(_) => Break(true),
        })
}

fn from_str(input: &str) -> Vec<Value> {
    serde_json::from_str(input).unwrap()
}

const DIV_2: &str = "[[2]]";
const DIV_6: &str = "[[6]]";

#[aoc(day13, part2)]
fn part_2(input: &str) -> u32 {
    let res = [DIV_2, DIV_6, input]
        .join("\n")
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            let value: Vec<Value> = serde_json::from_str(line).unwrap();
            Some(value)
        })
        .sorted_by(|left, right| match solve_pair(left, right) {
            true => Less,
            false => Greater,
        })
        .collect_vec();

    let div_2 = from_str(DIV_2);
    let div_6 = from_str(DIV_6);

    let div_2_pos = res.iter().position(|item| item == &div_2).unwrap() as u32;
    let div_6_pos = res.iter().position(|item| item == &div_6).unwrap() as u32;

    (div_2_pos + 1) * (div_6_pos + 1)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part_1() {
        let input =
            fs::read_to_string("test_input/day13.txt").expect("Error reading test input file");
        assert_eq!(part_1(&input), 13);
    }

    #[test]
    fn test_solve_pair() {
        assert_eq!(
            solve_pair(&from_str("[1,1,3,1,1]"), &from_str("[1,1,5,1,1]")),
            true
        );

        assert_eq!(
            solve_pair(&from_str("[[1],[2,3,4]]"), &from_str("[[1],4]")),
            true
        );

        assert_eq!(solve_pair(&from_str("[9]"), &from_str("[[8,7,6]]")), false);

        assert_eq!(
            solve_pair(&from_str("[[4,4],4,4]"), &from_str("[[4,4],4,4,4]")),
            true
        );

        assert_eq!(
            solve_pair(&from_str("[7,7,7,7]"), &from_str("[7,7,7]")),
            false
        );

        assert_eq!(solve_pair(&from_str("[]"), &from_str("[3]")), true);

        assert_eq!(solve_pair(&from_str("[[[]]]"), &from_str("[[]]")), false);

        assert_eq!(
            solve_pair(
                &from_str("[1,[2,[3,[4,[5,6,7]]]],8,9]"),
                &from_str("[1,[2,[3,[4,[5,6,0]]]],8,9]")
            ),
            false
        );
    }

    #[test]
    fn test_part_2() {
        let input =
            fs::read_to_string("test_input/day13.txt").expect("Error reading test input file");
        assert_eq!(part_2(&input), 140);
    }
}
