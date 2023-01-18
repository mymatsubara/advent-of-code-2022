use std::{
    cmp::{max, min},
    collections::HashSet,
    fs,
    time::Instant,
};

use day_15::{circle::Circle, point::Point};

fn part_one(input: &[String], row: i32) -> String {
    let sensor_beacons: Vec<_> = input.iter().map(|line| SensorBeacon::parse(line)).collect();
    let circles: Vec<_> = sensor_beacons.iter().map(|pair| pair.circle()).collect();

    let (mut max_x, mut min_x) = (i32::MIN, i32::MAX);
    for SensorBeacon { beacon, .. } in sensor_beacons.iter() {
        max_x = max(beacon.x, max_x);
        min_x = min(beacon.x, min_x);
    }

    let mut result = (min_x..=max_x)
        .map(|x| Point { x, y: row })
        .filter(|point| circles.iter().any(move |circle| circle.contains(*point)))
        .count();

    let points = sensor_beacons
        .iter()
        .flat_map(|pair| [pair.beacon, pair.sensor])
        .collect::<HashSet<_>>();

    result -= points.iter().filter(|point| point.y == row).count();

    result.to_string()
}

fn part_two(input: &[String], max: Point) -> String {
    let sensor_beacons: Vec<_> = input.iter().map(|line| SensorBeacon::parse(line)).collect();
    let circles: Vec<_> = sensor_beacons.iter().map(|pair| pair.circle()).collect();

    let (y, distress_beacon_range) = (0..max.y)
        .map(|y| {
            Range::merge(
                circles
                    .iter()
                    .filter_map(|circle| intersect(*circle, y))
                    .collect::<Vec<_>>(),
            )
        })
        .enumerate()
        .find(|(_, ranges)| ranges.len() > 1)
        .expect("should find distress beacon range");

    let distress_beacon = Point {
        x: distress_beacon_range.first().unwrap().end + 1,
        y: y as i32,
    };

    let distress_signal: isize =
        (4_000_000 * distress_beacon.x as isize) + distress_beacon.y as isize;

    distress_signal.to_string()
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
struct Range {
    start: i32,
    end: i32,
}

struct SensorBeacon {
    sensor: Point,
    beacon: Point,
}

impl SensorBeacon {
    fn parse(line: &str) -> SensorBeacon {
        let (sensor_x, rest) = line
            .trim()
            .trim_start_matches("Sensor at x=")
            .split_once(", y=")
            .expect("invalid sensor x input");

        let (sensor_y, rest) = rest
            .split_once(": closest beacon is at x=")
            .expect("invalid sensor y input");

        let (beacon_x, beacon_y) = rest.split_once(", y=").expect("invalid beacon input");

        let sensor = Point {
            x: sensor_x.parse().expect("invalid sensor x value"),
            y: sensor_y.parse().expect("invalid sensor y value"),
        };

        let beacon = Point {
            x: beacon_x.parse().expect("invalid beacon x value"),
            y: beacon_y.parse().expect("invalid beacon y value"),
        };

        SensorBeacon { sensor, beacon }
    }

    fn circle(&self) -> Circle {
        Circle {
            center: self.sensor,
            radius: self.sensor.manhattan_dist(self.beacon),
        }
    }
}

impl Range {
    fn new(start: i32, end: i32) -> Option<Self> {
        if start > end {
            return None;
        }

        Some(Self { start, end })
    }

    fn merge(mut ranges: Vec<Self>) -> Vec<Self> {
        ranges.sort();

        if ranges.len() == 0 {
            return vec![];
        }

        let mut result = Vec::with_capacity(ranges.len());
        result.push(ranges.first().copied().unwrap());

        for range in ranges.iter().skip(1) {
            let cur = result.last_mut().unwrap();
            match range.merge_with(cur) {
                Some(merged) => *cur = merged,
                None => result.push(*range),
            };
        }

        result
    }

    fn merge_with(&self, other: &Self) -> Option<Self> {
        if self.intersect(other) {
            return Some(Self {
                start: min(self.start, other.start),
                end: max(self.end, other.end),
            });
        }

        None
    }

    fn intersect(&self, other: &Self) -> bool {
        (other.start..=other.end).contains(&self.start)
            || (self.start..=self.end).contains(&other.start)
    }
}

fn intersect(circle: Circle, y: i32) -> Option<Range> {
    let center = circle.center;
    let d_y = (center.y - y).abs() as usize;

    if d_y > circle.radius {
        return None;
    }

    let d_x = (circle.radius - d_y) as i32;
    Range::new(center.x - d_x, center.x + d_x)
}

// --- TESTS ---

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = parse_input(true);
        let result = part_one(&input, 10);
        assert_eq!(result, "26");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input, (20, 20).into());
        assert_eq!(result, "56000011");
    }

    #[test]
    fn intersect_circle() {
        let circle = Circle {
            center: Point { x: 2, y: 2 },
            radius: 2,
        };

        assert_eq!(intersect(circle, -1), None);
        assert_eq!(intersect(circle, 0), Some(Range { start: 2, end: 2 }));
        assert_eq!(intersect(circle, 1), Some(Range { start: 1, end: 3 }));
        assert_eq!(intersect(circle, 2), Some(Range { start: 0, end: 4 }));
        assert_eq!(intersect(circle, 3), Some(Range { start: 1, end: 3 }));
        assert_eq!(intersect(circle, 4), Some(Range { start: 2, end: 2 }));
        assert_eq!(intersect(circle, 5), None);
    }
}

// --- Lines bellow do not need to be modified ---

fn main() {
    let input = parse_input(false);

    let start_one = Instant::now();
    let result_one = part_one(&input, 2_000_000);
    let elapsed_one = start_one.elapsed();

    let start_two = Instant::now();
    let result_two = part_two(&input, (4_000_000, 4_000_000).into());
    let elapsed_two = start_two.elapsed();

    println!("Part one result: {result_one} [time: {:.2?}]", elapsed_one);
    println!("Part two result: {result_two} [time: {:.2?}]", elapsed_two);
}

fn parse_input(test: bool) -> Vec<String> {
    let file = if test { "input.test.txt" } else { "input.txt" };

    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("'{file}' not found"))
        .lines()
        .map(|line| line.trim().to_owned())
        .collect()
}
