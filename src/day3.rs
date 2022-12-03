fn calculate_priority(c: &char) -> u32 {
    if c >= &'a' {
        return *c as u32 - 'a' as u32 + 1;
    } else {
        return *c as u32 - 'A' as u32 + 27;
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    input.lines().map(|line| {
        let half_idx = line.len() / 2;
        let first_half = &line[0..half_idx];
        let second_half = &line[half_idx..line.len()];

        let mut common: Option<char> = None;

        for c in first_half.chars() {
            if let Some(_) = second_half.find(c) {
                common = Some(c);
                break;
            }
        }

        if let Some(found_char) = common {
            return calculate_priority(&found_char);
        }

        return 0;
    }).sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut sum = 0;

    for group in lines.chunks(3) {
        for c in group[0].chars() {
            let mut found_total = 0;
            for i in 1..group.len() {
                if let Some(_) = group[i].find(c) {
                    found_total = found_total + 1;
                } else {
                    break; // we need all lines to have the char, continue to the next one
                }
            }

            // did all lines had the same char?
            if found_total == group.len() - 1 {
                sum = sum + calculate_priority(&c);
                break;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::*;

    const INPUT: &str = r##"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"##;

    #[test]
    fn test_example_input() {
        assert_eq!(157, part1(INPUT));
    }

    #[test]
    fn test_given_input() {
        let input = read_to_string("./input/2022/day3.txt").expect("Failed to read input file");
        assert_eq!(8202, part1(&input));
    }

    #[test]
    fn test_example_input_part2() {
        assert_eq!(70, part2(INPUT));
    }
}
