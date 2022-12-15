use crate::Solution;
use std::cmp::max;
use std::str::FromStr;

pub struct Day15 {
    target_row: i32,
    max_coordinate: i32,
}

impl Day15 {
    pub fn puzzle() -> Day15 {
        Self {
            target_row: 2_000_000,
            max_coordinate: 4_000_000,
        }
    }
}

impl Solution<15> for Day15 {
    type Output = u64;

    fn part1(&self, input: &str) -> Self::Output {
        let sensors = get_sensors_with_distance(input);

        let mut spans: Vec<(i32, i32)> = vec![(i32::MIN, i32::MAX); sensors.len()];
        let mut positions = 0;
        let mut max_end = i32::MIN;
        for &(start, end) in process_span_gaps(self.target_row, &mut spans, &sensors) {
            positions += (end - max(max_end, start)).max(0) as u32;
            max_end = end.max(max_end);
        }

        positions as u64
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        let sensors = get_sensors_with_distance(input);

        let mut spans: Vec<(i32, i32)> = vec![(i32::MIN, i32::MAX); sensors.len()];
        // Brute force go brrr
        for y in 0..=self.max_coordinate {
            let mut x = 0;
            for &(start, end) in process_span_gaps(y, &mut spans, &sensors) {
                if (start..=end).contains(&x) {
                    x = end + 1;
                }
            }

            if x <= self.max_coordinate {
                return Some(x as u64 * 4_000_000 + y as u64);
            }
        }

        None
    }
}

fn get_sensors_with_distance(input: &str) -> Vec<(Sensor, i32)> {
    let mut sensors = input
        .lines()
        .map(|l| l.parse::<Sensor>().unwrap())
        .map(|s| {
            let distance = s.beacon_distance() as i32;
            (s, distance)
        })
        .collect::<Vec<_>>();
    sensors.sort_by(|(a, dist_a), (b, dist_b)| {
        let a = (a.x - dist_a, a.y - dist_a);
        let b = (b.x - dist_b, b.y - dist_b);
        a.cmp(&b)
    });
    sensors
}

fn process_span_gaps<'a>(
    y: i32,
    spans: &'a mut [(i32, i32)],
    sensors: &[(Sensor, i32)],
) -> &'a [(i32, i32)] {
    let mut length = 0;
    for (sensor, distance) in sensors {
        let range = distance - sensor.y.abs_diff(y) as i32;

        if range >= 0 {
            let span = spans.get_mut(length).unwrap();
            *span = (sensor.x - range, sensor.x + range);
            length += 1;
        }
    }

    spans[..length].sort_by(|(a, _), (b, _)| a.cmp(b));
    &spans[..length]
}

#[derive(Debug, Eq, PartialEq)]
struct Sensor {
    x: i32,
    y: i32,
    beacon_x: i32,
    beacon_y: i32,
}

impl Sensor {
    fn beacon_distance(&self) -> u32 {
        self.x.abs_diff(self.beacon_x) + self.y.abs_diff(self.beacon_y)
    }
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::bytes::complete::tag;
        use nom::character::complete::{char, i32, space0};
        use nom::combinator::map;
        use nom::sequence::{pair, preceded, separated_pair, terminated};
        use nom::IResult;

        fn named_coordinate_pair(input: &str) -> IResult<&str, (i32, i32)> {
            separated_pair(
                preceded(tag("x="), i32),
                terminated(char(','), space0),
                preceded(tag("y="), i32),
            )(input)
        }

        type Coordinate = (i32, i32);

        fn sensor_line_tuples(input: &str) -> IResult<&str, (Coordinate, Coordinate)> {
            pair(
                preceded(tag("Sensor at "), named_coordinate_pair),
                preceded(tag(": closest beacon is at "), named_coordinate_pair),
            )(input)
        }

        fn sensor_line(input: &str) -> IResult<&str, Sensor> {
            map(sensor_line_tuples, |(sensor, beacon)| Sensor {
                x: sensor.0,
                y: sensor.1,
                beacon_x: beacon.0,
                beacon_y: beacon.1,
            })(input)
        }

        sensor_line(s).map_err(|_| ()).map(|t| t.1)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    impl Day15 {
        fn test() -> Self {
            Self {
                target_row: 10,
                max_coordinate: 20,
            }
        }
    }

    const TEST_INPUT: &str = indoc! {"
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
    "};

    #[test]
    fn test_part1() {
        assert_eq!(Day15::test().part1(TEST_INPUT), 26)
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day15::test().part2(TEST_INPUT), Some(56000011))
    }
}
