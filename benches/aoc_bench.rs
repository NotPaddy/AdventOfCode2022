use aoc_2022::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_day<const DAY: u8, S: Solution<DAY>>(solution: &S, input: &str, c: &mut Criterion) {
    let mut group = c.benchmark_group(format!("Day {DAY:02}"));
    group.noise_threshold(0.05);
    group.bench_function("Part 1", |b| {
        b.iter(|| solution.part1(black_box(&input.replace("\r\n", "\n"))))
    });
    group.bench_function("Part 2", |b| {
        b.iter(|| solution.part2(black_box(&input.replace("\r\n", "\n"))))
    });
    group.finish()
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

fn bench_day09(c: &mut Criterion) {
    bench_day(&day9::Day9, include_str!("../inputs/day09.txt"), c)
}
criterion_group!(day09, bench_day09);

fn bench_day10(c: &mut Criterion) {
    bench_day(&day10::Day10, include_str!("../inputs/day10.txt"), c)
}
criterion_group!(day10, bench_day10);

fn bench_day11(c: &mut Criterion) {
    bench_day(&day11::Day11, include_str!("../inputs/day11.txt"), c)
}
criterion_group!(day11, bench_day11);

fn bench_day12(c: &mut Criterion) {
    bench_day(&day12::Day12, include_str!("../inputs/day12.txt"), c)
}
criterion_group!(day12, bench_day12);

fn bench_day13(c: &mut Criterion) {
    bench_day(&day13::Day13, include_str!("../inputs/day13.txt"), c)
}
criterion_group!(day13, bench_day13);

fn bench_day14(c: &mut Criterion) {
    bench_day(&day14::Day14, include_str!("../inputs/day14.txt"), c)
}
criterion_group!(day14, bench_day14);

fn bench_day15(c: &mut Criterion) {
    bench_day(
        &day15::Day15::puzzle(),
        include_str!("../inputs/day15.txt"),
        c,
    )
}
criterion_group!(day15, bench_day15);

fn bench_day16(c: &mut Criterion) {
    bench_day(&day16::Day16, include_str!("../inputs/day16.txt"), c)
}
criterion_group!(day16, bench_day16);

fn bench_day17(c: &mut Criterion) {
    bench_day(&day17::Day17, include_str!("../inputs/day17.txt"), c)
}
criterion_group!(day17, bench_day17);

fn bench_day18(c: &mut Criterion) {
    bench_day(&day18::Day18, include_str!("../inputs/day18.txt"), c)
}
criterion_group!(day18, bench_day18);

fn bench_day19(c: &mut Criterion) {
    bench_day(&day19::Day19, include_str!("../inputs/day19.txt"), c)
}
criterion_group!(day19, bench_day19);

fn bench_day20(c: &mut Criterion) {
    bench_day(&day20::Day20, include_str!("../inputs/day20.txt"), c)
}
criterion_group!(day20, bench_day20);

fn bench_day21(c: &mut Criterion) {
    bench_day(&day21::Day21, include_str!("../inputs/day21.txt"), c)
}
criterion_group!(day21, bench_day21);

fn bench_day22(c: &mut Criterion) {
    bench_day(&day22::Day22, include_str!("../inputs/day22.txt"), c)
}
criterion_group!(day22, bench_day22);

fn bench_day23(c: &mut Criterion) {
    bench_day(&day23::Day23, include_str!("../inputs/day23.txt"), c)
}
criterion_group!(day23, bench_day23);

fn bench_day24(c: &mut Criterion) {
    bench_day(&day24::Day24, include_str!("../inputs/day24.txt"), c)
}
criterion_group!(day24, bench_day24);

fn bench_day25(c: &mut Criterion) {
    bench_day(&day25::Day25, include_str!("../inputs/day25.txt"), c)
}
criterion_group!(day25, bench_day25);

criterion_main!(
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);
