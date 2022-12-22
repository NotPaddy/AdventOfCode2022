use crate::Solution;
use fxhash::FxHashMap;
use nom::combinator::map_res;
use nom::{
    branch::alt,
    character::complete::alpha1,
    character::complete::multispace0,
    character::complete::space1,
    combinator::{iterator, map},
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::ops::{Add, Div, Mul, Sub};

pub struct Day21;

impl Solution<21> for Day21 {
    type Output = i64;

    fn part1(&self, input: &str) -> Self::Output {
        let monkeys = iterator(input, terminated(parse_monkey, multispace0))
            .map(|m| (m.name.clone(), m))
            .collect::<FxHashMap<_, _>>();
        let root = &monkeys["root"];
        root.calculate(&monkeys).unwrap()
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        let monkeys = iterator(input, terminated(parse_monkey, multispace0))
            .map(|m| (m.name.clone(), m))
            .collect::<FxHashMap<_, _>>();
        let root = &monkeys["root"];

        let mut values = FxHashMap::<String, i64>::default();
        let MonkeyExpression::Expression(root_calc) = &root.expression else {
            return None;
        };

        let left = monkeys.get(&root_calc.left)?;
        let right = monkeys.get(&root_calc.right)?;
        let left_val = left.calculate_equation_tree(&monkeys, &mut values);
        let right_val = right.calculate_equation_tree(&monkeys, &mut values);

        if let Some(left_val) = left_val {
            right.solve_for_human(&monkeys, &mut values, left_val)
        } else if let Some(right_val) = right_val {
            left.solve_for_human(&monkeys, &mut values, right_val)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Monkey<T> {
    name: String,
    expression: MonkeyExpression<T>,
}

const HUMAN_NAME: &str = "humn";

impl<T> Monkey<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    fn calculate<S>(&self, monkeys: &HashMap<String, Monkey<T>, S>) -> Option<T>
    where
        S: BuildHasher,
    {
        self.expression.eval(monkeys)
    }

    fn calculate_equation_tree<S>(
        &self,
        monkeys: &HashMap<String, Monkey<T>, S>,
        values: &mut HashMap<String, T, S>,
    ) -> Option<T>
    where
        S: BuildHasher,
    {
        if self.name == HUMAN_NAME {
            return None;
        }
        if let Some(&value) = values.get(&self.name) {
            return Some(value);
        }
        let res = self.expression.solve(monkeys, values)?;
        values.insert(self.name.clone(), res);
        Some(res)
    }

    fn solve_for_human<S>(
        &self,
        monkeys: &HashMap<String, Monkey<T>, S>,
        values: &mut HashMap<String, T, S>,
        value: T,
    ) -> Option<T>
    where
        S: BuildHasher,
    {
        match &self.expression {
            MonkeyExpression::Constant(_) => Some(value),
            MonkeyExpression::Expression(expression) => {
                let left = &monkeys.get(&expression.left)?;
                let right = &monkeys.get(&expression.right)?;
                let left_value = left.calculate_equation_tree(monkeys, values);
                let right_value = right.calculate_equation_tree(monkeys, values);
                let result =
                    expression
                        .operator
                        .solve_for_missing(left_value, right_value, value)?;
                if left_value.is_none() {
                    left.solve_for_human(monkeys, values, result)
                } else if right_value.is_none() {
                    right.solve_for_human(monkeys, values, result)
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug)]
enum MonkeyExpression<T> {
    Constant(T),
    Expression(Expression),
}

impl<T> MonkeyExpression<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    fn eval<S>(&self, monkeys: &HashMap<String, Monkey<T>, S>) -> Option<T>
    where
        S: BuildHasher,
    {
        match self {
            MonkeyExpression::Constant(c) => Some(*c),
            MonkeyExpression::Expression(expression) => {
                let left = monkeys.get(&expression.left)?.calculate(&monkeys)?;
                let right = monkeys.get(&expression.right)?.calculate(&monkeys)?;
                let res = expression.operator.eval(left, right);
                Some(res)
            }
        }
    }

    fn solve<S>(
        &self,
        monkeys: &HashMap<String, Monkey<T>, S>,
        values: &mut HashMap<String, T, S>,
    ) -> Option<T>
    where
        S: BuildHasher,
    {
        match self {
            MonkeyExpression::Constant(c) => Some(*c),
            MonkeyExpression::Expression(calculation) => {
                let left = monkeys
                    .get(&calculation.left)?
                    .calculate_equation_tree(&monkeys, values)?;
                let right = monkeys
                    .get(&calculation.right)?
                    .calculate_equation_tree(&monkeys, values)?;
                let res = calculation.operator.eval(left, right);
                Some(res)
            }
        }
    }
}

#[derive(Debug)]
struct Expression {
    left: String,
    right: String,
    operator: Operator,
}

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn eval<T>(&self, left: T, right: T) -> T
    where
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
    {
        match self {
            Operator::Add => left + right,
            Operator::Subtract => left - right,
            Operator::Multiply => left * right,
            Operator::Divide => left / right,
        }
    }

    fn solve_for_missing<T>(&self, left: Option<T>, right: Option<T>, result: T) -> Option<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
    {
        Some(match self {
            Operator::Add => result - left.or(right).unwrap(),
            Operator::Subtract => match (left, right) {
                (Some(left), _) => left - result,
                (_, Some(right)) => result + right,
                (_, _) => return None,
            },
            Operator::Multiply => result / left.or(right).unwrap(),
            Operator::Divide => match (left, right) {
                (Some(left), _) => left / result,
                (_, Some(right)) => result * right,
                (_, _) => return None,
            },
        })
    }
}

impl TryFrom<char> for Operator {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Operator::Add),
            '-' => Ok(Operator::Subtract),
            '*' => Ok(Operator::Multiply),
            '/' => Ok(Operator::Divide),
            _ => Err(()),
        }
    }
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey<i64>> {
    use nom::character::complete::char;
    map(
        separated_pair(
            map(alpha1, String::from),
            terminated(char(':'), space1),
            alt((parse_constant_monkey, parse_expression_monkey)),
        ),
        |(name, expression)| Monkey { name, expression },
    )(input)
}

fn parse_constant_monkey(input: &str) -> IResult<&str, MonkeyExpression<i64>> {
    use nom::character::complete::i64;
    map(i64, MonkeyExpression::Constant)(input)
}

fn parse_expression_monkey(input: &str) -> IResult<&str, MonkeyExpression<i64>> {
    map(parse_expression, MonkeyExpression::Expression)(input)
}

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    use nom::character::complete::char;
    map(
        tuple((
            map(alpha1, String::from),
            delimited(
                space1,
                map_res(
                    alt((char('+'), char('-'), char('*'), char('/'))),
                    Operator::try_from,
                ),
                space1,
            ),
            map(alpha1, String::from),
        )),
        |(left, operator, right): (String, Operator, String)| Expression {
            left,
            right,
            operator,
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        root: pppw + sjmn
        dbpl: 5
        cczh: sllz + lgvd
        zczc: 2
        ptdq: humn - dvpt
        dvpt: 3
        lfqf: 4
        humn: 5
        ljgn: 2
        sjmn: drzm * dbpl
        sllz: 4
        pppw: cczh / lfqf
        lgvd: ljgn * ptdq
        drzm: hmdt - zczc
        hmdt: 32
    "};

    #[test]
    fn test_part1() {
        assert_eq!(Day21.part1(TEST_INPUT), 152)
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day21.part2(TEST_INPUT), Some(301))
    }
}
