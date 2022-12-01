use std::fmt::Display;
use std::time::Instant;

pub trait Solution<const DAY: u8> {
    type Output: Display;
    fn part1(&self, input: &str) -> Self::Output;
    fn part2(&self, input: &str) -> Option<Self::Output> {
        let _ = input;
        None
    }
}

pub struct Runner<'a> {
    input: &'a str,
}

impl Runner<'_> {
    pub fn new(input: &'_ str) -> Runner<'_> {
        Runner { input }
    }

    pub fn run<const DAY: u8, S: Solution<DAY>>(&'_ self, solution: &S) {
        println!("--------");
        println!("Day {:02}", DAY);
        println!("--------");

        println!("Part 1:");
        let start = Instant::now();
        let result = solution.part1(self.input);
        let elapsed = start.elapsed();
        println!("Result: {} - Time elapsed {:?}", result, elapsed);

        println!("Part 2:");
        let start = Instant::now();
        let result = solution.part2(self.input);
        let elapsed = start.elapsed();
        if let Some(result) = result {
            println!("Result: {} - Time elapsed {:?}", result, elapsed);
        } else {
            println!("Result: Not solved");
        }
    }
}
