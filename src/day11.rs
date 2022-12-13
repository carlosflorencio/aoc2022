#[derive(Clone)]
struct Operation {
    str: String,
}

impl Operation {
    fn apply(&self, item: u64) -> u64 {
        let parts: Vec<&str> = self.str.split(" ").collect();

        let mut right: u64 = item;
        if parts[2] != "old" {
            right = parts[2].parse::<u64>().unwrap();
        }

        match parts[1] {
            "*" => item.wrapping_mul(right),
            "+" => item.wrapping_add(right),
            _ => panic!("operator not implemented"),
        }
    }
}

#[derive(Clone)]
struct Test {
    divisible: u64,
    target_positive: usize,
    target_negative: usize,
}

impl Test {
    fn result(&self, item: u64) -> usize {
        if item % self.divisible == 0 {
            self.target_positive
        } else {
            self.target_negative
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
    inspections: usize,
}

impl Monkey {
    fn turn(&mut self, transform: impl Fn(u64) -> u64) -> Vec<(usize, u64)> {
        let mut give: Vec<(usize, u64)> = Vec::new();

        loop {
            if self.items.len() == 0 {
                break;
            }

            self.inspections += 1;
            let mut c = self.operation.apply(self.items.pop().unwrap());
            c = transform(c);

            let target = self.test.result(c);

            give.push((target, c));

            if self.items.len() == 0 {
                break;
            }
        }

        give
    }
}

type Input = Vec<Monkey>;

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|m| {
            let lines: Vec<&str> = m.lines().skip(1).collect();

            let (_, numbers_str) = lines[0].split_once(": ").unwrap();
            let items: Vec<u64> = numbers_str
                .split(", ")
                .map(|str| str.parse::<u64>().unwrap())
                .collect();

            let operation = Operation {
                str: lines[1].split(" = ").collect::<Vec<&str>>()[1].into(),
            };

            let d = lines[2].split(" by ").collect::<Vec<&str>>()[1]
                .parse::<u64>()
                .unwrap();

            let positive = lines[3].split("monkey ").collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap();

            let negative = lines[4].split("monkey ").collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap();

            Monkey {
                items,
                operation,
                test: Test {
                    divisible: d,
                    target_positive: positive,
                    target_negative: negative,
                },
                inspections: 0,
            }
        })
        .collect::<Vec<Monkey>>()
}

#[aoc(day11, part1)]
fn part1(input: &Input) -> usize {
    let mut input = input.clone();

    for _ in 0..20 {
        for i in 0..input.len() {
            let give = input[i].turn(|w| w / 3);

            give.iter()
                .rev()
                .for_each(|(target, item)| input[*target].items.push(*item));
        }
    }

    let mut inspections = input.iter().map(|m| m.inspections).collect::<Vec<usize>>();
    inspections.sort();

    inspections.iter().rev().take(2).product()
}

#[aoc(day11, part2)]
fn part2(input: &Input) -> usize {
    let mut input = input.clone();
    let modulus = input.iter().map(|m| m.test.divisible).product::<u64>();

    for _ in 0..10000 {
        for i in 0..input.len() {
            let give = input[i].turn(|w| w % modulus);

            give.iter()
                .rev()
                .for_each(|(target, item)| input[*target].items.push(*item));
        }
    }

    let mut inspections = input.iter().map(|m| m.inspections).collect::<Vec<usize>>();
    inspections.sort();

    inspections.iter().rev().take(2).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r##"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"##;

    #[test]
    fn test_part1() {
        assert_eq!(10605, part1(&parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2713310158, part2(&parse_input(INPUT)));
    }
}
