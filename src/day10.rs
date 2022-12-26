use crate::day10::DayResult::{Part1, Part2};
use crate::Solution;
use std::fmt::{Display, Formatter};

pub struct Day10;

impl Solution<10> for Day10 {
    type Output = DayResult;

    fn part1(&self, input: &str) -> Self::Output {
        Part1(
            iterate_state(input)
                .enumerate()
                .skip(19)
                .step_by(40)
                .map(|(cycle, register)| (cycle + 1) as i64 * register)
                .sum(),
        )
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        let screen = iterate_state(input)
            .enumerate()
            .fold([false; 240], |mut screen, (cycle, register)| {
                if register.abs_diff(cycle as i64 % 40) <= 1 {
                    screen[cycle] = true;
                }
                screen
            })
            .map(|on| if on { '#' } else { '.' })
            .chunks(40)
            .map(|c| c.iter().collect::<String>())
            .fold(String::new(), |acc, line| acc + &line + "\n");

        Some(Part2(screen))
    }
}

fn iterate_state(input: &str) -> impl Iterator<Item = i64> + '_ {
    input
        .lines()
        .scan(1, |register, line| {
            let next = match line.split_once(' ') {
                Some(("addx", arg)) => {
                    let steps = vec![*register, *register];
                    *register += arg.parse::<i64>().ok()?;
                    steps
                }
                _ => vec![*register],
            };
            Some(next)
        })
        .flatten()
}

#[derive(Debug, Eq, PartialEq)]
pub enum DayResult {
    Part1(i64),
    Part2(String),
}

impl Display for DayResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Part1(r) => write!(f, "{r}")?,
            Part2(r) => write!(f, "{r}")?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::day10::DayResult::Part2;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        assert_eq!(Day10.part1(TEST_INPUT), Part1(13140));
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day10.part2(TEST_INPUT), Some(Part2(CRT_OUTPUT.to_owned())));
    }

    const CRT_OUTPUT: &str = indoc! {"
        ##..##..##..##..##..##..##..##..##..##..
        ###...###...###...###...###...###...###.
        ####....####....####....####....####....
        #####.....#####.....#####.....#####.....
        ######......######......######......####
        #######.......#######.......#######.....
    "};

    const TEST_INPUT: &str = indoc! {"
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
    "};
}
