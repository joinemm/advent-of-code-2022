use itertools::Itertools;

#[aoc(day10, part1)]
fn solve_part1(input: &str) -> i32 {
    let mut register = 1;
    let mut current_job: Option<i32> = None;
    let mut signals: Vec<i32> = Vec::new();
    let mut lines = input.lines();
    let mut i = 1;
    loop {
        println!("cycle {i}, X {register}");
        if i == 20 || (i > 20 && (i - 20) % 40 == 0) {
            let strength = i * register;
            println!("During the {i}th cycle, signal strength is {strength}");
            signals.push(strength)
        }
        match current_job {
            Some(job) => {
                register += job;
                current_job = None;
            }
            None => {
                let line = lines.next();
                match line {
                    Some(v) => {
                        if v.starts_with("addx") {
                            let n: i32 = v.split_once(" ").unwrap().1.parse().unwrap();
                            current_job = Some(n);
                        }
                    }
                    None => break,
                }
            }
        }
        i += 1;
    }
    signals.iter().sum()
}

#[aoc(day10, part2)]
fn solve_part2(input: &str) -> String {
    let mut register: i32 = 1;
    let mut current_job: Option<i32> = None;
    let mut crt_screen: Vec<Vec<char>> = Vec::new();
    let mut current_crt_row: Vec<char> = Vec::new();
    let mut lines = input.lines();
    let mut i = 1;
    loop {
        let mut this_pixel = '.';
        if (register..=register + 2).contains(&(i - 40 * crt_screen.len() as i32)) {
            this_pixel = '#'
        }
        current_crt_row.push(this_pixel);
        if i % 40 == 0 {
            crt_screen.push(current_crt_row);
            current_crt_row = Vec::new();
        }
        match current_job {
            Some(job) => {
                register += job;
                current_job = None;
            }
            None => {
                let line = lines.next();
                match line {
                    Some(v) => {
                        if v.starts_with("addx") {
                            let n: i32 = v.split_once(" ").unwrap().1.parse().unwrap();
                            current_job = Some(n);
                        }
                    }
                    None => break,
                }
            }
        }
        i += 1;
    }
    let result: String = crt_screen
        .iter()
        .map(|row| row.iter().collect::<String>())
        .join("\n");

    // only adding newline to the beginning so the resulting ascii letters are readable
    "\n".to_string() + &result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use test_case::test_case;

    const EXAMPLE_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

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

    #[test_case(EXAMPLE_INPUT => 13140)]
    fn part1_example(input: &str) -> i32 {
        solve_part1(input)
    }

    #[test_case(EXAMPLE_INPUT => PART2_EXAMPLE_OUTPUT)]
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
