use crate::Solution;
use fxhash::FxBuildHasher;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day18;

impl Solution<18> for Day18 {
    type Output = usize;

    fn part1(&self, input: &str) -> Self::Output {
        let mut cubes = HashSet::with_hasher(FxBuildHasher::default());
        cubes.extend(parse_input(input));

        cubes
            .iter()
            .flat_map(|p| neighbors(p))
            .filter(|n| !cubes.contains(n))
            .count()
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        let mut cubes = HashSet::with_hasher(FxBuildHasher::default());
        cubes.extend(parse_input(input));
        let mut seen = HashSet::with_hasher(FxBuildHasher::default());
        let mut stack = vec![(0, 0, 0)];

        let max_bounds = cubes.iter().flat_map(|&(x, y, z)| [x, y, z]).max().unwrap();
        let range = -1..=max_bounds + 1;

        while let Some(pos) = stack.pop() {
            for n in neighbors(&pos) {
                if !cubes.contains(&n)
                    && !seen.contains(&n)
                    && [n.0, n.1, n.2].iter().all(|&i| range.contains(&i))
                {
                    seen.insert(n);
                    stack.push(n);
                }
            }
        }

        let count = cubes
            .iter()
            .flat_map(|p| neighbors(p))
            .filter(|n| seen.contains(n))
            .count();
        Some(count)
    }
}

type Position = (i16, i16, i16);

fn neighbors(&(x, y, z): &Position) -> [Position; 6] {
    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

fn parse_input(input: &str) -> impl Iterator<Item = Position> + '_ {
    input
        .lines()
        .filter_map(|l| l.split(',').filter_map(|p| p.parse().ok()).collect_tuple())
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5
    "};

    #[test]
    fn test_part1() {
        assert_eq!(Day18.part1(TEST_INPUT), 64)
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day18.part2(TEST_INPUT), Some(58))
    }
}
