use crate::Solution;

pub struct Day3;

impl Solution<3> for Day3 {
    type Output = u32;

    fn part1(&self, input: &str) -> Self::Output {
        input
            .lines()
            .map(|l| l.split_at(l.len() / 2))
            .filter_map(|(c1, c2)| c1.chars().find(|c: &char| c2.contains(&c.to_string())))
            .filter_map(priority)
            .sum()
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        let sum = input
            .lines()
            .array_chunks::<3>()
            .filter_map(|chunks: [&str; 3]| {
                chunks[0].chars().find(|c| {
                    chunks[1].contains(&c.to_string()) && chunks[2].contains(&c.to_string())
                })
            })
            .filter_map(priority)
            .sum();

        Some(sum)
    }
}

fn priority(item: char) -> Option<u32> {
    if !item.is_ascii_alphabetic() {
        return None;
    }
    Some(u32::from(match item {
        'a'..='z' => item as u8 - b'a' + 1,
        'A'..='Z' => item as u8 - b'A' + 27,
        _ => unreachable!(),
    }))
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
        "};

    #[test]
    fn test_part1() {
        assert_eq!(Day3.part1(TEST_INPUT), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day3.part2(TEST_INPUT), Some(70));
    }
}
