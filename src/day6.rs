use std::collections::HashSet;

fn items_unique(bytes: &[u8]) -> bool {
    let mut existing: HashSet<u8> = HashSet::new();

    bytes.iter().for_each(|b| {
        existing.insert(*b);
    });

    existing.len() == bytes.len()
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let pos = input.as_bytes().windows(4).position(|c| items_unique(c));
    pos.unwrap() + 4
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let pos = input.as_bytes().windows(14).position(|c| items_unique(c));
    pos.unwrap() + 14
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r##"bvwbjplbgvbhsrlpgdmjqwftvncz"##;

    #[test]
    fn test_example_input() {
        assert_eq!(5, part1(INPUT));
    }
}
