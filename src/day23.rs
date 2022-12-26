use crate::Solution;
use fxhash::FxHashSet;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::hash::BuildHasher;
use std::ops::Add;

pub struct Day23;

impl Solution<23> for Day23 {
    type Output = usize;

    fn part1(&self, input: &str) -> Self::Output {
        let elves = parse_initial_state(input);
        let elves = (0..ROUNDS).fold(elves, |elves, round| step(&elves, round));
        let bounds = bounding_box(&elves);
        bounds.0 * bounds.1 - elves.len()
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        let mut elves = parse_initial_state(input);
        for round in 0.. {
            let next = step(&elves, round);
            if next == elves {
                return Some(round + 1);
            }
            elves = next
        }
        None
    }
}

const ROUNDS: usize = 10;

fn step<S>(elves: &HashSet<Position, S>, round: usize) -> HashSet<Position, S>
where
    S: BuildHasher + Default,
{
    let mut next_step = HashSet::with_capacity_and_hasher(elves.capacity(), S::default());
    for &elf in elves.iter() {
        let next_pos = find_destination(&elves, elf, round);
        if next_pos == elf {
            next_step.insert(elf);
        } else if !next_step.insert(next_pos) {
            next_step.remove(&next_pos);
            next_step.insert(elf);
            next_step.insert(Position(next_pos.0 * 2 - elf.0, next_pos.1 * 2 - elf.1));
        }
    }
    next_step
}

fn find_destination<S>(elves: &HashSet<Position, S>, elf: Position, round: usize) -> Position
where
    S: BuildHasher,
{
    if elf.neighbors().any(|n| elves.contains(&n)) {
        for r in round..round + 4 {
            let positions = MOVE_OFFSETS[r % 4].map(|o| o + elf);
            if !positions.iter().any(|p| elves.contains(p)) {
                return positions[1];
            }
        }
    }
    elf
}

fn bounding_box<S>(elves: &HashSet<Position, S>) -> (usize, usize) {
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (i32::MAX, i32::MIN, i32::MAX, i32::MIN);

    for elf in elves.iter() {
        min_x = min(min_x, elf.0);
        max_x = max(max_x, elf.0);
        min_y = min(min_y, elf.1);
        max_y = max(max_y, elf.1);
    }

    ((max_x - min_x + 1) as usize, (max_y - min_y + 1) as usize)
}

fn parse_initial_state(input: &str) -> FxHashSet<Position> {
    input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| line.chars().enumerate().map(move |(y, c)| (x, y, c)))
        .filter(|(_, _, c)| *c == '#')
        .map(|(y, x, _)| Position(x as i32, y as i32))
        .collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position(i32, i32);

impl Position {
    fn neighbors(&self) -> impl Iterator<Item = Position> {
        NEIGHBOR_OFFSETS
            .map(|o| Position(self.0 + o.0, self.1 + o.1))
            .into_iter()
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

const NEIGHBOR_OFFSETS: [Position; 8] = [
    Position(1, 1),
    Position(1, 0),
    Position(1, -1),
    Position(0, 1),
    Position(0, -1),
    Position(-1, 1),
    Position(-1, 0),
    Position(-1, -1),
];

const MOVE_OFFSETS: [[Position; 3]; 4] = [
    [Position(1, -1), Position(0, -1), Position(-1, -1)],
    [Position(1, 1), Position(0, 1), Position(-1, 1)],
    [Position(-1, 1), Position(-1, 0), Position(-1, -1)],
    [Position(1, 1), Position(1, 0), Position(1, -1)],
];

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        ..............
        ..............
        .......#......
        .....###.#....
        ...#...#.#....
        ....#...##....
        ...#.###......
        ...##.#.##....
        ....#..#......
        ..............
        ..............
        ..............
    "};

    #[test]
    fn test_part1() {
        assert_eq!(Day23.part1(TEST_INPUT), 110)
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day23.part2(TEST_INPUT), Some(20))
    }
}
