use crate::Solution;
use itertools::Itertools;
use std::cmp::max;
use std::str::FromStr;

pub struct Day15 {
    target_row: i32,
    max_coordinate: i32,
}

impl Day15 {
    #[must_use]
    pub fn puzzle() -> Day15 {
        Self {
            target_row: 2_000_000,
            max_coordinate: 4_000_000,
        }
    }
}

#[allow(clippy::cast_sign_loss)]
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

        u64::from(positions)
    }

    fn part2(&self, input: &str) -> Option<Self::Output> {
        let sensors = get_sensors_with_distance(input);

        let corners = sensors
            .iter()
            .map(|(sensor, distance)| (sensor.position.rotate(), distance))
            .flat_map(|(rotated, distance)| {
                vec![
                    Coordinate {
                        x: rotated.x + distance,
                        y: rotated.y + distance,
                    },
                    Coordinate {
                        x: rotated.x - distance,
                        y: rotated.y - distance,
                    },
                ]
            })
            .collect::<Vec<_>>();

        let x_candidates = corners
            .iter()
            .tuple_combinations()
            .filter(|(c1, c2)| c1.x.abs_diff(c2.x) == 2)
            .map(|(c1, c2)| max(c1.x, c2.x) - 1)
            .unique();

        let y_candidates = corners
            .iter()
            .tuple_combinations()
            .filter(|(c1, c2)| c1.y.abs_diff(c2.y) == 2)
            .map(|(c1, c2)| max(c1.y, c2.y) - 1)
            .unique();

        x_candidates
            .cartesian_product(y_candidates)
            .map(|(x, y)| Coordinate { x, y }.rotate_back())
            .filter(|coordinate| {
                let range = 0..=self.max_coordinate;
                range.contains(&coordinate.x) && range.contains(&coordinate.y)
            })
            .filter(|coordinate| {
                sensors
                    .iter()
                    .all(|(sensor, _)| !sensor.is_in_range(coordinate))
            })
            .map(|coordinate| coordinate.x as u64 * 4_000_000 + coordinate.y as u64)
            .next()
    }
}

#[allow(clippy::cast_possible_wrap)]
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
        let a = (a.position.x - dist_a, a.position.y - dist_a);
        let b = (b.position.x - dist_b, b.position.y - dist_b);
        a.cmp(&b)
    });
    sensors
}

#[allow(clippy::cast_possible_wrap)]
fn process_span_gaps<'a>(
    y: i32,
    spans: &'a mut [(i32, i32)],
    sensors: &[(Sensor, i32)],
) -> &'a [(i32, i32)] {
    let mut length = 0;
    for (sensor, distance) in sensors {
        let range = distance - sensor.position.y.abs_diff(y) as i32;

        if range >= 0 {
            let span = spans.get_mut(length).unwrap();
            *span = (sensor.position.x - range, sensor.position.x + range);
            length += 1;
        }
    }

    spans[..length].sort_by(|(a, _), (b, _)| a.cmp(b));
    &spans[..length]
}

#[derive(Debug, Eq, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn rotate(&self) -> Self {
        Self {
            x: self.y - self.x,
            y: self.y + self.x,
        }
    }

    fn rotate_back(&self) -> Self {
        let y = (self.x + self.y) / 2;
        Self { x: self.y - y, y }
    }

    fn distance(&self, other: &Coordinate) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Sensor {
    position: Coordinate,
    beacon: Coordinate,
}

impl Sensor {
    fn beacon_distance(&self) -> u32 {
        self.position.distance(&self.beacon)
    }

    fn is_in_range(&self, coordinate: &Coordinate) -> bool {
        coordinate.distance(&self.position) <= self.beacon_distance()
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

        fn named_coordinate_pair(input: &str) -> IResult<&str, Coordinate> {
            map(
                separated_pair(
                    preceded(tag("x="), i32),
                    terminated(char(','), space0),
                    preceded(tag("y="), i32),
                ),
                |(x, y)| Coordinate { x, y },
            )(input)
        }

        fn sensor_line_tuples(input: &str) -> IResult<&str, (Coordinate, Coordinate)> {
            pair(
                preceded(tag("Sensor at "), named_coordinate_pair),
                preceded(tag(": closest beacon is at "), named_coordinate_pair),
            )(input)
        }

        fn sensor_line(input: &str) -> IResult<&str, Sensor> {
            map(sensor_line_tuples, |(position, beacon)| Sensor {
                position,
                beacon,
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
        assert_eq!(Day15::test().part1(TEST_INPUT), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day15::test().part2(TEST_INPUT), Some(56_000_011));
    }
}
