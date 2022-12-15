use itertools::Itertools;

#[aoc(day10, part1)]
fn solve_part1(input: &str) -> i32 {
    let mut register = 1;
    let mut current_job: Option<i32> = None;
    let mut total_strength = 0;
    let mut line_iterator = input.lines();
    let mut i = 1;
    loop {
        if i == 20 || (i - 20) % 40 == 0 {
            total_strength += i * register;
        }
        match current_job {
            Some(instruction) => {
                register += instruction;
                current_job = None;
            }
            None => match line_iterator.next() {
                Some(v) if v.starts_with("addx") => {
                    current_job = Some(v.split_once(" ").unwrap().1.parse().unwrap());
                }
                Some(_) => (),
                None => break,
            },
        }
        i += 1;
    }
    total_strength
}

#[aoc(day10, part2)]
fn solve_part2(input: &str) -> String {
    let mut register: i32 = 1;
    let mut current_job: Option<i32> = None;
    let mut crt: Vec<Vec<char>> = Vec::new();
    let mut crt_register: Vec<char> = Vec::new();
    let mut line_iterator = input.lines();
    let mut i = 1;
    loop {
        crt_register.push(match () {
            _ if (register..=register + 2).contains(&(i - 40 * crt.len() as i32)) => '#',
            _ => '.',
        });
        if i % 40 == 0 {
            crt.push(crt_register.clone());
            crt_register.clear();
        }
        match current_job {
            Some(instruction) => {
                register += instruction;
                current_job = None;
            }
            None => match line_iterator.next() {
                Some(v) if v.starts_with("addx") => {
                    current_job = Some(v.split_once(" ").unwrap().1.parse().unwrap());
                }
                Some(_) => (),
                None => break,
            },
        }
        i += 1;
    }
    let result: String = crt
        .iter()
        .map(|row| row.iter().collect::<String>())
        .join("\n");

    // only adding newline to the beginning so the resulting ascii letters are more readable
    "\n".to_string() + &result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use test_case::test_case;

    const PART2_EXAMPLE_OUTPUT: &str = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    const REAL_OUTPUT: &str = "
###..#....####.####.#..#.#....###..###..
#..#.#....#....#....#..#.#....#..#.#..#.
#..#.#....###..###..#..#.#....#..#.###..
###..#....#....#....#..#.#....###..#..#.
#....#....#....#....#..#.#....#....#..#.
#....####.####.#.....##..####.#....###..";

    #[test_case(&get_input("day10_example") => 13140)]
    fn part1_example(input: &str) -> i32 {
        solve_part1(input)
    }

    #[test_case(&get_input("day10_example") => PART2_EXAMPLE_OUTPUT)]
    fn part2_example(input: &str) -> String {
        solve_part2(input)
    }

    #[test_case(&get_input("day10") => 16480)]
    fn part1_real(input: &str) -> i32 {
        solve_part1(input)
    }

    #[test_case(&get_input("day10") => REAL_OUTPUT)]
    fn part2_real(input: &str) -> String {
        solve_part2(input)
    }
}
