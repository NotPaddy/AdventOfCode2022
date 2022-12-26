use crate::Solution;
use std::iter::repeat_with;

enum Crane {
    CrateMover9000,
    CrateMover9001,
}

pub struct Day5;

impl Solution<5> for Day5 {
    type Output = String;

    fn part1(&self, input: &str) -> Self::Output {
        let (stacks, moves) = input.split_once("\n\n").unwrap();
        let mut stacks = parse_stacks(stacks);
        let moves = Move::parse_moves(moves);

        for m in moves {
            m.apply_to(&mut stacks, &Crane::CrateMover9000);
        }

        stacks.iter().filter_map(|s| s.last()).collect()
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        let (stacks, moves) = input.split_once("\n\n").unwrap();
        let mut stacks = parse_stacks(stacks);
        let moves = Move::parse_moves(moves);

        for m in moves {
            m.apply_to(&mut stacks, &Crane::CrateMover9001);
        }

        Some(stacks.iter().filter_map(|s| s.last()).collect())
    }
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut lines = input.lines().rev();
    let count = lines.next().unwrap().split_ascii_whitespace().count();
    let mut stacks: Vec<Vec<char>> = repeat_with(|| Vec::<char>::with_capacity(count))
        .take(count)
        .collect();

    for line in lines {
        let crates = line
            .chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, c)| !c.is_whitespace());

        for (idx, c) in crates {
            stacks[idx].push(c);
        }
    }

    stacks
}

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn parse_moves(input: &str) -> impl Iterator<Item = Move> + '_ {
        input.lines().filter_map(|l: &str| {
            let (count, rest) = l.strip_prefix("move ")?.split_once(" from ")?;
            let (from, to) = rest.split_once(" to ")?;
            Some(Move {
                count: count.parse().ok()?,
                from: from.parse().ok()?,
                to: to.parse().ok()?,
            })
        })
    }

    fn apply_to(&self, stacks: &mut [Vec<char>], crane: &Crane) {
        let from = stacks.get_mut(self.from - 1).unwrap();
        let mut crates = from.split_off(from.len() - self.count);
        match crane {
            Crane::CrateMover9000 => crates.reverse(),
            Crane::CrateMover9001 => {}
        }
        stacks.get_mut(self.to - 1).unwrap().append(&mut crates);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
            [D]
        [N] [C]
        [Z] [M] [P]
         1   2   3

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    "};

    #[test]
    fn test_part1() {
        assert_eq!(Day5.part1(TEST_INPUT), "CMZ");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day5.part2(TEST_INPUT), Some("MCD".to_string()));
    }
}
