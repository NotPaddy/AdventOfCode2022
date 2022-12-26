use crate::Solution;
use std::str::FromStr;

pub struct Day24;

impl Solution<24> for Day24 {
    type Output = usize;

    fn part1(&self, input: &str) -> Self::Output {
        let mut map = input.parse::<Map>().unwrap();
        let mut minutes = 0;
        while !map.is_at_goal() {
            map.step();
            minutes += 1;
        }
        minutes
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        let mut map = input.parse::<Map>().ok()?;
        let mut minutes = 0;
        while !map.is_at_goal() {
            map.step();
            minutes += 1;
        }
        map.reset_to_goal();
        while !map.is_at_start() {
            map.step();
            minutes += 1;
        }
        map.reset_to_start();
        while !map.is_at_goal() {
            map.step();
            minutes += 1;
        }
        Some(minutes)
    }
}

type RowMask = u128;

#[derive(Default)]
struct Blizzards {
    north: Vec<RowMask>,
    south: Vec<RowMask>,
    west: Vec<RowMask>,
    east: Vec<RowMask>,
}

impl Blizzards {
    fn move_north(&mut self, height: usize) {
        self.north[1..(height - 1)].rotate_left(1);
    }

    fn move_south(&mut self, height: usize) {
        self.south[1..(height - 1)].rotate_right(1);
    }

    fn move_west(&mut self, walls: &[RowMask]) {
        for (row, wind) in self.west.iter_mut().enumerate() {
            *wind >>= 1;
            if *wind & walls[row] != 0 {
                *wind |= walls[row] >> 1
            }
        }
    }

    fn move_east(&mut self, walls: &[RowMask]) {
        for (row, wind) in self.east.iter_mut().enumerate() {
            *wind <<= 1;
            if *wind & walls[row] != 0 {
                *wind |= 1 << 1
            }
        }
    }

    fn combined(&self, row: usize) -> RowMask {
        self.north[row] | self.south[row] | self.west[row] | self.east[row]
    }
}

struct Map {
    walls: Vec<RowMask>,
    blizzards: Blizzards,
    width: usize,
    height: usize,
    positions: Vec<RowMask>,
}

impl Map {
    fn step(&mut self) {
        self.blizzards.move_north(self.height);
        self.blizzards.move_south(self.height);
        self.blizzards.move_west(&self.walls);
        self.blizzards.move_east(&self.walls);

        (0..self.height).fold(0, |previous_row, row| {
            let current = self.positions[row];
            self.explore_adjacent(&previous_row, row);
            self.cull_elves(row);
            current
        });
    }

    fn explore_adjacent(&mut self, previous_row: &RowMask, row: usize) {
        self.positions[row] |=
            previous_row | (self.positions[row] << 1) | (self.positions[row] >> 1);
        if row < self.height - 1 {
            self.positions[row] |= self.positions[row + 1];
        }
    }

    fn cull_elves(&mut self, row: usize) {
        let blocked = self.walls[row] | self.blizzards.combined(row);
        self.positions[row] &= !blocked;
    }

    fn is_at_start(&self) -> bool {
        self.positions[0] & (1 << 1) != 0
    }

    fn is_at_goal(&self) -> bool {
        let goal = 1 << (self.width - 2);
        self.positions[self.height - 1] & goal != 0
    }

    fn reset_to_start(&mut self) {
        self.positions.fill(0);
        self.positions[0] |= 1 << 1;
    }

    fn reset_to_goal(&mut self) {
        self.positions.fill(0);
        self.positions[self.height - 1] |= 1 << (self.width - 2);
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut walls: Vec<RowMask> = vec![];
        let mut blizzards = Blizzards::default();
        let lines = s.lines().collect::<Vec<_>>();

        for line in lines.iter() {
            assert!(line.len() <= 128, "input shouldn't be wider than 128");

            let [wall, north, south, west, east] =
                line.chars()
                    .enumerate()
                    .fold([0u128; 5], |mut acc, (col, c)| {
                        match c {
                            '#' => acc[0] |= 1 << col,
                            '^' => acc[1] |= 1 << col,
                            'v' => acc[2] |= 1 << col,
                            '<' => acc[3] |= 1 << col,
                            '>' => acc[4] |= 1 << col,
                            _ => (),
                        }
                        acc
                    });

            walls.push(wall);
            blizzards.north.push(north);
            blizzards.south.push(south);
            blizzards.west.push(west);
            blizzards.east.push(east);
        }

        let mut positions = vec![0; lines.len()];
        positions[0] = 1 << 1;

        Ok(Map {
            blizzards,
            walls,
            width: lines[0].len(),
            height: lines.len(),
            positions,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        #.######
        #>>.<^<#
        #.<..<<#
        #>v.><>#
        #<^v^^>#
        ######.#
    "};

    #[test]
    fn test_part1() {
        assert_eq!(Day24.part1(TEST_INPUT), 18)
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day24.part2(TEST_INPUT), Some(54))
    }
}
