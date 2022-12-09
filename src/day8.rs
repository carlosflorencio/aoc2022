type Input = Vec<Vec<usize>>;
pub struct Forest(Input);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Tree {
    is_visible: bool,
    scenic_score: usize,
}

impl Forest {
    fn count_outside(&self) -> usize {
        let y = self.0.len();
        let x = self.0[0].len();

        y * 2 + (x - 2) * 2
    }

    fn parse_inside(&self) -> Vec<Tree> {
        let mut result: Vec<Tree> = vec![];

        for y in 1..self.0.len() - 1 {
            for x in 1..self.0[y].len() - 1 {
                let curr_height = self.0[y][x];
                let up = scan_direction(&self.0, y, x, curr_height, &Direction::Up);
                let down = scan_direction(&self.0, y, x, curr_height, &Direction::Down);
                let left = scan_direction(&self.0, y, x, curr_height, &Direction::Left);
                let right = scan_direction(&self.0, y, x, curr_height, &Direction::Right);

                result.push(Tree {
                    is_visible: up.0 || down.0 || left.0 || right.0,
                    scenic_score: up.1 * down.1 * left.1 * right.1,
                });
            }
        }

        result
    }
}

fn scan_direction(board: &Input, y: usize, x: usize, h: usize, dir: &Direction) -> (bool, usize) {
    let mut is_visible: bool = false;
    let mut trees: usize = 0;
    let mut y = y as i32;
    let mut x = x as i32;

    loop {
        match dir {
            Direction::Up => y -= 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }

        if x < 0 || y < 0 || y as usize >= board.len() || x as usize >= board[y as usize].len() {
            is_visible = true;
            break;
        }

        trees += 1;
        if board[y as usize][x as usize] >= h {
            break;
        }
    }

    (is_visible, trees)
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Forest {
    let parsed = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|c| c - b'0')
                .map(|b| b.into())
                .collect::<Vec<usize>>()
        })
        .collect::<Input>();

    Forest(parsed)
}

#[aoc(day8, part1)]
pub fn part1(input: &Forest) -> usize {
    input.parse_inside().iter().filter(|r| r.is_visible).count() + input.count_outside()
}

#[aoc(day8, part2)]
pub fn part2(input: &Forest) -> usize {
    input
        .parse_inside()
        .iter()
        .map(|r| r.scenic_score)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r##"30373
25512
65332
33549
35390"##;

    #[test]
    fn test_part1() {
        assert_eq!(21, part1(&parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(8, part2(&parse_input(INPUT)));
    }
}
