use itertools::Itertools;

#[aoc(day1, part1)]
fn part_1(input: &str) -> u32 {
    input
        .trim()
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|item| item.trim().parse::<u32>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
fn part_2(input: &str) -> u32 {
    input
        .trim()
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|item| item.trim().parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("test_input/day01.txt").expect("Error reading file");

        assert_eq!(part_1(&input), 24000);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("test_input/day01.txt").expect("Error reading file");

        assert_eq!(part_2(&input), 45000);
    }
}
