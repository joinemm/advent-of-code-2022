use std::collections::HashSet;

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_to(&mut self, dir: &str) {
        match dir {
            "R" => self.x += 1,
            "L" => self.x -= 1,
            "U" => self.y += 1,
            "D" => self.y -= 1,
            _ => panic!(),
        }
    }

    fn move_towards(&mut self, point: &Point) {
        if !self.is_touching(&point) {
            self.x += (point.x - self.x).clamp(-1, 1);
            self.y += (point.y - self.y).clamp(-1, 1);
        }
    }

    fn is_touching(&self, point: &Point) -> bool {
        return (self.x - point.x).abs() <= 1 && (self.y - point.y).abs() <= 1;
    }
}

#[aoc(day9, part1)]
fn solve_part1(input: &str) -> u32 {
    let mut tail_spots = HashSet::<Point>::new();
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    for (command, count) in input.lines().map(|l| l.split(" ").collect_tuple().unwrap()) {
        for _ in 0..count.parse().unwrap() {
            head.move_to(command);
            tail.move_towards(&head);
            tail_spots.insert(tail.clone());
        }
    }

    tail_spots.len() as u32
}

#[aoc(day9, part2)]
fn solve_part2(input: &str) -> u32 {
    let mut tail_spots = HashSet::<Point>::new();
    let mut knots = vec![Point { x: 0, y: 0 }; 10];
    for (command, count) in input.lines().map(|l| l.split(" ").collect_tuple().unwrap()) {
        for _ in 0..count.parse().unwrap() {
            knots[0].move_to(command);
            for i in 1..knots.len() {
                let parent = knots[i - 1];
                knots[i].move_towards(&parent);
            }
            tail_spots.insert(knots.last().unwrap().clone());
        }
    }

    tail_spots.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use test_case::test_case;

    const EXAMPLE_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    const EXAMPLE_INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test_case(EXAMPLE_INPUT => 13)]
    fn part1_example(input: &str) -> u32 {
        solve_part1(input)
    }

    #[test_case(EXAMPLE_INPUT => 1)]
    fn part2_example(input: &str) -> u32 {
        solve_part2(input)
    }

    #[test_case(EXAMPLE_INPUT_2 => 36)]
    fn part2_example_2(input: &str) -> u32 {
        solve_part2(input)
    }

    #[test_case(&get_input("day9") => 5874)]
    fn part1_real(input: &str) -> u32 {
        solve_part1(input)
    }

    #[test_case(&get_input("day9") => 2467)]
    fn part2_real(input: &str) -> u32 {
        solve_part2(input)
    }
}
