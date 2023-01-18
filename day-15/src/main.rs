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

    let (mut max_x, mut max_y, mut min_x, mut min_y) = (i32::MIN, i32::MIN, i32::MAX, i32::MAX);
    for SensorBeacon { beacon, .. } in sensor_beacons.iter() {
        max_x = max(beacon.x, max_x);
        max_y = max(beacon.y, max_y);
        min_x = min(beacon.x, min_x);
        min_y = min(beacon.y, min_y);
    }

    dbg!((max_x, max_y, min_x, min_y));

    let mut result = (min_x..=max_x)
        .map(|x| Point { x, y: row })
        .filter(|point| circles.iter().any(move |circle| circle.contains(*point)))
        .count();

    dbg!(result);

    let points = sensor_beacons
        .iter()
        .flat_map(|pair| [pair.beacon, pair.sensor])
        .collect::<HashSet<_>>();

    result -= points.iter().filter(|point| point.y == row).count();

    result.to_string()
}

fn part_two(input: &[String]) -> String {
    "NOT IMPLEMENTED".to_owned()
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
        let result = part_two(&input);
        assert_eq!(result, "NOT IMPLEMENTED");
    }
}

// --- Lines bellow do not need to be modified ---

fn main() {
    let input = parse_input(false);

    let start_one = Instant::now();
    let result_one = part_one(&input, 2_000_000);
    let elapsed_one = start_one.elapsed();

    let start_two = Instant::now();
    let result_two = part_two(&input);
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
