use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::io::{repeat, Read};
use std::ops::{Add, AddAssign, SubAssign};
use std::{fmt, iter};

#[derive(Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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
            head = Point {
                x: head.x + delta.x,
                y: head.y + delta.y,
            };

            if tail.x.abs_diff(head.x) > 1 || tail.y.abs_diff(head.y) > 1 {
                tail = Point {
                    x: head.x - delta.x,
                    y: head.y - delta.y,
                };
                visits.insert(tail.clone());
            }
        }
    }

    visits.len()
}

#[aoc(day9, part2)]
fn part2(input: &Input) -> usize {
    let mut head = Point { x: 0, y: 0 };
    let mut tails: Vec<Point> = iter::repeat(head.clone()).take(9).collect();
    let mut visits = Visits::new();

    visits.insert(head.clone());

    for (dir, amount) in input {
        for _ in 0..*amount {
            let delta = dir.delta();
            head = Point {
                x: head.x + delta.x,
                y: head.y + delta.y,
            };

            for i in 0..tails.len() {
                let prev: &Point = if i == 0 { &head } else { &tails[0] };

                let curr = &tails[i];
            }
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
