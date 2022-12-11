use std::vec;

use itertools::Itertools;

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    inspected: u32,
    operation: fn(u32) -> u32,
    test: fn(u32) -> usize,
}

fn get_input_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![89, 84, 88, 78, 70],
            inspected: 0,
            operation: |item| item * 5,
            test: |item| if item % 7 == 0 { 6 } else { 7 },
        },
        Monkey {
            items: vec![76, 62, 61, 54, 69, 60, 85],
            inspected: 0,
            operation: |item| item + 1,
            test: |item| if item % 17 == 0 { 0 } else { 6 },
        },
        Monkey {
            items: vec![83, 89, 53],
            inspected: 0,
            operation: |item| item + 8,
            test: |item| if item % 11 == 0 { 5 } else { 3 },
        },
        Monkey {
            items: vec![95, 94, 85, 57],
            inspected: 0,
            operation: |item| item + 4,
            test: |item| if item % 13 == 0 { 0 } else { 1 },
        },
        Monkey {
            items: vec![82, 98],
            inspected: 0,
            operation: |item| item + 7,
            test: |item| if item % 19 == 0 { 5 } else { 2 },
        },
        Monkey {
            items: vec![69],
            inspected: 0,
            operation: |item| item + 2,
            test: |item| if item % 2 == 0 { 1 } else { 3 },
        },
        Monkey {
            items: vec![82, 70, 58, 87, 59, 99, 92, 65],
            inspected: 0,
            operation: |item| item * 11,
            test: |item| if item % 5 == 0 { 7 } else { 4 },
        },
        Monkey {
            items: vec![91, 53, 96, 98, 68, 82],
            inspected: 0,
            operation: |item| item * item,
            test: |item| if item % 3 == 0 { 4 } else { 2 },
        },
    ]
}

#[aoc(day11, part1)]
fn part_1(_input: &str) -> u32 {
    solve_part_1(get_input_monkeys())
}

fn solve_part_1(mut monkeys: Vec<Monkey>) -> u32 {
    for _ in 0..20 {
        process_round(&mut monkeys);
    }
    monkeys
        .iter()
        .map(|monkey| monkey.inspected)
        .sorted()
        .rev()
        .take(2)
        .reduce(|acc, inspected| acc * inspected)
        .unwrap()
}

fn process_round<'a>(monkeys: &'a mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        let items = monkeys[i].items.drain(0..).collect_vec();
        for item in items {
            let item = (monkeys[i].operation)(item) / 3;
            let throw_to = (monkeys[i].test)(item);
            monkeys[throw_to].items.push(item);
            monkeys[i].inspected += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monkey() {
        let monkey = Monkey {
            items: vec![79, 98],
            inspected: 0,
            operation: |item| item * 19,
            test: |item| if item % 23 == 0 { 2 } else { 3 },
        };

        assert_eq!((monkey.operation)(79), 1501);
        assert_eq!((monkey.test)(79), 3);
        assert_eq!((monkey.test)(23), 2);
        assert_eq!((monkey.test)(46), 2);
    }

    fn get_test_monkeys() -> Vec<Monkey> {
        vec![
            Monkey {
                items: vec![79, 98],
                inspected: 0,
                operation: |item| item * 19,
                test: |item| if item % 23 == 0 { 2 } else { 3 },
            },
            Monkey {
                items: vec![54, 65, 75, 74],
                inspected: 0,
                operation: |item| item + 6,
                test: |item| if item % 19 == 0 { 2 } else { 0 },
            },
            Monkey {
                items: vec![79, 60, 97],
                inspected: 0,
                operation: |item| item * item,
                test: |item| if item % 13 == 0 { 1 } else { 3 },
            },
            Monkey {
                items: vec![74],
                inspected: 0,
                operation: |item| item + 3,
                test: |item| if item % 17 == 0 { 0 } else { 1 },
            },
        ]
    }

    #[test]
    fn test_process_round() {
        let mut monkeys = get_test_monkeys();
        process_round(&mut monkeys);

        assert_eq!(monkeys[0].items, vec![20, 23, 27, 26]);
        assert_eq!(monkeys[0].inspected, 2);

        assert_eq!(monkeys[1].items, vec![2080, 25, 167, 207, 401, 1046]);
        assert_eq!(monkeys[1].inspected, 4);

        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[2].inspected, 3);

        assert_eq!(monkeys[3].items, vec![]);
        assert_eq!(monkeys[3].inspected, 5);
    }

    #[test]
    fn test_part_1() {
        let input = get_test_monkeys();
        assert_eq!(solve_part_1(input), 10605);
    }
}
