#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let sums = sums_calculator(input);

   return *sums.iter().max().unwrap();
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut sums = sums_calculator(input);

    sums.sort();

    return sums.iter().rev().take(3).sum();
}

pub fn sums_calculator(input: &str) -> Vec<u32> {
    let elfs = input.split("\n\n");

    let sums: Vec<u32> = elfs.map(|calories| {
        calories.lines().fold(0, |acc, line| acc + line.parse::<u32>().unwrap())
    }).collect();

    sums
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r##"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"##;

    #[test]
    fn test_example_input() {
        assert_eq!(24000, part1(INPUT));
    }

    #[test]
    fn test_example_input_part2() {
        assert_eq!(45000, part2(INPUT));
    }
}
