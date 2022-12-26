use crate::Solution;
use std::ops::RangeInclusive;

pub struct Day4;

impl Solution<4> for Day4 {
    type Output = usize;

    fn part1(&self, input: &str) -> Self::Output {
        filter_count_ranges(input, |l, r| l.fully_contains(r) || r.fully_contains(l))
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        Some(filter_count_ranges(input, RangeCheck::overlaps))
    }
}

fn filter_count_ranges<P>(input: &str, predicate: P) -> usize
where
    P: Fn(&RangeInclusive<u8>, &RangeInclusive<u8>) -> bool,
{
    input
        .lines()
        .filter_map(|l| l.split_once(','))
        .filter_map(|(l, r)| Some((parse_range(l)?, parse_range(r)?)))
        .filter(|(l, r)| predicate(l, r))
        .count()
}

fn parse_range(elf: &str) -> Option<RangeInclusive<u8>> {
    let (begin, end) = elf.split_once('-')?;
    let begin = begin.parse::<u8>().ok()?;
    let end = end.parse::<u8>().ok()?;
    Some(begin..=end)
}

trait RangeCheck {
    fn fully_contains(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

impl<T: Ord> RangeCheck for RangeInclusive<T> {
    fn fully_contains(&self, other: &Self) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start() <= other.end() && self.end() >= other.start()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "};

    #[test]
    fn test_part1() {
        assert_eq!(Day4.part1(TEST_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day4.part2(TEST_INPUT), Some(4));
    }
}
