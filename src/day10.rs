use itertools::Itertools;

#[aoc_generator(day10)]
fn generator(input: &str) -> Vec<i32> {
    let mut x = 1;
    input
        .trim()
        .lines()
        .flat_map(|ins| {
            let ins = ins.split_whitespace().collect_vec();
            match ins[0] {
                "noop" => {
                    vec![x]
                }
                "addx" => {
                    let pre_x = x;
                    let val: i32 = ins[1].parse().unwrap();
                    x += val;
                    vec![pre_x, pre_x]
                }
                _ => unreachable!(),
            }
        })
        .collect()
}

#[aoc(day10, part1)]
fn part_1(input: &[i32]) -> i32 {
    input
        .iter()
        .enumerate()
        .filter_map(|(i, x)| {
            // println!("At {}: {:?}", i + 1, ins);
            match i {
                19 | 59 | 99 | 139 | 179 | 219 => {
                    // println!("At {}: {:?}", i + 1, ins);
                    Some((1 + i as i32) * x)
                }
                _ => None,
            }
        })
        .sum()
}

fn map_sprite(i: usize, x: i32) -> char {
    let cycle = i as i32;
    if x == cycle || x == cycle - 1 || x == cycle + 1 {
        '#'
    } else {
        '.'
    }
}

#[aoc(day10, part2)]
fn part_2(input: &[i32]) -> String {
    (0..6)
        .map(|i| {
            input
                .iter()
                .skip(i * 40)
                .take(40)
                .enumerate()
                .map(|(i, x)| map_sprite(i, *x))
                .collect::<String>()
        })
        .join("\n")
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_generator() {
        let input = "noop
        addx 3
        addx -5";

        assert_eq!(generator(&input), vec![1, 1, 1, 4, 4]);
    }

    #[test]
    fn test_part_1() {
        let input =
            fs::read_to_string("test_input/day10.txt").expect("Error reading test input file");
        let input = generator(&input);
        assert_eq!(part_1(&input), 13140);
    }

    #[test]
    fn test_part_2() {
        let input =
            fs::read_to_string("test_input/day10.txt").expect("Error reading test input file");
        let input = generator(&input);
        assert_eq!(
            part_2(&input),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}

// ####.####..##..####.###..#..#.###..####.
// #....#....#..#.#....#..#.#..#.#..#.#....
// ###..###..#....###..#..#.#..#.#..#.###..
// #....#....#.##.#....###..#..#.###..#....
// #....#....#..#.#....#.#..#..#.#.#..#....
// ####.#.....###.####.#..#..##..#..#.####.
