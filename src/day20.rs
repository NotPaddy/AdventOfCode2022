use crate::Solution;
use std::collections::VecDeque;

pub struct Day20;

impl Solution<20> for Day20 {
    type Output = i64;

    fn part1(&self, input: &str) -> Self::Output {
        let input: Vec<i64> = input.lines().filter_map(|l| l.parse().ok()).collect();
        decrypt(&input, 1, 1).unwrap()
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        let input: Vec<i64> = input.lines().filter_map(|l| l.parse().ok()).collect();
        decrypt(&input, 811_589_153, 10)
    }
}

struct Entry {
    value: i64,
    index: usize,
}

fn decrypt(input: &[i64], key: i64, iterations: usize) -> Option<i64> {
    let mut entries = input
        .iter()
        .map(|value| value * key)
        .enumerate()
        .map(|(index, value)| Entry { value, index })
        .collect::<VecDeque<_>>();

    for _ in 0..iterations {
        for v_idx in 0..entries.len() {
            let idx = entries
                .iter()
                .position(|&Entry { index, .. }| v_idx == index)
                .unwrap();
            entries.rotate_left(idx);
            let entry = entries.pop_front()?;
            let d = entry.value.rem_euclid(entries.len() as i64) as usize;
            entries.rotate_left(d);
            entries.push_front(entry);
        }
    }
    let zero_idx = entries.iter().position(|&Entry { value, .. }| value == 0)?;
    let sum = (1..=3)
        .map(|i| entries[(zero_idx + 1000 * i) % entries.len()].value)
        .sum();
    Some(sum)
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        1
        2
        -3
        3
        -2
        0
        4
    "};

    #[test]
    fn test_part1() {
        assert_eq!(Day20.part1(TEST_INPUT), 3)
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day20.part2(TEST_INPUT), Some(1_623_178_306))
    }
}
