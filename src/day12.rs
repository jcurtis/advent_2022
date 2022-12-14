extern crate pathfinding;

use std::collections::HashMap;

use itertools::Itertools;
use pathfinding::prelude::bfs;

type Pos = (i32, i32);

#[derive(Debug)]
struct Grid {
    grid: HashMap<Pos, u8>,
    start: Pos,
    end: Pos,
}

#[aoc_generator(day12)]
fn generator(input: &str) -> Grid {
    let mut grid = HashMap::new();
    let mut start = None;
    let mut end = None;
    input.trim().lines().enumerate().for_each(|(y, line)| {
        for (x, item) in line.bytes().enumerate() {
            let val = match item {
                b'S' => {
                    start = Some((x as i32, y as i32));
                    b'a'
                }
                b'E' => {
                    end = Some((x as i32, y as i32));
                    b'z'
                }
                b'a'..=b'z' => item,
                _ => unreachable!(),
            };
            grid.insert((x as i32, y as i32), val);
        }
    });

    Grid {
        grid,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

// wrong: 418
#[aoc(day12, part1)]
fn part_1(input: &Grid) -> usize {
    let result = bfs(
        &input.start,
        |&pos| {
            DIRS.iter()
                .flat_map(|dir| check_direction(input, pos, *dir))
                .collect_vec()
        },
        |&p| p == input.end,
    )
    .unwrap();
    result.len() - 1
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

const DIRS: [Dir; 4] = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];

impl Dir {
    fn value(&self) -> (i32, i32) {
        match *self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        }
    }
}

fn check_direction(input: &Grid, pos: Pos, dir: Dir) -> Option<Pos> {
    let current_height = *input
        .grid
        .get(&pos)
        .expect("Tried to access point not on the grid!");
    let dest = (pos.0 + dir.value().0, pos.1 + dir.value().1);
    if let Some(dest_height) = input.grid.get(&dest) {
        if current_height >= dest_height - 1 {
            return Some(dest);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test_check_direction() {
        let input = generator(SAMPLE);

        assert_eq!(check_direction(&input, (0, 0), Dir::Up), None);
        assert_eq!(check_direction(&input, (0, 0), Dir::Down), Some((0, 1)));
        assert_eq!(check_direction(&input, (0, 0), Dir::Left), None);
        assert_eq!(check_direction(&input, (0, 0), Dir::Right), Some((1, 0)));
    }

    #[test]
    fn test_part_1() {
        let input = generator(SAMPLE);
        assert_eq!(part_1(&input), 31);
    }
}
