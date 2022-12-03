use itertools::Itertools;

#[aoc(day3, part1)]
fn solve_part1(input: &str) -> u32 {
    let backpacks: Vec<(&str, &str)> = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .collect();

    let mut both: Vec<char> = Vec::new();
    for backpack in backpacks {
        for c in backpack.0.chars() {
            if backpack.1.chars().contains(&c) {
                both.push(c.clone());
                break;
            }
        }
    }

    let mut total = 0;

    for c in both {
        let mut value = c as u32;
        if c.is_uppercase() {
            value -= 38
        } else {
            value -= 96
        }
        total += value
    }

    total
}

#[aoc(day3, part2)]
fn solve_part2(input: &str) -> u32 {
    let mut both: Vec<char> = Vec::new();
    for group in &input.lines().chunks(3) {
        let (b1, b2, b3) = group.collect_tuple().unwrap();

        for c in b1.chars() {
            if b2.chars().contains(&c) && b3.chars().contains(&c) {
                both.push(c);
                break;
            }
        }
    }

    let mut total = 0;

    for c in both {
        let mut value = c as u32;
        if c.is_uppercase() {
            value -= 38
        } else {
            value -= 96
        }
        total += value
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            solve_part1(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            157
        )
    }

    #[test]
    fn part2() {
        assert_eq!(
            solve_part2(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            70
        )
    }
}
