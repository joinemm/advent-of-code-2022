use itertools::Itertools;

#[aoc(dayX, part1)]
fn solve_part1(input: &str) -> u32 {
    1
}

// #[aoc(dayX, part2)]
// fn solve_part2(input: &str) -> u32 {
//     1
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use test_case::test_case;

    const EXAMPLE_INPUT: &str = "";

    #[test_case(EXAMPLE_INPUT => 000)]
    fn part1_example(input: &str) -> u32 {
        solve_part1(input)
    }

    // #[test_case(EXAMPLE_INPUT => 000)]
    // fn part2_example(input: &str) -> u32 {
    //     solve_part2(input)
    // }

    // #[test_case(&get_input("dayX") => 000)]
    // fn part1_real(input: &str) -> u32 {
    //     solve_part1(input)
    // }

    // #[test_case(&get_input("dayX") => 000)]
    // fn part2_real(input: &str) -> u32 {
    //     solve_part2(input)
    // }
}
