use itertools::Itertools;

fn char_value(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 38
    } else {
        c as u32 - 96
    }
}

#[aoc(day3, part1)]
fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(com1, com2)| {
            com1.chars()
                .filter(|c| com2.chars().contains(c))
                .take(1)
                .map(char_value)
                .sum::<u32>()
        })
        .sum()
}

#[aoc(day3, part2)]
fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .tuples()
        .map(|(b1, b2, b3)| {
            b1.chars()
                .filter(|c| b2.chars().contains(c) && b3.chars().contains(c))
                .take(1)
                .map(char_value)
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;
    use test_case::test_case;

    const EXAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test_case(EXAMPLE_INPUT => 157)]
    fn part1_example(input: &str) -> u32 {
        solve_part1(input)
    }

    #[test_case(EXAMPLE_INPUT => 70)]
    fn part2_example(input: &str) -> u32 {
        solve_part2(input)
    }

    #[test_case(&get_input("day3") => 8139)]
    fn part1_real(input: &str) -> u32 {
        solve_part1(input)
    }

    #[test_case(&get_input("day3") => 2668)]
    fn part2_real(input: &str) -> u32 {
        solve_part2(input)
    }
}
