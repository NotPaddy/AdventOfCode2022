use crate::Solution;
use std::collections::HashSet;

pub struct Day9;

impl Solution<9> for Day9 {
    type Output = usize;

    fn part1(&self, input: &str) -> Self::Output {
        get_visited_count(input, 2)
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        Some(get_visited_count(input, 10))
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_visited_count(input: &str, length: usize) -> usize {
    let mut knots: Vec<(i32, i32)> = vec![(0, 0); length];
    let mut visited = HashSet::from([knots[0]]);

    for (dir, steps) in input.lines().filter_map(get_move) {
        for _ in 0..steps {
            match dir {
                Direction::Up => knots[0].1 += 1,
                Direction::Down => knots[0].1 -= 1,
                Direction::Left => knots[0].0 -= 1,
                Direction::Right => knots[0].0 += 1,
            }

            for i in 1..length {
                let prev = knots[i - 1];
                let next = &mut knots[i];

                if (prev.0 - next.0).abs() > 1 || (prev.1 - next.1).abs() > 1 {
                    next.0 += (prev.0 - next.0).signum();
                    next.1 += (prev.1 - next.1).signum();
                }
            }

            visited.insert(*knots.last().unwrap());
        }
    }

    visited.len()
}

fn get_move(line: &str) -> Option<(Direction, usize)> {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    if let [dir, length] = parts[..] {
        let dir = match dir {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return None,
        };
        Some((dir, length.parse().ok()?))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day9.part1(indoc! {"
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2
        "}),
            13
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day9.part2(indoc! {"
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
        "}),
            Some(36)
        );
    }
}
