use itertools::Itertools;

#[derive(PartialEq, Clone, Copy)]
pub enum Hand {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

impl Hand {
    fn from_letter(letter: char) -> Self {
        match letter {
            'A' | 'X' => Hand::ROCK,
            'B' | 'Y' => Hand::PAPER,
            'C' | 'Z' => Hand::SCISSORS,
            _ => panic!(),
        }
    }

    fn from_outcome(opponent: &Hand, letter: char) -> Self {
        match letter {
            'X' => opponent.beats(),
            'Y' => opponent.clone(),
            'Z' => opponent.beats().beats(),
            _ => panic!(),
        }
    }

    fn beats(&self) -> Self {
        match *self {
            Hand::ROCK => Hand::SCISSORS,
            Hand::PAPER => Hand::ROCK,
            Hand::SCISSORS => Hand::PAPER,
        }
    }

    fn round_outcome(&self, opponent: &Hand) -> u32 {
        (if self == opponent {
            3
        } else if self.beats() == *opponent {
            6
        } else {
            0
        }) + *self as u32
    }
}

#[aoc_generator(day2)]
pub fn input_parser(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| !c.is_whitespace())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

#[aoc(day2, part1)]
fn solve_part1(rounds: &Vec<(char, char)>) -> u32 {
    rounds
        .iter()
        .map(|(opponent, me)| Hand::from_letter(*me).round_outcome(&Hand::from_letter(*opponent)))
        .sum()
}

#[aoc(day2, part2)]
fn solve_part2(rounds: &Vec<(char, char)>) -> u32 {
    rounds
        .iter()
        .map(|(opponent, me)| {
            let opponent_hand = &Hand::from_letter(*opponent);
            Hand::from_outcome(opponent_hand, *me).round_outcome(opponent_hand)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_parser("A Y\nB X\nC Z")), 15)
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_parser("A Y\nB X\nC Z")), 12)
    }
}
