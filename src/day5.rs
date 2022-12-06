use itertools::Itertools;

#[aoc(day5, part1)]
fn solve_part1(input: &str) -> String {
    let (crates, instructions) = input.split_once("\n\n").unwrap();

    let mut crate_vecs: Vec<Vec<char>> = Vec::new();
    for line in crates
        .lines()
        .rev()
        .collect::<Vec<_>>()
        .split_first()
        .unwrap()
        .1
    {
        let mut n = 1;
        let mut i = 0;
        loop {
            let c = line.chars().nth(n);
            match c {
                Some(c) => {
                    if c.is_alphabetic() {
                        if crate_vecs.len() > i {
                            crate_vecs[i].push(c);
                        } else {
                            crate_vecs.push(vec![c])
                        }
                    }
                }
                None => break,
            };
            n += 4;
            i += 1;
        }
    }

    for ins in instructions.lines() {
        let splits: Vec<&str> = ins.split(" ").collect();
        let moves: usize = splits[1].parse().unwrap();
        let from: usize = splits[3].parse().unwrap();
        let to: usize = splits[5].parse().unwrap();

        for _ in 0..moves {
            let val = crate_vecs[from - 1].pop().unwrap();
            crate_vecs[to - 1].push(val);
        }
    }

    crate_vecs
        .iter()
        .map(|stack| stack.last().unwrap())
        .join("")
}

#[aoc(day5, part2)]
fn solve_part2(input: &str) -> String {
    let (crates, instructions) = input.split_once("\n\n").unwrap();

    let mut crate_vecs: Vec<Vec<char>> = Vec::new();
    for line in crates
        .lines()
        .rev()
        .collect::<Vec<_>>()
        .split_first()
        .unwrap()
        .1
    {
        let mut n = 1;
        let mut i = 0;
        loop {
            let c = line.chars().nth(n);
            match c {
                Some(c) => {
                    if c.is_alphabetic() {
                        if crate_vecs.len() > i {
                            crate_vecs[i].push(c);
                        } else {
                            crate_vecs.push(vec![c])
                        }
                    }
                }
                None => break,
            };
            n += 4;
            i += 1;
        }
    }

    for ins in instructions.lines() {
        let splits: Vec<&str> = ins.split(" ").collect();
        let moves: usize = splits[1].parse().unwrap();
        let from: usize = splits[3].parse().unwrap();
        let to: usize = splits[5].parse().unwrap();

        let from_i = crate_vecs[from - 1].len() - moves;
        let new_vals: Vec<char> = crate_vecs[from - 1].drain(from_i..).collect();
        for val in new_vals {
            crate_vecs[to - 1].push(val);
        }
    }

    crate_vecs
        .iter()
        .map(|stack| stack.last().unwrap())
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use test_case::test_case;

    const EXAMPLE_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test_case(EXAMPLE_INPUT => "CMZ".to_string())]
    fn part1_example(input: &str) -> String {
        solve_part1(input)
    }

    #[test_case(EXAMPLE_INPUT => "MCD".to_string())]
    fn part2_example(input: &str) -> String {
        solve_part2(input)
    }

    #[test_case(&get_input("day5") => "RNZLFZSJH".to_string())]
    fn part1_real(input: &str) -> String {
        solve_part1(input)
    }

    #[test_case(&get_input("day5") => "CNSFCGJSM".to_string())]
    fn part2_real(input: &str) -> String {
        solve_part2(input)
    }
}
