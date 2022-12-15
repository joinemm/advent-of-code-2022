use itertools::Itertools;
use take_until::TakeUntilExt;

#[aoc_generator(day8)]
fn input_parser(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect()
}

#[aoc(day8, part1)]
fn solve_part1(matrix: &Vec<Vec<u32>>) -> u32 {
    let mut visible = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let value = &matrix[i][j];
            if i == 0
                || j == 0
                || i == matrix.len() - 1
                || j == matrix[i].len() - 1
                || matrix[i].iter().take(j).max() < Some(value)
                || matrix[i].iter().skip(j + 1).max() < Some(value)
                || (0..i).map(|n| matrix[n][j]).max() < Some(*value)
                || (i + 1..matrix.len()).map(|n| matrix[n][j]).max() < Some(*value)
            {
                visible += 1;
            }
        }
    }

    visible
}

#[aoc(day8, part2)]
fn solve_part2(matrix: &Vec<Vec<u32>>) -> u32 {
    let mut scores = Vec::<u32>::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            scores.push(
                [
                    matrix[i].iter().take(j).rev().collect_vec(),
                    matrix[i].iter().skip(j + 1).collect_vec(),
                    (0..i).map(|n| &matrix[n][j]).rev().collect_vec(),
                    (i + 1..matrix.len()).map(|n| &matrix[n][j]).collect_vec(),
                ]
                .map(|dir| dir.iter().take_until(|h| **h >= &matrix[i][j]).count() as u32)
                .iter()
                .product(),
            )
        }
    }

    *scores.iter().max().unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use test_case::test_case;

    const EXAMPLE_INPUT: &str = "30373
25512
65332
33549
35390
";

    #[test_case(EXAMPLE_INPUT => 21)]
    fn part1_example(input: &str) -> u32 {
        solve_part1(&input_parser(input))
    }

    #[test_case(EXAMPLE_INPUT => 8)]
    fn part2_example(input: &str) -> u32 {
        solve_part2(&input_parser(input))
    }

    #[test_case(&get_input("day8") => 1807)]
    fn part1_real(input: &str) -> u32 {
        solve_part1(&input_parser(input))
    }

    #[test_case(&get_input("day8") => 480000)]
    fn part2_real(input: &str) -> u32 {
        solve_part2(&input_parser(input))
    }
}
