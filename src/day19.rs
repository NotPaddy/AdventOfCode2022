use crate::Solution;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::multispace0;
use nom::character::complete::u32 as n_u32;
use nom::character::complete::u8 as n_u8;
use nom::combinator::{iterator, map};
use nom::error::ParseError;
use nom::sequence::{delimited, terminated, tuple};
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use nom::{AsChar, InputTakeAtPosition, Parser};
use rayon::prelude::*;
use std::array;
use std::cmp::max;

pub struct Day19;

impl Solution<19> for Day19 {
    type Output = u32;

    fn part1(&self, input: &str) -> Self::Output {
        iterator(input, terminated(parse_blueprint, multispace0))
            .collect::<Vec<_>>()
            .par_iter()
            .map(|bp| bp.quality_level(24))
            .sum()
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        Some(
            iterator(input, terminated(parse_blueprint, multispace0))
                .take(3)
                .collect::<Vec<_>>()
                .par_iter()
                .map(|bp| bp.maximum_geode_count(32))
                .product(),
        )
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u8,
    robot_costs: [RobotCost; 4],
}

type RobotCost = [u32; 4];

struct State {
    inventory: [u32; 4],
    robots: [u32; 4],
    minute: u32,
}

impl State {
    fn get_wait_time(&self, material_costs: &RobotCost, max_time: u32) -> u32 {
        (0..3)
            .filter(|&material| material_costs[material] > 0)
            .map(|material| {
                if material_costs[material] <= self.inventory[material] {
                    0
                } else if self.robots[material] == 0 {
                    max_time + 1
                } else {
                    (material_costs[material] - self.inventory[material] + self.robots[material]
                        - 1)
                        / self.robots[material]
                }
            })
            .max()
            .unwrap_or(0)
    }

    fn step(
        &self,
        material: usize,
        material_costs: &RobotCost,
        wait_time: u32,
    ) -> ([u32; 4], [u32; 4]) {
        let inventory = array::from_fn(|m| {
            self.inventory[m] + self.robots[m] * (wait_time + 1) - material_costs[m]
        });
        let robots = array::from_fn(|m| self.robots[m] + u32::from(m == material));
        (inventory, robots)
    }
}

impl Blueprint {
    fn quality_level(&self, minutes: u32) -> u32 {
        self.maximum_geode_count(minutes) * u32::from(self.id)
    }

    fn maximum_geode_count(&self, minutes: u32) -> u32 {
        let robot_caps = array::from_fn(|material| {
            self.robot_costs
                .iter()
                .map(|cost| cost[material])
                .max()
                .filter(|&cost| cost > 0)
                .unwrap_or(u32::MAX)
        });

        let mut max_geodes = 0;
        self.recurse(
            &State {
                inventory: [0; 4],
                robots: [1, 0, 0, 0],
                minute: 0,
            },
            minutes,
            &robot_caps,
            &mut max_geodes,
        );
        max_geodes
    }

    fn recurse(&self, state: &State, minutes: u32, robot_caps: &[u32; 4], max_geodes: &mut u32) {
        for robot in 0..4 {
            if state.robots[robot] == robot_caps[robot] {
                continue;
            }
            let costs = &self.robot_costs[robot];

            let wait_time = state.get_wait_time(costs, minutes);
            let next_time = state.minute + wait_time + 1;
            if next_time >= minutes {
                continue;
            }
            let (inventory, robots) = state.step(robot, costs, wait_time);
            let remaining_time = minutes - next_time;
            let upper_bound = inventory[3]
                + remaining_time * robots[3]
                + ((remaining_time - 1) * remaining_time) / 2;
            if upper_bound < *max_geodes {
                continue;
            }

            self.recurse(
                &State {
                    inventory,
                    robots,
                    minute: next_time,
                },
                minutes,
                robot_caps,
                max_geodes,
            );
        }
        *max_geodes = max(
            *max_geodes,
            state.inventory[3] + state.robots[3] * (minutes - state.minute),
        );
    }
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    map(
        tuple((
            delimited(tag("Blueprint "), n_u8, terminated(char(':'), multispace0)),
            parse_ore_bot,
            parse_clay_bot,
            parse_obsidian_bot,
            parse_geode_bot,
        )),
        |(id, ore, clay, obsidian, geode)| Blueprint {
            id,
            robot_costs: [ore, clay, obsidian, geode],
        },
    )(input)
}

fn whitespace<I, F, O, E: ParseError<I>>(parser: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: Parser<I, O, E>,
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    delimited(multispace0, parser, multispace0)
}

fn parse_ore_bot(input: &str) -> IResult<&str, RobotCost> {
    map(
        whitespace(preceded(
            tag("Each ore robot costs "),
            terminated(n_u32, tag(" ore.")),
        )),
        |o| [o, 0, 0, 0],
    )(input)
}

fn parse_clay_bot(input: &str) -> IResult<&str, RobotCost> {
    map(
        whitespace(preceded(
            tag("Each clay robot costs "),
            terminated(n_u32, tag(" ore.")),
        )),
        |o| [o, 0, 0, 0],
    )(input)
}

fn parse_obsidian_bot(input: &str) -> IResult<&str, RobotCost> {
    map(
        whitespace(preceded(
            tag("Each obsidian robot costs "),
            separated_pair(
                terminated(n_u32, tag(" ore")),
                whitespace(tag("and")),
                terminated(n_u32, tag(" clay.")),
            ),
        )),
        |(o, c)| [o, c, 0, 0],
    )(input)
}

fn parse_geode_bot(input: &str) -> IResult<&str, RobotCost> {
    map(
        whitespace(preceded(
            tag("Each geode robot costs "),
            separated_pair(
                terminated(n_u32, tag(" ore")),
                whitespace(tag("and")),
                terminated(n_u32, tag(" obsidian.")),
            ),
        )),
        |(o, obs)| [o, 0, obs, 0],
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use std::assert_matches::assert_matches;

    const TEST_INPUT: &str = indoc! {"
        Blueprint 1:
          Each ore robot costs 4 ore.
          Each clay robot costs 2 ore.
          Each obsidian robot costs 3 ore and 14 clay.
          Each geode robot costs 2 ore and 7 obsidian.

        Blueprint 2:
          Each ore robot costs 2 ore.
          Each clay robot costs 3 ore.
          Each obsidian robot costs 3 ore and 8 clay.
          Each geode robot costs 3 ore and 12 obsidian.
    "};

    #[test]
    fn test_parse() {
        assert_matches!(
            parse_blueprint(TEST_INPUT.split_once("\n\n").unwrap().0),
            Ok(_)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(Day19.part1(TEST_INPUT), 33);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day19.part2(TEST_INPUT), Some(56 * 62));
    }
}
