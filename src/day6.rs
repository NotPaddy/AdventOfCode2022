#![feature(array_windows)]

use aoc_2022::{Runner, Solution};
use std::collections::HashSet;
use std::hash::Hash;

fn main() {
    Runner::new(include_str!("../inputs/day06.txt")).run(&Day6)
}
struct Day6;

impl Solution<6> for Day6 {
    type Output = usize;

    fn part1(&self, input: &str) -> Self::Output {
        n_distinct_after::<4>(input)
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        Some(n_distinct_after::<14>(input))
    }
}

fn n_distinct_after<const N: usize>(input: &str) -> usize {
    input
        .chars()
        .collect::<Vec<_>>()
        .array_windows::<N>()
        .enumerate()
        .find(|(_, chars)| chars.iter().are_unique())
        .unwrap()
        .0
        + N
}

trait UniqueExt: Iterator {
    fn are_unique(&mut self) -> bool
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        let mut seen = HashSet::with_capacity(self.size_hint().0);
        self.all(|e| seen.insert(e))
    }
}

impl<I: Iterator> UniqueExt for I {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Day6.part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(Day6.part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(Day6.part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(Day6.part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(Day6.part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day6.part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(19));
        assert_eq!(Day6.part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
        assert_eq!(Day6.part2("nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
        assert_eq!(Day6.part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(29));
        assert_eq!(Day6.part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(26));
    }
}
