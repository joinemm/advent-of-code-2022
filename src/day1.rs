#[aoc_generator(day1)]
pub fn input_parser(input: &str) -> Vec<u32> {
    input.lines().fold(vec![0], |mut elves, line| {
        if line.is_empty() {
            elves.push(0);
        } else {
            *elves.last_mut().unwrap() += line.parse::<u32>().unwrap();
        }
        elves
    })
}

#[aoc(day1, part1)]
pub fn solve_part1(elves: &Vec<u32>) -> u32 {
    *elves.iter().max().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(elves: &Vec<u32>) -> u32 {
    let mut sorted_elves = elves.clone();
    sorted_elves.sort();
    (0..3).map(|_| sorted_elves.pop().unwrap()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            solve_part1(&input_parser(
                "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"
            )),
            24000
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            solve_part2(&input_parser(
                "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"
            )),
            45000
        );
    }
}
