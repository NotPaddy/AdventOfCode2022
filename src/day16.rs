use crate::Solution;
use ndarray::Array3;
use std::cmp::{max, Reverse};
use std::collections::HashMap;
use std::str::FromStr;

const MINUTES: usize = 30;

pub struct Day16;

impl Solution<16> for Day16 {
    type Output = u32;

    fn part1(&self, input: &str) -> Self::Output {
        let mut valves = input
            .lines()
            .map(|l| l.parse::<Valve>().expect("Line to be parseable"))
            .collect::<Vec<_>>();

        let (start_index, flow_valves, flow_state) = optimize_flow(&mut valves, "AA");

        flow_state[(MINUTES - 1, start_index, flow_valves - 1)]
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        let mut valves = input
            .lines()
            .map(|l| l.parse::<Valve>().expect("Line to be parseable"))
            .collect::<Vec<_>>();

        let (start_index, flow_valves, flow_state) = optimize_flow(&mut valves, "AA");

        let mut max_flow = 0;
        for valve_a in 0..flow_valves / 2 {
            let valve_b = flow_valves - 1 - valve_a;
            let own_flow = flow_state[(MINUTES - 5, start_index, valve_a)];
            let elephant_flow = flow_state[(MINUTES - 5, start_index, valve_b)];
            max_flow = max(max_flow, own_flow + elephant_flow);
        }

        Some(max_flow)
    }
}

fn optimize_flow(valves: &mut Vec<Valve>, start_name: &str) -> (usize, usize, Array3<u32>) {
    valves.sort_by_key(|v| Reverse(v.flow_rate));
    let valve_indices = valves
        .iter()
        .enumerate()
        .map(|(i, Valve { name, .. })| (name, i))
        .collect::<HashMap<_, _>>();
    let flow_valve_count = valves.iter().filter(|v| v.flow_rate > 0).count();
    let valve_count = valves.len();

    let tunnel_adjacency = valves
        .iter()
        .map(|valve| (valve, valve_indices[&valve.name]))
        .fold(
            vec![vec![0usize; 0]; valve_count],
            |mut acc, (valve, idx)| {
                acc[idx] = valve
                    .tunnels
                    .iter()
                    .map(|t| valve_indices[t])
                    .collect::<Vec<_>>();
                acc
            },
        );

    let flow = valves
        .iter()
        .map(|valve| (valve, valve_indices[&valve.name]))
        .fold(vec![0u32; valve_count], |mut acc, (valve, idx)| {
            acc[idx] = valve.flow_rate;
            acc
        });

    let start_index = valve_indices[&start_name.to_string()];

    let flow_valve_bits = 1 << flow_valve_count;
    let mut flow_state = Array3::<u32>::zeros([MINUTES, valve_count, flow_valve_bits]);
    for step in 1..MINUTES {
        for valve_idx in 0..valve_count {
            let valve_bit = 1 << valve_idx;
            for m_v in 0..flow_valve_bits {
                let mut max_flow = flow_state[(step, valve_idx, m_v)];
                if valve_bit & m_v != 0 && step >= 1 {
                    let current_flow = flow_state[(step - 1, valve_idx, m_v - valve_bit)];
                    max_flow = max(
                        max_flow,
                        current_flow + flow[valve_idx] as u32 * step as u32,
                    );
                }
                for &tunnel_idx in &tunnel_adjacency[valve_idx] {
                    max_flow = max(max_flow, flow_state[(step - 1, tunnel_idx, m_v)]);
                }
                flow_state[(step, valve_idx, m_v)] = max_flow;
            }
        }
    }
    (start_index, flow_valve_bits, flow_state)
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

impl FromStr for Valve {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::branch::alt;
        use nom::bytes::complete::tag;
        use nom::character::complete::{alpha1, char, space0, space1, u32};
        use nom::combinator::map;
        use nom::error::Error;
        use nom::multi::separated_list0;
        use nom::sequence::{delimited, pair, preceded, separated_pair};
        use nom::{Finish, IResult};

        fn parse_name(input: &str) -> IResult<&str, String> {
            map(alpha1, |n: &str| n.to_string())(input)
        }
        fn parse_valve(input: &str) -> IResult<&str, (String, u32)> {
            pair(
                preceded(tag("Valve"), delimited(space1, parse_name, space1)),
                preceded(tag("has flow rate="), u32),
            )(input)
        }
        fn parse_name_list(input: &str) -> IResult<&str, Vec<String>> {
            separated_list0(delimited(space0, char(','), space1), parse_name)(input)
        }
        fn parse_targets(input: &str) -> IResult<&str, Vec<String>> {
            preceded(
                alt((tag("tunnels lead to valves"), tag("tunnel leads to valve"))),
                preceded(space1, parse_name_list),
            )(input)
        }
        fn parse_valve_line(input: &str) -> IResult<&str, Valve> {
            map(
                separated_pair(
                    parse_valve,
                    delimited(space0, char(';'), space0),
                    parse_targets,
                ),
                |((name, flow_rate), targets)| Valve {
                    name,
                    flow_rate,
                    tunnels: targets,
                },
            )(input)
        }

        match parse_valve_line(s).finish() {
            Ok((_, valve)) => Ok(valve),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II
    "};

    #[test]
    fn test_part1() {
        assert_eq!(Day16.part1(TEST_INPUT), 1651)
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day16.part2(TEST_INPUT), Some(1707))
    }
}
