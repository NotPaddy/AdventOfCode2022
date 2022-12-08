use aoc_2022::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_day<const DAY: u8, S: Solution<DAY>>(solution: &S, input: &str, c: &mut Criterion) {
    let mut group = c.benchmark_group(format!("Day {}", DAY));
    group.bench_function("Part 1", |b| {
        b.iter(|| solution.part1(black_box(&input.replace("\r\n", "\n"))))
    });
    group.bench_function("Part 2", |b| {
        b.iter(|| solution.part2(black_box(&input.replace("\r\n", "\n"))))
    });
}

fn bench_day01(c: &mut Criterion) {
    bench_day(&day1::Day1, include_str!("../inputs/day01.txt"), c)
}
criterion_group!(day01, bench_day01);

fn bench_day02(c: &mut Criterion) {
    bench_day(&day2::Day2, include_str!("../inputs/day02.txt"), c)
}
criterion_group!(day02, bench_day02);

fn bench_day03(c: &mut Criterion) {
    bench_day(&day3::Day3, include_str!("../inputs/day03.txt"), c)
}
criterion_group!(day03, bench_day03);

fn bench_day04(c: &mut Criterion) {
    bench_day(&day4::Day4, include_str!("../inputs/day04.txt"), c)
}
criterion_group!(day04, bench_day04);

fn bench_day05(c: &mut Criterion) {
    bench_day(&day5::Day5, include_str!("../inputs/day05.txt"), c)
}
criterion_group!(day05, bench_day05);

fn bench_day06(c: &mut Criterion) {
    bench_day(&day6::Day6, include_str!("../inputs/day06.txt"), c)
}
criterion_group!(day06, bench_day06);

fn bench_day07(c: &mut Criterion) {
    bench_day(&day7::Day7, include_str!("../inputs/day07.txt"), c)
}
criterion_group!(day07, bench_day07);

fn bench_day08(c: &mut Criterion) {
    bench_day(&day8::Day8, include_str!("../inputs/day08.txt"), c)
}
criterion_group!(day08, bench_day08);

criterion_main!(day01, day02, day03, day04, day05, day06, day07, day08);
