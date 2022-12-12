use std::collections::HashSet;
use std::iter;

#[derive(Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_to(&mut self, direction: &Point) {
        *self = Point {
            x: self.x + direction.x,
            y: self.y + direction.y,
        }
    }

    fn is_adjacent(&self, other: &Point) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }

    fn follow(&mut self, head: &Point) {
        let dx = head.x - self.x;
        let dy = head.y - self.y;

        self.x += if dx.abs() == 2 { dx / 2 } else { dx };
        self.y += if dy.abs() == 2 { dy / 2 } else { dy };
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(&self) -> Point {
        match self {
            Direction::Up => Point { x: 0, y: -1 },
            Direction::Down => Point { x: 0, y: 1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
        }
    }
}

type Visits = HashSet<Point>;
type Input = Vec<(Direction, u32)>;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" ").unwrap();

            let direction = match left {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Unknown direction"),
            };

            (
                direction,
                right.parse::<u32>().expect("Failed to parse amount"),
            )
        })
        .collect::<Input>()
}

#[aoc(day9, part1)]
fn part1(input: &Input) -> usize {
    let mut head = Point { x: 0, y: 0 };
    let mut tail = head.clone(); // overlap
    let mut visits = Visits::new();

    visits.insert(head.clone());

    for (dir, amount) in input {
        for _ in 0..*amount {
            let delta = dir.delta();
            head.move_to(&delta);

            if !tail.is_adjacent(&head) {
                tail.follow(&head);
                visits.insert(tail.clone());
            }
        }
    }

    visits.len()
}

#[aoc(day9, part2)]
fn part2(input: &Input) -> usize {
    let mut knots: Vec<Point> = Vec::with_capacity(10);
    let mut visits = Visits::new();

    knots.push(Point { x: 0, y: 0 });
    iter::repeat(knots[0].clone())
        .take(9)
        .for_each(|tail| knots.push(tail));

    visits.insert(knots[0].clone());

    for (dir, amount) in input {
        for _ in 0..*amount {
            let delta = dir.delta();
            let head = &mut knots[0];
            head.x = head.x + delta.x;
            head.y = head.y + delta.y;

            for i in 1..knots.len() {
                let prev = &knots[i - 1].clone();
                if !knots[i].is_adjacent(&knots[i - 1]) {
                    knots[i].follow(prev);
                }
            }

            visits.insert(knots[knots.len() - 1].clone());
        }
    }

    visits.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r##"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"##;

    #[test]
    fn test_part1() {
        assert_eq!(13, part1(&parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1, part2(&parse_input(INPUT)));
    }
}
