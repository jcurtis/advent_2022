use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Dir {
    dirs: Vec<String>,
    size: u32,
}

type FileSystem = HashMap<Vec<String>, Dir>;

#[aoc_generator(day7)]
fn generator(input: &str) -> FileSystem {
    let mut file_system = HashMap::new();
    file_system.insert(
        vec!["/".to_string()],
        Dir {
            dirs: vec![],
            size: 0,
        },
    );
    let mut pwd: Vec<String> = vec![];

    input.trim().lines().for_each(|line| {
        let line = line.split_whitespace().collect_vec();
        match line[0] {
            "$" => match line[1] {
                "cd" => match line[2] {
                    "/" => {
                        pwd = vec!["/".to_string()];
                    }
                    ".." => {
                        pwd.pop();
                    }
                    _ => {
                        // $ cd <dir>
                        pwd.push(line[2].to_string());
                        let dir = file_system.get_mut(&pwd);
                        let dir = match dir {
                            Some(d) => d.to_owned(),
                            None => Dir {
                                dirs: vec![],
                                size: 0,
                            },
                        };
                        file_system.insert(pwd.clone(), dir);
                    }
                },
                "ls" => {
                    // do nothing?
                }
                _ => {
                    panic!("command not recognized");
                }
            },
            "dir" => {
                let dir = file_system.get_mut(&pwd).unwrap();
                dir.dirs.push(line[1].to_string());
            }
            _ => {
                let size: u32 = line[0].parse().unwrap();

                // bump all dir sizes
                for i in 1..(pwd.len() + 1) {
                    let path = &pwd[0..i];
                    let dir = file_system.get_mut(path).unwrap();
                    dir.size += size;
                }
            }
        };
    });

    file_system
}

#[aoc(day7, part1)]
fn part_1(input: &FileSystem) -> u32 {
    input
        .iter()
        .map(|(_, dir)| {
            if dir.size <= 100000 {
                return dir.size;
            }
            0
        })
        .sum()
}

#[aoc(day7, part2)]
fn part_2(input: &FileSystem) -> u32 {
    let root = vec!["/".to_string()];
    let to_free_size = input.get(&root).unwrap().size - 40000000;
    input
        .iter()
        .filter_map(|(_, dir)| {
            if dir.size >= to_free_size {
                return Some(dir.size);
            }
            None
        })
        .sorted()
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part_1() {
        let input =
            fs::read_to_string("test_input/day07.txt").expect("Error reading test input file");
        let input = generator(&input);
        assert_eq!(part_1(&input), 95437);
    }

    #[test]
    fn test_part_2() {
        let input =
            fs::read_to_string("test_input/day07.txt").expect("Error reading test input file");
        let input = generator(&input);
        assert_eq!(part_2(&input), 24933642);
    }
}
