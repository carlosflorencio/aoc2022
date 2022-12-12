enum Operation {
    Addx(i32),
    Noop,
}

type Input = Vec<i32>;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Input {
    let operations = input
        .lines()
        .map(|line| {
            if line.contains("noop") {
                Operation::Noop
            } else {
                let (_, right) = line.split_once(" ").unwrap();

                Operation::Addx(right.parse::<i32>().unwrap())
            }
        })
        .collect::<Vec<Operation>>();

    operations
        .iter()
        .flat_map(|op| match op {
            Operation::Addx(num) => {
                vec![0, *num]
            }
            Operation::Noop => {
                vec![0]
            }
        })
        .collect()
}

#[aoc(day10, part1)]
fn part1(input: &Input) -> i32 {
    let mut register = 1;

    let checks = [20, 60, 100, 140, 180, 220];
    let mut sums: Vec<i32> = vec![];

    for i in 0..input.len() {
        let cycle = i as i32 + 1;

        if checks.contains(&cycle) {
            sums.push(cycle * register);
        }

        register += input[i];
    }

    sums.iter().sum()
}

#[aoc(day10, part2)]
fn part2(input: &Input) -> usize {
    let mut register: i32 = 1;
    let mut line: Vec<char> = Vec::with_capacity(40);

    for i in 0..input.len() {
        let cycle = i as i32 + 1;
        let crt_pos: i32 = cycle % 40;

        let char = if crt_pos >= register && crt_pos <= register + 2 {
            '#'
        } else {
            '.'
        };

        line.push(char);

        if cycle % 40 == 0 {
            let str: String = line.iter().collect();
            println!("{}", str);
            line = Vec::with_capacity(40);
        }

        register += input[i];
    }

    0 // workaround aoc limitation
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r##"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"##;

    #[test]
    fn test_part1() {
        assert_eq!(13140, part1(&parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1, part2(&parse_input(INPUT)));
    }
}
