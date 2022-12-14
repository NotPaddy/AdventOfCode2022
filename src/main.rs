use aoc_2022::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(long_about = None)]
struct Args {
    #[arg(default_value_t = 1)]
    day: u8,
}

fn main() {
    let day = Args::parse().day;

    match day {
        1 => Runner::new(include_str!("../inputs/day01.txt")).run(&day1::Day1),
        2 => Runner::new(include_str!("../inputs/day02.txt")).run(&day2::Day2),
        3 => Runner::new(include_str!("../inputs/day03.txt")).run(&day3::Day3),
        4 => Runner::new(include_str!("../inputs/day04.txt")).run(&day4::Day4),
        5 => Runner::new(include_str!("../inputs/day05.txt")).run(&day5::Day5),
        6 => Runner::new(include_str!("../inputs/day06.txt")).run(&day6::Day6),
        7 => Runner::new(include_str!("../inputs/day07.txt")).run(&day7::Day7),
        8 => Runner::new(include_str!("../inputs/day08.txt")).run(&day8::Day8),
        9 => Runner::new(include_str!("../inputs/day09.txt")).run(&day9::Day9),
        10 => Runner::new(include_str!("../inputs/day10.txt")).run(&day10::Day10),
        11 => Runner::new(include_str!("../inputs/day11.txt")).run(&day11::Day11),
        12 => Runner::new(include_str!("../inputs/day12.txt")).run(&day12::Day12),
        13 => Runner::new(include_str!("../inputs/day13.txt")).run(&day13::Day13),
        14 => Runner::new(include_str!("../inputs/day14.txt")).run(&day14::Day14),
        _ => panic!("Could not find day {}", day),
    }
}
