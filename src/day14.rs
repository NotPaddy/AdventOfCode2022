use crate::Solution;
use fxhash::FxBuildHasher;
use nom::combinator::iterator;
use nom::IResult;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::hash::BuildHasher;

pub struct Day14;

impl Solution<14> for Day14 {
    type Output = u32;

    fn part1(&self, input: &str) -> Self::Output {
        let mut input_parser = iterator(input, coordinate_line);
        let mut world: HashSet<Point, _> = HashSet::with_hasher(FxBuildHasher::default());
        input_parser.for_each(|l| draw_line(&mut world, &l));

        let mut units = 0;
        let mut path = vec![Point { x: 500, y: 0 }];
        let bottom_edge = world.iter().map(|p| p.y).max().unwrap();
        while simulate_sand(&mut world, &mut path, bottom_edge, false) {
            units += 1;
        }
        units
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        let mut input_parser = iterator(input, coordinate_line);
        let mut world: HashSet<Point, _> = HashSet::with_hasher(FxBuildHasher::default());
        input_parser.for_each(|l| draw_line(&mut world, &l));

        let mut units = 0;
        let mut path = vec![Point { x: 500, y: 0 }];
        let bottom_edge = world.iter().map(|p| p.y).max().unwrap();
        while simulate_sand(&mut world, &mut path, bottom_edge, true) {
            units += 1;
        }
        Some(units)
    }
}

fn draw_line<S>(world: &mut HashSet<Point, S>, line: &[Point])
where
    S: BuildHasher,
{
    for [from, to] in line.array_windows::<2>() {
        world.insert(*from);
        if from.x == to.x {
            let start = min(from.y, to.y);
            let end = max(from.y, to.y);
            for y in start..=end {
                world.insert(Point { x: from.x, y });
            }
        }

        if from.y == to.y {
            let start = min(from.x, to.x);
            let end = max(from.x, to.x);
            for x in start..=end {
                world.insert(Point { x, y: from.y });
            }
        }
    }
}

fn simulate_sand<S>(
    world: &mut HashSet<Point, S>,
    path: &mut Vec<Point>,
    lowest_y: u32,
    floor: bool,
) -> bool
where
    S: BuildHasher,
{
    if path.is_empty() {
        return false;
    }
    loop {
        let position = *path.last().unwrap();
        if !floor && position.y == lowest_y {
            return false;
        } else if floor && position.y == lowest_y + 2 {
            world.insert(position);
            path.pop();
            continue;
        }
        let next_pos = Point {
            x: position.x,
            y: position.y + 1,
        };
        if !world.contains(&next_pos) {
            path.push(next_pos);
            continue;
        }

        let next_pos = Point {
            x: position.x - 1,
            y: position.y + 1,
        };
        if !world.contains(&next_pos) {
            path.push(next_pos);
            continue;
        }

        let next_pos = Point {
            x: position.x + 1,
            y: position.y + 1,
        };
        if !world.contains(&next_pos) {
            path.push(next_pos);
            continue;
        }

        world.insert(position);
        path.pop();
        return true;
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: u32,
    y: u32,
}

fn coordinate_line(input: &str) -> IResult<&str, Vec<Point>> {
    use nom::bytes::complete::tag;
    use nom::character::complete::line_ending;
    use nom::character::complete::space0;
    use nom::combinator::opt;
    use nom::multi::separated_list1;
    use nom::sequence::{delimited, terminated};

    terminated(
        separated_list1(delimited(space0, tag("->"), space0), coordinate_pair),
        opt(line_ending),
    )(input)
}

fn coordinate_pair(input: &str) -> IResult<&str, Point> {
    use nom::character::complete::{char, u32};
    use nom::combinator::map;
    use nom::sequence::{preceded, tuple};

    map(tuple((u32, preceded(char(','), u32))), |(x, y)| Point {
        x,
        y,
    })(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use std::assert_matches::assert_matches;

    const TEST_INPUT: &str = indoc! {"
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
    "};

    #[test]
    fn test_parser() {
        assert_eq!(
            coordinate_line("498,4 -> 498,6"),
            Ok(("", vec![Point { x: 498, y: 4 }, Point { x: 498, y: 6 }]))
        );

        assert_matches!(coordinate_line(""), Err(_));
    }

    #[test]
    fn test_part1() {
        assert_eq!(Day14.part1(TEST_INPUT), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day14.part2(TEST_INPUT), Some(93));
    }
}
