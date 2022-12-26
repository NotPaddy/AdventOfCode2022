use crate::Solution;

pub struct Day1;

impl Solution<1> for Day1 {
    type Output = u32;

    fn part1(&self, input: &str) -> Self::Output {
        Self::get_elf_calories(input).max().unwrap()
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        let mut elves: Vec<u32> = Self::get_elf_calories(input).collect();
        elves.sort_unstable();

        let sum = elves.iter().rev().take(3).sum();
        Some(sum)
    }
}

impl Day1 {
    fn get_elf_calories(input: &str) -> impl Iterator<Item = u32> + '_ {
        input
            .split("\n\n")
            .map(|elf| elf.lines().map(|l| l.parse::<u32>().unwrap_or(0)).sum())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
        "};

    #[test]
    fn test_part1() {
        assert_eq!(Day1.part1(TEST_INPUT), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day1.part2(TEST_INPUT), Some(45000));
    }
}
