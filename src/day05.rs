use itertools::Itertools;

#[derive(Debug)]
struct Command {
    count: usize,
    from: usize,
    to: usize,
}

type Stacks = Vec<Vec<char>>;

struct Input {
    stacks: Stacks,
    commands: Vec<Command>,
}

fn generator(input: &str, num_stacks: usize, stack_height: usize) -> Input {
    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..num_stacks {
        stacks.push(vec![]);
    }

    input.lines().take(stack_height).for_each(|stack_line| {
        let stack_line = stack_line.chars().collect_vec();
        for i in 0..num_stacks {
            let c = stack_line[1 + (i * 4)];
            if c != ' ' {
                stacks[i].insert(0, c);
            }
        }
    });

    let commands = input
        .lines()
        .skip(stack_height + 2)
        .map(|command_line| {
            let command_line = command_line.split_whitespace().collect_vec();
            Command {
                count: command_line[1].parse().unwrap(),
                from: command_line[3].parse().unwrap(),
                to: command_line[5].parse().unwrap(),
            }
        })
        .collect_vec();

    Input { stacks, commands }
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Input {
    generator(input, 9, 8)
}

#[aoc(day5, part1)]
fn part_1(input: &Input) -> String {
    let mut stacks = input.stacks.clone();
    for command in input.commands.iter() {
        for _ in 0..command.count {
            let c = stacks[command.from - 1].pop().unwrap();
            stacks[command.to - 1].push(c);
        }
    }

    read_answer(&stacks)
}

#[aoc(day5, part2)]
fn part_2(input: &Input) -> String {
    let mut stacks = input.stacks.clone();
    for command in input.commands.iter() {
        let stack_size = stacks[command.from - 1].len();
        let mut drained = stacks[command.from - 1]
            .drain((stack_size - command.count)..)
            .collect_vec();
        stacks[command.to - 1].append(&mut drained);
    }

    read_answer(&stacks)
}

fn read_answer(stacks: &Stacks) -> String {
    let mut answer = String::new();
    for stack in stacks {
        answer.push(*stack.last().unwrap());
    }
    answer
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part_1() {
        let input =
            fs::read_to_string("test_input/day05.txt").expect("Error reading test input file");
        let input = generator(&input, 3, 3);
        assert_eq!(part_1(&input), "CMZ");
    }

    #[test]
    fn test_part_2() {
        let input =
            fs::read_to_string("test_input/day05.txt").expect("Error reading test input file");
        let input = generator(&input, 3, 3);
        assert_eq!(part_2(&input), "MCD");
    }
}
