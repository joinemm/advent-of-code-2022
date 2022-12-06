use std::collections::HashSet;

#[aoc(day4, part1)]
fn solve_part1(input: &str) -> u32 {
    let elves: Vec<(&str, &str)> = input
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .collect();
    let mut pairs = 0;
    for (elf1, elf2) in elves {
        let elf1_nums: HashSet<u32> = elf1
            .split_once("-")
            .map(|(s, e)| (s.parse::<u32>().unwrap()..=e.parse::<u32>().unwrap()).collect())
            .unwrap();

        let elf2_nums: HashSet<u32> = elf2
            .split_once("-")
            .map(|(s, e)| (s.parse::<u32>().unwrap()..=e.parse::<u32>().unwrap()).collect())
            .unwrap();

        if elf1_nums.is_subset(&elf2_nums) || elf2_nums.is_subset(&elf1_nums) {
            pairs += 1;
        }
    }
    pairs
}

#[aoc(day4, part2)]
fn solve_part2(input: &str) -> u32 {
    let elves: Vec<(&str, &str)> = input
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .collect();
    let mut pairs = 0;
    for (elf1, elf2) in elves {
        let elf1_nums: HashSet<u32> = elf1
            .split_once("-")
            .map(|(s, e)| (s.parse::<u32>().unwrap()..=e.parse::<u32>().unwrap()).collect())
            .unwrap();

        let elf2_nums: HashSet<u32> = elf2
            .split_once("-")
            .map(|(s, e)| (s.parse::<u32>().unwrap()..=e.parse::<u32>().unwrap()).collect())
            .unwrap();

        if elf1_nums.intersection(&elf2_nums).count() > 0 {
            pairs += 1;
        }
    }
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use test_case::test_case;

    const EXAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test_case(EXAMPLE_INPUT => 2)]
    fn part1_example(input: &str) -> u32 {
        solve_part1(input)
    }

    #[test_case(EXAMPLE_INPUT => 4)]
    fn part2_example(input: &str) -> u32 {
        solve_part2(input)
    }

    #[test_case(&get_input("day4") => 424)]
    fn part1_real(input: &str) -> u32 {
        solve_part1(input)
    }

    // #[test_case(&get_input("day4") => 2668)]
    // fn part2_real(input: &str) -> u32 {
    //     solve_part2(input)
    // }
}
