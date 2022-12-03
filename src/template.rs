use itertools::Itertools;

#[aoc(dayX, part1)]
fn solve_part1(input: &str) -> u32 {}

// #[aoc(dayX, part2)]
// fn solve_part2(input: &str) -> u32 {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;
    use test_case::test_case;

    const EXAMPLE_INPUT: &str = "";

    #[test_case(EXAMPLE_INPUT => 157)]
    fn part1_example(input: &str) -> u32 {
        solve_part1(input)
    }

    // #[test_case(EXAMPLE_INPUT => 70)]
    // fn part2_example(input: &str) -> u32 {
    //     solve_part2(input)
    // }

    // #[test_case(&get_input("dayX") => 8139)]
    // fn part1_real(input: &str) -> u32 {
    //     solve_part1(input)
    // }

    // #[test_case(&get_input("dayX") => 2668)]
    // fn part2_real(input: &str) -> u32 {
    //     solve_part2(input)
    // }
}
