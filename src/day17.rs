use crate::Solution;
use fxhash::FxBuildHasher;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct Day17;

impl Solution<17> for Day17 {
    type Output = usize;

    fn part1(&self, input: &str) -> Self::Output {
        const TOTAL_ROCK_COUNT: usize = 2022;
        let jets = Direction::parse_jets(input);
        let mut tower = Vec::with_capacity(TOTAL_ROCK_COUNT * 4);

        ROCKS
            .iter()
            .cycle()
            .take(TOTAL_ROCK_COUNT)
            .fold(0, |jet_idx, &rock| {
                drop_rock(&mut tower, rock, &jets, jet_idx)
            });

        tower.len()
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        const TOTAL_ROCK_COUNT: usize = 1_000_000_000_000;
        const INITIAL_CAPACITY: usize = 2048;
        const MIN_HEIGHT: usize = 8;
        let jets = Direction::parse_jets(input);
        let mut tower = Vec::with_capacity(INITIAL_CAPACITY);
        let mut previous_states =
            HashMap::with_capacity_and_hasher(INITIAL_CAPACITY, FxBuildHasher::default());

        let mut cycle_height = 0;
        let mut jet_idx = 0;
        let mut rock_count = 0;

        for (rock_idx, &rock) in ROCKS.iter().enumerate().cycle() {
            if rock_count >= TOTAL_ROCK_COUNT {
                break;
            }
            rock_count += 1;
            jet_idx = drop_rock(&mut tower, rock, &jets, jet_idx);

            if tower.len() < MIN_HEIGHT {
                continue;
            }

            let latest_chunk =
                u64::from_le_bytes(tower[tower.len() - MIN_HEIGHT..].try_into().ok()?);
            let state = (latest_chunk, rock_idx, jet_idx);

            match previous_states.entry(state) {
                Entry::Occupied(entry) => {
                    let (prev_n, prev_height) = entry.get();
                    let cycle_size = rock_count - prev_n;
                    let num_cycles = (TOTAL_ROCK_COUNT - rock_count) / cycle_size;
                    rock_count += cycle_size * num_cycles;
                    cycle_height += (tower.len() - prev_height) * num_cycles;
                    previous_states.clear();
                }
                Entry::Vacant(entry) => {
                    entry.insert((rock_count, tower.len()));
                }
            }
        }

        Some(tower.len() + cycle_height)
    }
}
#[derive(Copy, Clone)]
struct Rock(u32);

const ROCKS: &[Rock; 5] = &[
    Rock(u32::from_le_bytes([
        0b01_1110, 0b00_0000, 0b00_0000, 0b00_0000,
    ])),
    Rock(u32::from_le_bytes([
        0b00_1000, 0b01_1100, 0b00_1000, 0b00_0000,
    ])),
    Rock(u32::from_le_bytes([
        0b01_1100, 0b00_0100, 0b00_0100, 0b00_0000,
    ])),
    Rock(u32::from_le_bytes([
        0b01_0000, 0b01_0000, 0b01_0000, 0b01_0000,
    ])),
    Rock(u32::from_le_bytes([
        0b01_1000, 0b01_1000, 0b00_0000, 0b00_0000,
    ])),
];

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn parse_jets(input: &str) -> Vec<Direction> {
        input
            .chars()
            .filter_map(|c| match c {
                '<' => Some(Direction::Left),
                '>' => Some(Direction::Right),
                _ => None,
            })
            .collect()
    }
}

const WALLS: (u32, u32) = (
    u32::from_le_bytes([0b0100_0000, 0b0100_0000, 0b0100_0000, 0b0100_0000]),
    u32::from_le_bytes([0b0000_0001, 0b0000_0001, 0b0000_0001, 0b0000_0001]),
);

impl Rock {
    fn push(&mut self, direction: &Direction, collision_mask: u32) {
        let pos = match direction {
            Direction::Left if self.0 & WALLS.0 == 0 => self.0 << 1,
            Direction::Right if self.0 & WALLS.1 == 0 => self.0 >> 1,
            _ => return,
        };

        if pos & collision_mask == 0 {
            self.0 = pos;
        }
    }

    fn byte_rows(self) -> impl Iterator<Item = u8> {
        self.0.to_le_bytes().into_iter().take_while(|&b| b != 0)
    }
}

fn get_tower_bytes(tower: &[u8], height: usize) -> u32 {
    if height >= tower.len() {
        0
    } else {
        tower[height..]
            .iter()
            .take(4)
            .rev()
            .fold(0u32, |acc, b| (acc << 8) | u32::from(*b))
    }
}

fn drop_rock(tower: &mut Vec<u8>, mut rock: Rock, jets: &[Direction], mut jet_idx: usize) -> usize {
    let mut insertion_height = tower.len() + 3;

    loop {
        let tower_bytes = get_tower_bytes(tower, insertion_height);
        rock.push(&jets[jet_idx], tower_bytes);
        jet_idx = (jet_idx + 1) % jets.len();

        if insertion_height > tower.len() {
            insertion_height -= 1;
        } else if insertion_height == 0
            || rock.0 & get_tower_bytes(tower, insertion_height - 1) != 0
        {
            insert_rock(tower, rock, insertion_height);
            return jet_idx;
        } else {
            insertion_height -= 1;
        }
    }
}

fn insert_rock(tower: &mut Vec<u8>, rock: Rock, height: usize) {
    rock.byte_rows().fold(height, |h, rock_byte| {
        if h < tower.len() {
            tower[h] |= rock_byte;
        } else {
            tower.push(rock_byte);
        }
        h + 1
    });
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part1() {
        assert_eq!(Day17.part1(TEST_INPUT), 3068);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day17.part2(TEST_INPUT), Some(1_514_285_714_288));
    }
}
