#[aoc_generator(day5)]
fn parse_input_day(input: &str) -> (Vec<Vec<char>>, Vec<Vec<usize>>) {
    let (stack, moves) = input.split_once("\n\n").unwrap();

    let length = stack
        .lines()
        .last()
        .unwrap()
        .chars()
        .filter(|c| *c != ' ')
        .count();

    let mut board: Vec<Vec<char>> = vec![vec![]; length];

    stack.lines().rev().skip(1).for_each(|l| {
        l.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, c)| *c != ' ')
            .for_each(|(i, c)| board[i].push(c))
    });

    let moves = moves
        .lines()
        .map(|l| {
            l.split(" ")
                .skip(1)
                .step_by(2)
                .map(|c| c.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    (board, moves)
}

#[aoc(day5, part1)]
pub fn part1(input: &(Vec<Vec<char>>, Vec<Vec<usize>>)) -> String {
    let input = input.clone(); // cargo aoc workaround
    let mut board = input.0;
    let moves = input.1;

    for m in moves {
        for _ in 0..m[0] {
            let t = board[m[1] - 1].pop().unwrap();
            board[m[2] - 1].push(t);
        }
    }

    board.iter().map(|v| v.last().unwrap()).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r##"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"##;

    #[test]
    fn test_example_input() {
        assert_eq!("CMZ", part1(&parse_input_day(INPUT)));
    }
}
