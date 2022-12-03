#[aoc(day2, part1)]
fn part_1(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut round = line.chars();
            let elf = parse_elf(round.next().unwrap());
            let me = parse_me(round.nth(1).unwrap());
            (get_res(elf, me) as u32) + (me as u32)
        })
        .sum()
}

// 1 for Rock, 2 for Paper, and 3 for Scissors
#[derive(Debug, PartialEq, Copy, Clone)]
enum Rps {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

// 0 if you lost, 3 if the round was a draw, and 6 if you won
#[derive(Debug)]
enum Res {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

// A for Rock, B for Paper, and C for Scissors
fn parse_elf(input: char) -> Rps {
    match input {
        'A' => Rps::Rock,
        'B' => Rps::Paper,
        'C' => Rps::Scissors,
        _ => panic!("invalid instruction"),
    }
}

// X for Rock, Y for Paper, and Z for Scissors
fn parse_me(input: char) -> Rps {
    match input {
        'X' => Rps::Rock,
        'Y' => Rps::Paper,
        'Z' => Rps::Scissors,
        _ => panic!("invalid instruction"),
    }
}

fn get_res(elf: Rps, me: Rps) -> Res {
    match elf {
        Rps::Rock => match me {
            Rps::Rock => Res::Draw,
            Rps::Paper => Res::Win,
            Rps::Scissors => Res::Loss,
        },
        Rps::Paper => match me {
            Rps::Rock => Res::Loss,
            Rps::Paper => Res::Draw,
            Rps::Scissors => Res::Win,
        },
        Rps::Scissors => match me {
            Rps::Rock => Res::Win,
            Rps::Paper => Res::Loss,
            Rps::Scissors => Res::Draw,
        },
    }
}

#[aoc(day2, part2)]
fn part_2(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut round = line.chars();
            let elf = parse_elf(round.next().unwrap());
            let res = parse_res(round.nth(1).unwrap());
            let me = get_me(elf, res);
            (get_res(elf, me) as u32) + (me as u32)
        })
        .sum()
}

// X means you need to lose,
// Y means you need to end the round in a draw, and
// Z means you need to win
fn parse_res(input: char) -> Res {
    match input {
        'X' => Res::Loss,
        'Y' => Res::Draw,
        'Z' => Res::Win,
        _ => panic!("invalid instruction"),
    }
}

fn get_me(elf: Rps, res: Res) -> Rps {
    match elf {
        Rps::Rock => match res {
            Res::Win => Rps::Paper,
            Res::Loss => Rps::Scissors,
            Res::Draw => Rps::Rock,
        },
        Rps::Paper => match res {
            Res::Win => Rps::Scissors,
            Res::Loss => Rps::Rock,
            Res::Draw => Rps::Paper,
        },
        Rps::Scissors => match res {
            Res::Win => Rps::Rock,
            Res::Loss => Rps::Paper,
            Res::Draw => Rps::Scissors,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("test_input/day02.txt").expect("error reading file");
        assert_eq!(part_2(&input), 12);
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("test_input/day02.txt").expect("error reading file");
        assert_eq!(part_1(&input), 15);
    }
}
