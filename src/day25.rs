use crate::Solution;

pub struct Day25;

impl Solution<25> for Day25 {
    type Output = String;

    fn part1(&self, input: &str) -> Self::Output {
        encode_snafu(input.lines().map(decode_snafu).sum())
    }
}

fn decode_snafu(snafu: &str) -> usize {
    const SNAFU_SYMBOLS: &str = "=-012";
    snafu
        .bytes()
        .filter_map(|c| SNAFU_SYMBOLS.bytes().position(|a| a == c))
        .fold(0, |num, d| num * 5 + d - 2)
}

fn encode_snafu(num: usize) -> String {
    const SNAFU_SYMBOLS: [&str; 5] = ["0", "1", "2", "=", "-"];
    if num == 0 {
        "".to_string()
    } else {
        encode_snafu((num + 2) / 5) + SNAFU_SYMBOLS[num % 5]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        1=-0-2
        12111
        2=0=
        21
        2=01
        111
        20012
        112
        1=-1=
        1-12
        12
        1=
        122
    "};

    #[test]
    fn test_part1() {
        assert_eq!(Day25.part1(TEST_INPUT), "2=-1=0")
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day25.part2(TEST_INPUT), None)
    }
}
