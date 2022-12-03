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
    use std::fs;

    #[test]
    fn part1_example() {
        assert_eq!(
            solve_part1(&input_parser(
                "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"
            )),
            24000
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&input_parser(
                "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"
            )),
            45000
        );
    }

    #[test]
    fn part1_real() {
        assert_eq!(
            solve_part1(&input_parser(
                &fs::read_to_string("input/2022/day1.txt").unwrap()
            )),
            65912
        )
    }

    #[test]
    fn part2_real() {
        assert_eq!(
            solve_part2(&input_parser(
                &fs::read_to_string("input/2022/day1.txt").unwrap()
            )),
            195625
        )
    }
}
