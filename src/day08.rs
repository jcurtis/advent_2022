use itertools::Itertools;

type Grid = Vec<Vec<u8>>;
type Input = (Grid, usize);

#[aoc_generator(day08)]
fn generator(input: &str) -> Input {
    let grid = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect_vec()
        })
        .collect_vec();
    let size = grid[0].len();

    (grid, size)
}

#[aoc(day08, part1)]
fn part_1((grid, size): &Input) -> usize {
    let mut visible = (size * 2) + ((size - 2) * 2);

    for x in 1..(size - 1) {
        for y in 1..(size - 1) {
            let val = grid[x][y];

            let left = (0..x).all(|i| grid[i][y] < val);
            let right = ((x + 1)..*size).all(|i| grid[i][y] < val);

            let up = (0..y).all(|j| grid[x][j] < val);
            let down = ((y + 1)..*size).all(|j| grid[x][j] < val);

            if left || right || up || down {
                visible += 1;
            }
        }
    }

    visible
}

#[aoc(day08, part2)]
fn part_2((grid, size): &Input) -> usize {
    let mut high_score = 0;

    for x in 1..(size - 1) {
        for y in 1..(size - 1) {
            let val = grid[x][y];

            // left
            let mut left_score = 0;
            for i in (0..x).rev() {
                left_score += 1;
                if grid[i][y] >= val {
                    break;
                }
            }

            // right
            let mut right_score = 0;
            #[allow(clippy::needless_range_loop)]
            for i in (x + 1)..*size {
                right_score += 1;
                if grid[i][y] >= val {
                    break;
                }
            }

            // up
            let mut up_score = 0;
            for j in (0..y).rev() {
                up_score += 1;
                if grid[x][j] >= val {
                    break;
                }
            }

            // down
            let mut down_score = 0;
            for j in (y + 1)..*size {
                down_score += 1;
                if grid[x][j] >= val {
                    break;
                }
            }

            let tree_score = left_score * right_score * up_score * down_score;
            if tree_score > high_score {
                high_score = tree_score;
            }
        }
    }

    high_score
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part_1() {
        let input =
            fs::read_to_string("test_input/day08.txt").expect("Error reading test input file");
        let input = generator(&input);
        assert_eq!(part_1(&input), 21);
    }

    #[test]
    fn test_part_2() {
        let input =
            fs::read_to_string("test_input/day08.txt").expect("Error reading test input file");
        let input = generator(&input);
        assert_eq!(part_2(&input), 8);
    }
}
