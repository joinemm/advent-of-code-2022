use itertools::Itertools;
use std::collections::VecDeque;

#[aoc(day6, part1)]
fn solve_part1(input: &str) -> u32 {
    let mut queue = VecDeque::new();
    let mut i = 1;
    for c in input.chars() {
        queue.push_back(c);

        if queue.len() > 4 {
            queue.pop_front();
        }

        if queue.len() == 4 {
            let mut b: Vec<char> = queue.clone().into();
            b.sort();
            b.dedup();
            if b.len() == queue.len() {
                return i;
            }
        }
        i += 1;
    }
    0
}

#[aoc(day6, part2)]
fn solve_part2(input: &str) -> u32 {
    let mut queue = VecDeque::new();
    let mut i = 1;
    for c in input.chars() {
        queue.push_back(c);

        if queue.len() > 14 {
            queue.pop_front();
        }

        if queue.len() == 14 {
            let mut b: Vec<char> = queue.clone().into();
            b.sort();
            b.dedup();
            if b.len() == queue.len() {
                return i;
            }
        }
        i += 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use test_case::test_case;

    const EXAMPLE_INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test_case(EXAMPLE_INPUT => 7)]
    fn part1_example(input: &str) -> u32 {
        solve_part1(input)
    }

    #[test_case(EXAMPLE_INPUT => 19)]
    fn part2_example(input: &str) -> u32 {
        solve_part2(input)
    }

    #[test_case(&get_input("day6") => 1300)]
    fn part1_real(input: &str) -> u32 {
        solve_part1(input)
    }

    #[test_case(&get_input("day6") => 3986)]
    fn part2_real(input: &str) -> u32 {
        solve_part2(input)
    }
}
