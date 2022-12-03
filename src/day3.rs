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
                .take_while(|c| !com2.chars().contains(c))
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
                .take_while(|c| !(b2.chars().contains(c) && b3.chars().contains(c)))
                .map(char_value)
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            solve_part1(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            157
        )
    }

    #[test]
    fn part2() {
        assert_eq!(
            solve_part2(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            70
        )
    }
}
