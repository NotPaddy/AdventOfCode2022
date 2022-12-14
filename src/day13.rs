use crate::Solution;
use std::cmp::Ordering;
use std::slice;
use std::str::FromStr;

pub struct Day13;

impl Solution<13> for Day13 {
    type Output = usize;

    fn part1(&self, input: &str) -> Self::Output {
        input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.parse::<Packet>().expect("Packet can be parsed"))
            .array_chunks::<2>()
            .enumerate()
            .filter(|(_, [a, b])| a <= b)
            .map(|(idx, _)| idx + 1)
            .sum()
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        let dividers = vec![
            "[[2]]".parse::<Packet>().expect("Divider must be valid"),
            "[[6]]".parse::<Packet>().expect("Divider must be valid"),
        ];
        let mut packets = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.parse::<Packet>().expect("Packe can be parsed"))
            .collect::<Vec<_>>();

        packets.push("[[2]]".parse::<Packet>().expect("Divider must be valid"));
        packets.push("[[6]]".parse::<Packet>().expect("Divider must be valid"));
        packets.sort();

        Some(
            packets
                .iter()
                .enumerate()
                .filter(|(_, p)| dividers.contains(p))
                .map(|(idx, _)| idx + 1)
                .product(),
        )
    }
}

#[derive(Eq, PartialEq, Clone)]
pub enum Packet {
    Integer(u8),
    List(Vec<Self>),
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::branch::alt;
        use nom::character::complete::{char, u8};
        use nom::combinator::map;
        use nom::multi::separated_list0;
        use nom::sequence::delimited;
        use nom::{Finish, IResult};

        fn parse_packet(input: &str) -> IResult<&str, Packet> {
            alt((map(u8, Packet::Integer), parse_enclosed_list))(input)
        }

        fn parse_list(input: &str) -> IResult<&str, Packet> {
            map(separated_list0(char(','), parse_packet), Packet::List)(input)
        }

        fn parse_enclosed_list(input: &str) -> IResult<&str, Packet> {
            delimited(char('['), parse_list, char(']'))(input)
        }

        parse_enclosed_list(s).finish().map_err(|_| ()).map(|r| r.1)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Integer(s), Self::Integer(o)) => s.cmp(o),
            (Self::List(s), Self::List(o)) => s.as_slice().cmp(o.as_slice()),
            (s @ Self::Integer(_), Self::List(o)) => slice::from_ref(s).cmp(o.as_slice()),
            (Self::List(s), o @ Self::Integer(_)) => s.as_slice().cmp(slice::from_ref(o)),
        }
    }
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        [1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
    "};

    #[test]
    fn test_part1() {
        assert_eq!(Day13.part1(TEST_INPUT), 13)
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day13.part2(TEST_INPUT), Some(140))
    }
}
