use aoc_2022::{Runner, Solution};

fn main() {
    Runner::new(include_str!("../inputs/day01.txt")).run(&Day1)
}

struct Day1;

impl Solution<1> for Day1 {
    type Output = usize;

    fn part1(&self, _input: &str) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Day1.part1(""), 0)
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day1.part2(""), Some(0))
    }
}
