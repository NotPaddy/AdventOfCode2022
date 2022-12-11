use crate::Solution;
use std::collections::VecDeque;
use std::str::FromStr;

pub struct Day11;

impl Solution<11> for Day11 {
    type Output = u64;

    fn part1(&self, input: &str) -> Self::Output {
        let monkeys = &mut Monkey::parse_all(input).collect::<Vec<_>>();
        get_monkey_business(monkeys, 20, |w| w / 3)
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        let monkeys = &mut Monkey::parse_all(input).collect::<Vec<_>>();
        // Congruence relation go brr
        let divisor_product: u64 = monkeys.iter().map(|m| m.test_divisor).product();
        Some(get_monkey_business(monkeys, 10_000, |w| {
            w % divisor_product
        }))
    }
}

fn get_monkey_business<F: Fn(u64) -> u64>(
    monkeys: &mut Vec<Monkey>,
    rounds: usize,
    worry_fn: F,
) -> u64 {
    let mut inspections = vec![0u64; monkeys.len()];
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let Monkey {
                operation,
                test_divisor,
                target_true,
                target_false,
                ..
            } = monkeys[i];
            inspections[i] += monkeys[i].items.len() as u64;
            while !monkeys[i].items.is_empty() {
                let item = monkeys[i].items.pop_front().unwrap();
                let item = worry_fn(operation.invoke(item));
                let target = if item % test_divisor == 0 {
                    target_true
                } else {
                    target_false
                };
                monkeys[target].items.push_back(item)
            }
        }
    }

    inspections.sort_by(|a, b| b.cmp(a));
    inspections.iter().take(2).product()
}

struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test_divisor: u64,
    target_true: usize,
    target_false: usize,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iter = &mut s.lines().filter(|l| !l.is_empty());
        iter.next();
        Ok(Monkey {
            items: Self::next_strip_prefix(iter, "Starting items: ")?
                .split(',')
                .filter_map(|i| i.trim().parse().ok())
                .collect(),
            operation: Self::next_strip_prefix(iter, "Operation: ")?
                .parse()
                .map_err(|_| ())?,
            test_divisor: Self::next_strip_prefix(iter, "Test: divisible by ")?
                .trim()
                .parse()
                .map_err(|_| ())?,
            target_true: Self::next_strip_prefix(iter, "If true: throw to monkey ")?
                .trim()
                .parse()
                .map_err(|_| ())?,
            target_false: Self::next_strip_prefix(iter, "If false: throw to monkey ")?
                .trim()
                .parse()
                .map_err(|_| ())?,
        })
    }
}

impl Monkey {
    fn parse_all(input: &str) -> impl Iterator<Item = Monkey> + '_ {
        input.split("\n\n").filter_map(|m| m.parse::<Monkey>().ok())
    }

    fn next_strip_prefix<'a, I>(iter: &mut I, prefix: &str) -> Result<&'a str, ()>
    where
        I: Iterator<Item = &'a str>,
    {
        iter.next()
            .ok_or(())?
            .trim_start()
            .strip_prefix(prefix)
            .ok_or(())
    }
}

#[derive(Copy, Clone)]
enum Operation {
    Square,
    Multiply(u64),
    Add(u64),
}

impl Operation {
    fn invoke(&self, value: u64) -> u64 {
        match self {
            Operation::Square => value * value,
            Operation::Multiply(arg) => value * arg,
            Operation::Add(arg) => value + arg,
        }
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let operation = s.strip_prefix("new = old ").ok_or(())?;
        match operation.split_whitespace().collect::<Vec<_>>()[..] {
            ["*", "old"] => Ok(Operation::Square),
            ["*", arg] => Ok(Operation::Multiply(arg.parse().map_err(|_| ())?)),
            ["+", arg] => Ok(Operation::Add(arg.parse().map_err(|_| ())?)),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
    Monkey 0:
      Starting items: 79, 98
      Operation: new = old * 19
      Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3

    Monkey 1:
      Starting items: 54, 65, 75, 74
      Operation: new = old + 6
      Test: divisible by 19
        If true: throw to monkey 2
        If false: throw to monkey 0

    Monkey 2:
      Starting items: 79, 60, 97
      Operation: new = old * old
      Test: divisible by 13
        If true: throw to monkey 1
        If false: throw to monkey 3

    Monkey 3:
      Starting items: 74
      Operation: new = old + 3
      Test: divisible by 17
        If true: throw to monkey 0
        If false: throw to monkey 1
    "};

    #[test]
    fn test_part1() {
        assert_eq!(Day11.part1(TEST_INPUT), 10605)
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day11.part2(TEST_INPUT), Some(271331015820))
    }
}
