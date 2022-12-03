#[derive(Debug, PartialEq, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn points(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3
        }
    }
}

enum HandResult {
    Win,
    Draw,
    Lose,
}

impl HandResult {
    fn points(&self) -> u32 {
        match self {
            HandResult::Win => 6,
            HandResult::Draw => 3,
            HandResult::Lose => 0,
        }
    }
}

impl Hand {
    fn new(char: char) -> Self {
        match char {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("Not recognized char")
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Move {
    player1: Hand,
    player2: Hand,
}

trait Game {
    fn score(&self) -> u32;
    fn score_alternative(&self) -> u32;
}

// Calculate the final result for the play
fn play_result(other_hand: &Hand, my_hand: &Hand) -> HandResult {
    match (other_hand, my_hand) {
        (Hand::Paper, Hand::Paper) | (Hand::Rock, Hand::Rock) | (Hand::Scissors, Hand::Scissors) => HandResult::Draw,
        (Hand::Paper, Hand::Rock) | (Hand::Rock, Hand::Scissors) | (Hand::Scissors, Hand::Paper) => HandResult::Lose,
        (Hand::Paper, Hand::Scissors) | (Hand::Rock, Hand::Paper) | (Hand::Scissors, Hand::Rock) => HandResult::Win,
    }
}

impl Move {
    fn new(line: &str) -> Self {
        let chars: Vec<char> = line.chars().collect();
        Self {
            player1: Hand::new(chars[0]),
            player2: Hand::new(chars[2]),
        }
    }


    fn find_required_play(&self) -> Hand {
        // needs a draw
        if self.player2 == Hand::Paper {
            return self.player1.clone();
        }

        // needs to lose
        if self.player2 == Hand::Rock {
            return match self.player1 {
                Hand::Rock => Hand::Scissors,
                Hand::Paper => Hand::Rock,
                Hand::Scissors => Hand::Paper,
            };
        }

        // needs to win
        return match self.player1 {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        };
    }
}

impl Game for Move {
    fn score(&self) -> u32 {
        let player_score = self.player2.points();

        let move_score = play_result(&self.player1, &self.player2).points();

        player_score + move_score
    }

    fn score_alternative(&self) -> u32 {
        let required_hand = self.find_required_play();

        let move_score = play_result(&self.player1, &required_hand).points();

        required_hand.points() + move_score
    }
}

#[aoc_generator(day2)]
fn parse_input_day(input: &str) -> Vec<Move> {
    input.lines().map(|line| Move::new(line)).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<Move>) -> u32 {
    input.iter().fold(0, |acc, curr| acc + curr.score())
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<Move>) -> u32 {
    input.iter().fold(0, |acc, curr| acc + curr.score_alternative())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r##"A Y
B X
C Z"##;

    #[test]
    fn test_input_generator() {
        let take = parse_input_day(INPUT);
        assert_eq!(vec![
            Move { player1: Hand::Rock, player2: Hand::Paper },
            Move { player1: Hand::Paper, player2: Hand::Rock },
            Move { player1: Hand::Scissors, player2: Hand::Scissors },
        ], take);
    }

    #[test]
    fn test_example_input() {
        assert_eq!(15, part1(&parse_input_day(INPUT)));
    }

    #[test]
    fn test_example_input_part2() {
        assert_eq!(12, part2(&parse_input_day(INPUT)));
    }
}
