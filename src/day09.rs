use std::collections::HashSet;

fn step(direction: char, mut head: (i32, i32), mut tail: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    match direction {
        'R' => {
            head.0 += 1;
        }
        'L' => {
            head.0 -= 1;
        }
        'U' => {
            head.1 += 1;
        }
        'D' => {
            head.1 -= 1;
        }
        _ => {}
    };

    let hor = head.0.abs_diff(tail.0) > 1;
    let ver = head.1.abs_diff(tail.1) > 1;
    if hor && head.1 != tail.1 || ver && head.0 != tail.0 {
        if head.0 > tail.0 && head.1 > tail.1 {
            tail.0 += 1;
            tail.1 += 1;
        } else if head.0 > tail.0 && head.1 < tail.1 {
            tail.0 += 1;
            tail.1 -= 1;
        } else if head.0 < tail.0 && head.1 > tail.1 {
            tail.0 -= 1;
            tail.1 += 1;
        } else {
            tail.0 -= 1;
            tail.1 -= 1;
        }
        // println!("tail follows diagonally {:?} {:?}", head, tail);
    } else if hor {
        if head.0 > tail.0 {
            tail.0 += 1;
        } else {
            tail.0 -= 1;
        }
        // println!("tail follows horizontally {:?} {:?}", head, tail);
    } else if ver {
        if head.1 > tail.1 {
            tail.1 += 1;
        } else {
            tail.1 -= 1;
        }
        // println!("tail follows vertically {:?} {:?}", head, tail);
    };

    (head, tail)
}

#[aoc(day09, part1)]
fn part_1(input: &str) -> usize {
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    input.trim().lines().for_each(|motion| {
        let mut motion = motion.split_whitespace();
        let direction = motion.next().unwrap().chars().next().unwrap();
        let steps = motion.next().unwrap().parse::<u32>().unwrap();

        (0..steps).for_each(|_| {
            (head, tail) = step(direction, head, tail);
            visited.insert(tail);
        });
    });

    visited.len()
}

#[aoc(day09, part2)]
fn part_2(_input: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part_1() {
        let input =
            fs::read_to_string("test_input/day09.txt").expect("Error reading test input file");
        assert_eq!(part_1(&input), 13);
    }

    #[test]
    fn test_part_2() {
        let input =
            fs::read_to_string("test_input/day09.txt").expect("Error reading test input file");
        assert_eq!(part_2(&input), 36);
    }
}
