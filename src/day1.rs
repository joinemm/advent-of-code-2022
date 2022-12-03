use itertools::Itertools;
use std::cmp::Reverse;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .sorted_by_key(|&x: &u32| Reverse(x))
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;
    use test_case::test_case;

    const EXAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test_case(EXAMPLE_INPUT => 24000)]
    fn part1_example(input: &str) -> u32 {
        solve_part1(input)
    }

    #[test_case(EXAMPLE_INPUT => 45000)]
    fn part2_example(input: &str) -> u32 {
        solve_part2(input)
    }

    #[test_case(&get_input("day1") => 65912)]
    fn part1_real(input: &str) -> u32 {
        solve_part1(input)
    }

    #[test_case(&get_input("day1") => 195625)]
    fn part2_real(input: &str) -> u32 {
        solve_part2(input)
    }
}
