use itertools::Itertools;
use std::{collections::HashSet, fs, ops::RangeInclusive, time::Instant};

use day_15::{circle::Circle, clamp::Clamp, point::Point};

fn part_one(input: &[String], y: i32) -> String {
    let sensor_beacons: Vec<_> = input.iter().map(|line| SensorBeacon::parse(line)).collect();
    let map = Map::new(sensor_beacons);

    map.non_beacon_places(y).to_string()
}

fn part_two(input: &[String], max: Point) -> String {
    let sensor_beacons: Vec<_> = input.iter().map(|line| SensorBeacon::parse(line)).collect();
    let map = Map::new(sensor_beacons);

    let distress_beacon = map
        .find_distress_beacon(0..=max.x, 0..=max.y)
        .expect("should find distress beacon");

    let distress_signal: isize =
        (4_000_000 * distress_beacon.x as isize) + distress_beacon.y as isize;

    distress_signal.to_string()
}

struct Map {
    circles: Vec<Circle>,
    sensor_beacons: Vec<SensorBeacon>,
}

struct SensorBeacon {
    sensor: Point,
    beacon: Point,
}

impl Map {
    fn new(sensor_beacons: Vec<SensorBeacon>) -> Self {
        let circles: Vec<_> = sensor_beacons.iter().map(|pair| pair.circle()).collect();

        Self {
            circles,
            sensor_beacons,
        }
    }

    fn sensors_coverage(&self, y: i32) -> impl Iterator<Item = RangeInclusive<i32>> {
        // For each circle get the intersection range
        let ranges = self
            .circles
            .iter()
            .filter_map(move |circle| {
                let center = circle.center;
                let dy = center.y.abs_diff(y);

                // Check if circle intersect y line
                if dy > circle.radius {
                    return None;
                }

                let dx = circle.radius - dy;
                Some(center.x - dx as i32..=center.x + dx as i32)
            })
            .sorted_by_key(|range| *range.start());

        // Merge the intersection ranges
        ranges.coalesce(|r1, r2| {
            if *r1.end() + 1 >= *r2.start() {
                if r2.end() > r1.end() {
                    Ok(*r1.start()..=*r2.end())
                } else {
                    Ok(r1)
                }
            } else {
                Err((r1, r2))
            }
        })
    }

    fn non_beacon_places(&self, y: i32) -> usize {
        let beacons: HashSet<_> = self
            .sensor_beacons
            .iter()
            .map(|pair| pair.beacon)
            .filter_map(|beacon| if beacon.y == y { Some(beacon.x) } else { None })
            .collect();

        self.sensors_coverage(y)
            .map(|range| {
                let size = range.end() - range.start() + 1;
                let beacons_in_range = beacons
                    .iter()
                    .filter(|beacon| range.contains(beacon))
                    .count();

                size as usize - beacons_in_range
            })
            .sum()
    }

    fn find_distress_beacon(
        &self,
        x_range: RangeInclusive<i32>,
        y_range: RangeInclusive<i32>,
    ) -> Option<Point> {
        y_range
            .filter_map(|y| {
                self.sensors_coverage(y)
                    .filter_map(|range| {
                        let clamped = range.clamp(&x_range);
                        if clamped.start() <= clamped.end() {
                            Some(clamped)
                        } else {
                            None
                        }
                    })
                    .nth(1)
                    .map(|range| Point {
                        x: range.start() - 1,
                        y,
                    })
            })
            .next()
    }
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
        let result = part_two(&input, (20, 20).into());
        assert_eq!(result, "56000011");
    }

    #[test]
    fn sensor_coverage() {
        let sensor_beacons = vec![
            SensorBeacon {
                sensor: (10, 10).into(),
                beacon: (10, 8).into(),
            },
            SensorBeacon {
                sensor: (0, 0).into(),
                beacon: (0, 10).into(),
            },
        ];

        let map = Map::new(sensor_beacons);
        let coverages: Vec<_> = map.sensors_coverage(10).collect();

        assert_eq!(coverages, vec![0..=0, 8..=12]);
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
