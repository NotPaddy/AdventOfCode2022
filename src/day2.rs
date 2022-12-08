use crate::Solution;

pub struct Day2;

impl Solution<2> for Day2 {
    type Output = u32;

    fn part1(&self, input: &str) -> Self::Output {
        input
            .lines()
            .filter_map(parse_line)
            .map(|(opp, own)| score(opp, own) as u32)
            .sum()
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        let score = input
            .lines()
            .filter_map(parse_line)
            .map(|(opp, outcome)| (opp, get_own(opp, outcome)))
            .map(|(opp, own)| score(opp, own) as u32)
            .sum();
        Some(score)
    }
}

fn parse_line(line: &str) -> Option<(u8, u8)> {
    let bytes = line.as_bytes();
    let opp = bytes.first()? - b'A' + 1;
    let own = bytes.get(2)? - b'X' + 1;
    Some((opp, own))
}

fn score(opp: u8, own: u8) -> u8 {
    let outcome = ((4 + own - opp) % 3) * 3;
    own + outcome
}

fn get_own(opp: u8, outcome: u8) -> u8 {
    (opp + outcome) % 3 + 1
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        A Y
        B X
        C Z
        "};

    #[test]
    fn test_part1() {
        assert_eq!(Day2.part1(TEST_INPUT), 15)
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day2.part2(TEST_INPUT), Some(12))
    }
}
