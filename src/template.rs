#[aoc(dayX, part1)]
fn part_1(_input: &str) -> u32 {
    todo!();
}

#[aoc(dayX, part2)]
fn part_2(_input: &str) -> u32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part_1() {
        let input =
            fs::read_to_string("test_input/dayX.txt").expect("Error reading test input file");
        assert_eq!(part_1(&input), 0);
    }

    #[test]
    fn test_part_2() {
        let input =
            fs::read_to_string("test_input/dayX.txt").expect("Error reading test input file");
        assert_eq!(part_2(&input), 0);
    }
}
