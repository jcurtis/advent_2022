use std::ops::ControlFlow::{self, Break, Continue};

use itertools::{
    EitherOrBoth::{Both, Left, Right},
    Itertools,
};
use serde_json::{json, Value};

#[aoc(day13, part1)]
fn part_1(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .enumerate()
        .filter_map(|(i, pairs)| {
            let pairs = pairs.split_whitespace().collect_vec();
            let left = &serde_json::from_str(pairs[0]).unwrap();
            let right = &serde_json::from_str(pairs[1]).unwrap();
            if solve_pair(left, right) {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

fn test(left: &Value, right: &Value) -> ControlFlow<bool> {
    println!("Test {:?} vs {:?}", left, right);
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => {
            let left = left.as_u64().unwrap();
            let right = right.as_u64().unwrap();

            if left < right {
                Break(true)
            } else if left > right {
                Break(false)
            } else {
                Continue(())
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
        _ => todo!(),
    }
}

fn solve_pair(input_left: &Vec<Value>, input_right: &Vec<Value>) -> bool {
    match solve_pair_rec(input_left, input_right) {
        Break(val) => val,
        Continue(_) => unreachable!(),
    }
}

fn solve_pair_rec(input_left: &Vec<Value>, input_right: &Vec<Value>) -> ControlFlow<bool> {
    input_left
        .iter()
        .zip_longest(input_right.iter())
        .try_for_each(|pair| match pair {
            Both(left, right) => test(left, right),
            // actually right
            Left(_) => Break(false),
            // actuall left
            Right(_) => Break(true),
        })
}

// #[aoc(day13, part2)]
// fn part_2(_input: &str) -> u32 {
//     todo!();
// }

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
            solve_pair(
                &serde_json::from_str("[1,1,3,1,1]").unwrap(),
                &serde_json::from_str("[1,1,5,1,1]").unwrap()
            ),
            true
        );

        assert_eq!(
            solve_pair(
                &serde_json::from_str("[[1],[2,3,4]]").unwrap(),
                &serde_json::from_str("[[1],4]").unwrap()
            ),
            true
        );

        assert_eq!(
            solve_pair(
                &serde_json::from_str("[9]").unwrap(),
                &serde_json::from_str("[[8,7,6]]").unwrap()
            ),
            false
        );

        assert_eq!(
            solve_pair(
                &serde_json::from_str("[[4,4],4,4]").unwrap(),
                &serde_json::from_str("[[4,4],4,4,4]").unwrap()
            ),
            true
        );

        assert_eq!(
            solve_pair(
                &serde_json::from_str("[7,7,7,7]").unwrap(),
                &serde_json::from_str("[7,7,7]").unwrap()
            ),
            false
        );

        assert_eq!(
            solve_pair(
                &serde_json::from_str("[]").unwrap(),
                &serde_json::from_str("[3]").unwrap()
            ),
            true
        );

        assert_eq!(
            solve_pair(
                &serde_json::from_str("[[[]]]").unwrap(),
                &serde_json::from_str("[[]]").unwrap()
            ),
            false
        );

        assert_eq!(
            solve_pair(
                &serde_json::from_str("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap(),
                &serde_json::from_str("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap()
            ),
            false
        );
    }

    // #[test]
    // fn test_part_2() {
    //     let input =
    //         fs::read_to_string("test_input/day13.txt").expect("Error reading test input file");
    //     assert_eq!(part_2(&input), 0);
    // }
}
