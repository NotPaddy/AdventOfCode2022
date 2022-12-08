use aoc_2022::{
    day1::Day1, day2::Day2, day3::Day3, day4::Day4, day5::Day5, day6::Day6, day7::Day7, day8::Day8,
    *,
};
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
        1 => Runner::new(include_str!("../inputs/day01.txt")).run(&Day1),
        2 => Runner::new(include_str!("../inputs/day02.txt")).run(&Day2),
        3 => Runner::new(include_str!("../inputs/day03.txt")).run(&Day3),
        4 => Runner::new(include_str!("../inputs/day04.txt")).run(&Day4),
        5 => Runner::new(include_str!("../inputs/day05.txt")).run(&Day5),
        6 => Runner::new(include_str!("../inputs/day06.txt")).run(&Day6),
        7 => Runner::new(include_str!("../inputs/day07.txt")).run(&Day7),
        8 => Runner::new(include_str!("../inputs/day08.txt")).run(&Day8),
        _ => panic!("Could not find day {}", day),
    }
}
