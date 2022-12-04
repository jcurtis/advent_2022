use itertools::Itertools;

#[aoc(day4, part1)]
fn part_1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|line| {
            let line = line
                .split(&[',', '-'])
                .map(|num| num.parse::<u32>().unwrap())
                .collect_vec();
            let first_overlaps =
                (line[0]..=line[1]).all(|section| (line[2]..=line[3]).contains(&section));
            let second_overlaps =
                (line[2]..=line[3]).all(|section| (line[0]..=line[1]).contains(&section));

            first_overlaps || second_overlaps
        })
        .count()
}

#[aoc(day4, part2)]
fn part_2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|line| {
            let line = line
                .split(&[',', '-'])
                .map(|num| num.parse::<u32>().unwrap())
                .collect_vec();
            let first_overlaps =
                (line[0]..=line[1]).any(|section| (line[2]..=line[3]).contains(&section));
            let second_overlaps =
                (line[2]..=line[3]).any(|section| (line[0]..=line[1]).contains(&section));

            first_overlaps || second_overlaps
        })
        .count()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part_1() {
        let input =
            fs::read_to_string("test_input/day04.txt").expect("Error reading test input file");
        assert_eq!(part_1(&input), 2);
    }

    #[test]
    fn test_part_2() {
        let input =
            fs::read_to_string("test_input/day04.txt").expect("Error reading test input file");
        assert_eq!(part_2(&input), 4);
    }
}
