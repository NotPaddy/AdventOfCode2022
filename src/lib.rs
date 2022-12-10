#![feature(iter_array_chunks, array_windows)]

use std::fmt::Display;

pub trait Solution<const DAY: u8> {
    type Output: Display;

    fn part1(&self, input: &str) -> Self::Output;
    fn part2(&self, input: &str) -> Option<Self::Output> {
        let _ = input;
        None
    }
}

pub struct Runner<'a> {
    input: &'a str,
}

impl Runner<'_> {
    pub fn new(input: &'_ str) -> Runner<'_> {
        Runner { input }
    }

    pub fn run<const DAY: u8, S: Solution<DAY>>(&'_ self, solution: &S) {
        let input = &self.input.replace("\r\n", "\n");
        println!("--------");
        println!("Day {:02}", DAY);
        println!("--------");

        let result = solution.part1(input);
        println!("Part 1:\n{}", result);

        let result = solution.part2(input);
        if let Some(result) = result {
            println!("Part 2:\n{}", result);
        } else {
            println!("Part 2:\nNot solved");
        }
    }
}

pub mod day1;
pub mod day10;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
