use std::cmp::{max, min};

type Points = (u32, u32, u32, u32);

#[aoc_generator(day4)]
fn parse_input_day(input: &str) -> Vec<Points> {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(",").unwrap();
            let (x1, x2) = first.split_once("-").unwrap();
            let (x3, x4) = second.split_once("-").unwrap();
            (
                x1.parse().unwrap(),
                x2.parse().unwrap(),
                x3.parse().unwrap(),
                x4.parse().unwrap(),
            )
        })
        .collect::<Vec<Points>>()
}

#[aoc(day4, part1)]
pub fn part1(input: &Vec<Points>) -> usize {
    input
        .iter()
        .filter(|(x1, x2, x3, x4)| (x3 >= x1 && x4 <= x2) || (x1 >= x3 && x2 <= x4))
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &Vec<Points>) -> usize {
    input
        .iter()
        .filter(|(x1, x2, x3, x4)| max(x1, x3) <= min(x2, x4))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r##"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"##;

    #[test]
    fn test_example_input() {
        assert_eq!(2, part1(&parse_input_day(INPUT)));
    }

    #[test]
    fn test_example_input_part2() {
        assert_eq!(4, part2(&parse_input_day(INPUT)));
    }
}
