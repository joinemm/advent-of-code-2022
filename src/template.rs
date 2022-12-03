use itertools::Itertools;

// #[aoc_generator(dayX)]
// pub fn input_parser(input: &str) -> &str {}

#[aoc(dayX, part1)]
fn solve_part1(input: &str) -> u32 {}

// #[aoc(dayX, part2)]
// fn solve_part2(input: &str) -> u32 {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(""), 000)
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(solve_part2(""), 000)
    // }

    // #[test]
    // fn part1_real() {
    //     assert_eq!(
    //         solve_part1(&fs::read_to_string("input/2022/dayX.txt").unwrap()),
    //         000
    //     )
    // }

    // #[test]
    // fn part2_real() {
    //     assert_eq!(
    //         solve_part2(&fs::read_to_string("input/2022/dayX.txt").unwrap()),
    //         000
    //     )
    // }
}
