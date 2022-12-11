use std::vec;

use itertools::Itertools;

#[derive(Debug)]
struct Monkey {
    items: Vec<u128>,
    inspected: u128,
    operation: fn(u128) -> u128,
    test: u128,
    t: usize,
    f: usize,
}

impl Monkey {
    fn throw_to(&self, item: u128) -> usize {
        if item % self.test == 0 {
            self.t
        } else {
            self.f
        }
    }
}

fn get_input_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![89, 84, 88, 78, 70],
            inspected: 0,
            operation: |item| item * 5,
            test: 7,
            t: 6,
            f: 7,
        },
        Monkey {
            items: vec![76, 62, 61, 54, 69, 60, 85],
            inspected: 0,
            operation: |item| item + 1,
            test: 17,
            t: 0,
            f: 6,
        },
        Monkey {
            items: vec![83, 89, 53],
            inspected: 0,
            operation: |item| item + 8,
            test: 11,
            t: 5,
            f: 3,
        },
        Monkey {
            items: vec![95, 94, 85, 57],
            inspected: 0,
            operation: |item| item + 4,
            test: 13,
            t: 0,
            f: 1,
        },
        Monkey {
            items: vec![82, 98],
            inspected: 0,
            operation: |item| item + 7,
            test: 19,
            t: 5,
            f: 2,
        },
        Monkey {
            items: vec![69],
            inspected: 0,
            operation: |item| item + 2,
            test: 2,
            t: 1,
            f: 3,
        },
        Monkey {
            items: vec![82, 70, 58, 87, 59, 99, 92, 65],
            inspected: 0,
            operation: |item| item * 11,
            test: 5,
            t: 7,
            f: 4,
        },
        Monkey {
            items: vec![91, 53, 96, 98, 68, 82],
            inspected: 0,
            operation: |item| item * item,
            test: 3,
            t: 4,
            f: 2,
        },
    ]
}

#[aoc(day11, part1)]
fn part_1(_input: &str) -> u128 {
    solve_part_1(get_input_monkeys())
}

fn solve_part_1(monkeys: Vec<Monkey>) -> u128 {
    solve(monkeys, 20, Some(3))
}

#[aoc(day11, part2)]
fn part_2(_input: &str) -> u128 {
    solve_part_2(get_input_monkeys(), 10_000)
}

fn solve_part_2(monkeys: Vec<Monkey>, rounds: usize) -> u128 {
    solve(monkeys, rounds, None)
}

fn find_modulo(monkeys: &[Monkey]) -> u128 {
    monkeys
        .iter()
        .map(|monkey| monkey.test)
        .reduce(|acc, test| acc * test)
        .unwrap()
}

fn solve(mut monkeys: Vec<Monkey>, rounds: usize, worry_div: Option<u128>) -> u128 {
    let modulo = find_modulo(&monkeys);
    for _ in 0..rounds {
        process_round(&mut monkeys, worry_div, modulo);
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

fn process_round(monkeys: &mut Vec<Monkey>, worry_div: Option<u128>, modulo: u128) {
    for i in 0..monkeys.len() {
        let items = monkeys[i].items.drain(0..).collect_vec();
        for item in items {
            let item = item % modulo;
            let item = (monkeys[i].operation)(item);
            let item = if let Some(divider) = worry_div {
                item / divider
            } else {
                item
            };
            let throw_to = monkeys[i].throw_to(item);
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
            test: 23,
            t: 2,
            f: 3,
        };

        assert_eq!((monkey.operation)(79), 1501);
        assert_eq!(monkey.throw_to(79), 3);
        assert_eq!(monkey.throw_to(23), 2);
        assert_eq!(monkey.throw_to(46), 2);
    }

    fn get_test_monkeys() -> Vec<Monkey> {
        vec![
            Monkey {
                items: vec![79, 98],
                inspected: 0,
                operation: |item| item * 19,
                test: 23,
                t: 2,
                f: 3,
            },
            Monkey {
                items: vec![54, 65, 75, 74],
                inspected: 0,
                operation: |item| item + 6,
                test: 19,
                t: 2,
                f: 0,
            },
            Monkey {
                items: vec![79, 60, 97],
                inspected: 0,
                operation: |item| item * item,
                test: 13,
                t: 1,
                f: 3,
            },
            Monkey {
                items: vec![74],
                inspected: 0,
                operation: |item| item + 3,
                test: 17,
                t: 0,
                f: 1,
            },
        ]
    }

    #[test]
    fn test_process_round() {
        let mut monkeys = get_test_monkeys();
        let modulo = find_modulo(&monkeys);
        process_round(&mut monkeys, Some(3), modulo);

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

    #[test]
    fn test_part_2_1() {
        let input = get_test_monkeys();
        assert_eq!(solve_part_2(input, 1), 24);
    }

    #[test]
    fn test_part_2_20() {
        let input = get_test_monkeys();
        assert_eq!(solve_part_2(input, 20), 10197);
    }

    #[test]
    fn test_part_2_1000() {
        let input = get_test_monkeys();
        assert_eq!(solve_part_2(input, 1000), 27019168);
    }

    #[test]
    fn test_part_2_10000() {
        let input = get_test_monkeys();
        assert_eq!(solve_part_2(input, 10_000), 2713310158);
    }
}
