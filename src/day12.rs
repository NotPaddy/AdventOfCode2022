use crate::Solution;
use std::collections::VecDeque;
use std::str::FromStr;

pub struct Day12;

impl Solution<12> for Day12 {
    type Output = u32;

    fn part1(&self, input: &str) -> Self::Output {
        shortest_path(input, b'S').unwrap_or_default()
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        shortest_path(input, b'a')
    }
}

fn shortest_path(input: &str, start_at: u8) -> Option<u32> {
    let heightmap = Heightmap::from_str(input).ok()?;
    if start_at == b'S' {
        heightmap.bfs(heightmap.start, heightmap.end)
    } else {
        let start_at = start_at - b'a';
        heightmap
            .grid
            .iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &height)| height == start_at)
                    .map(move |(y, _)| (x, y))
            })
            .filter_map(|start| heightmap.bfs(start, heightmap.end))
            .min()
    }
}

struct Heightmap {
    grid: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

#[allow(clippy::cast_possible_wrap)]
impl Heightmap {
    fn bfs(&self, start: (usize, usize), end: (usize, usize)) -> Option<u32> {
        let mut queue = VecDeque::from([(start, 0)]);
        let mut seen = vec![vec![false; self.grid[0].len()]; self.grid.len()];
        while let Some((position, distance)) = queue.pop_front() {
            if position == end {
                return Some(distance);
            }
            let (x, y) = position;
            MOVE_OFFSETS
                .iter()
                .filter_map(|(dx, dy)| {
                    let x_n: usize = (x as isize + dx).try_into().ok()?;
                    let y_n: usize = (y as isize + dy).try_into().ok()?;
                    if x_n < self.grid.len() && y_n < self.grid[0].len() {
                        Some((x_n, y_n))
                    } else {
                        None
                    }
                })
                .filter(|&(x_n, y_n)| self.grid[x_n][y_n] <= self.grid[x][y] + 1)
                .for_each(|(x_n, y_n)| {
                    if !seen[x_n][y_n] {
                        seen[x_n][y_n] = true;
                        queue.push_back(((x_n, y_n), distance + 1));
                    }
                });
        }

        None
    }
}

const MOVE_OFFSETS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl FromStr for Heightmap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = (0usize, 0usize);
        let mut end = (0usize, 0usize);
        let grid = s
            .lines()
            .enumerate()
            .map(|(row, l)| {
                l.bytes()
                    .enumerate()
                    .map(|(col, c)| match c {
                        b'S' => {
                            start = (row, col);
                            0
                        }
                        b'E' => {
                            end = (row, col);
                            25
                        }
                        _ => c - b'a',
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(Heightmap { grid, start, end })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    "};

    #[test]
    fn test_part1() {
        assert_eq!(Day12.part1(TEST_INPUT), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day12.part2(TEST_INPUT), Some(29));
    }
}
