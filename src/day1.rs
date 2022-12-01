use aoc_2022::{Runner, Solution};

fn main() {
    Runner::new(include_str!("../inputs/day01.txt")).run(&Day1)
}

struct Day1;

impl Solution<1> for Day1 {
    type Output = u32;

    fn part1(&self, input: &str) -> Self::Output {
        let elves = Self::get_elf_calories(input);

        *elves.iter().max().unwrap()
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        let mut elves = Self::get_elf_calories(input);
        elves.sort();

        let sum = elves.iter().rev().take(3).sum();
        Some(sum)
    }
}

impl Day1 {
    fn get_elf_calories(input: &str) -> Vec<u32> {
        let mut elves: Vec<u32> = Vec::from([0]);
        for line in input.lines() {
            match line.parse::<u32>() {
                Ok(cal) => {
                    *elves.last_mut().unwrap() += cal;
                }
                Err(_) => elves.push(0),
            }
        }
        elves
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
        assert_eq!(Day1.part1(TEST_INPUT), 24000)
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day1.part2(TEST_INPUT), Some(45000))
    }
}
