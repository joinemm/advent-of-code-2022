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

#[aoc(day2, part1)]
fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| !c.is_whitespace())
                .map(Hand::from_letter)
                .tuples()
                .map(|(opp, me)| me.round_outcome(&opp))
                .sum::<u32>()
        })
        .sum()
}

#[aoc(day2, part2)]
fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| !c.is_whitespace())
                .tuples()
                .map(|(opp, me)| {
                    let opp_hand = &Hand::from_letter(opp);
                    Hand::from_outcome(opp_hand, me).round_outcome(&opp_hand)
                })
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve_part1("A Y\nB X\nC Z"), 15)
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2("A Y\nB X\nC Z"), 12)
    }
}
