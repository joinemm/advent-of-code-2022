extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

#[cfg(test)]
use std::fs;

#[cfg(test)]
fn get_input(day: &str) -> String {
    fs::read_to_string(format!("input/2022/{}.txt", day)).unwrap()
}

aoc_lib! { year = 2022 }
