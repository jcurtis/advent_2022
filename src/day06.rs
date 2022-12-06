use std::collections::HashSet;

use itertools::Itertools;

#[aoc(day6, part1)]
fn part_1(input: &str) -> usize {
    let chars = input.bytes().collect_vec();
    for i in 3..chars.len() {
        if chars[i - 3] != chars[i - 2]
            && chars[i - 3] != chars[i - 1]
            && chars[i - 3] != chars[i]
            && chars[i - 2] != chars[i - 1]
            && chars[i - 2] != chars[i]
            && chars[i - 1] != chars[i]
        {
            return i + 1;
        }
    }

    panic!("Not found!");
}

#[aoc(day6, part2)]
fn part_2(input: &str) -> usize {
    let chars = input.bytes().collect_vec();
    let mut set = HashSet::new();
    for i in 14..chars.len() {
        set.clear();
        for j in 0..14 {
            if !set.insert(chars[i - j]) {
                break;
            }
        }
        if set.len() == 14 {
            return i + 1;
        }
    }
    panic!("Not found!");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const SAMPLE_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const SAMPLE_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const SAMPLE_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const SAMPLE_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_1), 7);
        assert_eq!(part_1(SAMPLE_2), 5);
        assert_eq!(part_1(SAMPLE_3), 6);
        assert_eq!(part_1(SAMPLE_4), 10);
        assert_eq!(part_1(SAMPLE_5), 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_1), 19);
        assert_eq!(part_2(SAMPLE_2), 23);
        assert_eq!(part_2(SAMPLE_3), 23);
        assert_eq!(part_2(SAMPLE_4), 29);
        assert_eq!(part_2(SAMPLE_5), 26);
    }
}
