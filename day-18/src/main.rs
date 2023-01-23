use std::{collections::HashSet, fs, time::Instant};

use grid3::{Grid3, Ranges3};
use point3::Point3;

mod grid3;
mod point3;

const NEIGHBOORS: [Point3; 6] = [
    Point3 { x: 1, y: 0, z: 0 },
    Point3 { x: -1, y: 0, z: 0 },
    Point3 { x: 0, y: 1, z: 0 },
    Point3 { x: 0, y: -1, z: 0 },
    Point3 { x: 0, y: 0, z: 1 },
    Point3 { x: 0, y: 0, z: -1 },
];

fn part_one(input: &[String]) -> String {
    let points = input.iter().map(|line| Point3::parse(line));
    let grid = Grid3::from_points(points);

    let total_surface: usize = grid
        .iter()
        .filter(|(_, element)| matches!(element, Element::Rock))
        .map(|(point, _)| {
            NEIGHBOORS
                .iter()
                .filter_map(|n| grid.get(point + *n))
                .filter(|e| matches!(e, Element::Air))
                .count()
        })
        .sum();

    total_surface.to_string()
}

fn part_two(input: &[String]) -> String {
    let points = input.iter().map(|line| Point3::parse(line));
    let mut grid = Grid3::from_points(points);

    grid.pour_water();

    let total_surface: usize = grid
        .iter()
        .filter(|(_, element)| matches!(element, Element::Rock))
        .map(|(point, _)| {
            NEIGHBOORS
                .iter()
                .filter_map(|n| grid.get(point + *n))
                .filter(|e| matches!(e, Element::Water))
                .count()
        })
        .sum();

    total_surface.to_string()
}

#[derive(Debug, Copy, Clone)]
enum Element {
    Air,
    Rock,
    Water,
}

impl Point3 {
    fn parse(line: &str) -> Self {
        let mut coords = line.trim().split(",");

        Self {
            x: coords.next().unwrap().parse().expect("invalid x value"),
            y: coords.next().unwrap().parse().expect("invalid y value"),
            z: coords.next().unwrap().parse().expect("invalid z value"),
        }
    }
}

impl Grid3<Element> {
    fn from_points(iter: impl Iterator<Item = Point3> + Clone) -> Self {
        let (min, max) = Point3::min_max(iter.clone()).expect("empty iterator");
        let ranges = Ranges3 {
            x: (min.x - 1..=max.x + 1),
            y: (min.y - 1..=max.y + 1),
            z: (min.z - 1..=max.z + 1),
        };

        let mut grid = Self::new_set_default(ranges, Element::Air);

        for point in iter {
            *grid.get_mut_unchecked(point) = Element::Rock;
        }

        grid
    }

    fn pour_water(&mut self) {
        let ranges = self.ranges();
        let start: Point3 = (*ranges.x.start(), *ranges.y.start(), *ranges.z.start()).into();

        let mut cur = vec![start];
        let mut next = vec![];

        while !cur.is_empty() {
            next.clear();

            for point in cur.iter() {
                let element = self.get_mut(*point).unwrap();

                if let Element::Water = element {
                    continue;
                }

                *element = Element::Water;

                next.extend(
                    NEIGHBOORS
                        .iter()
                        .map(|n| *n + *point)
                        .filter(|p| matches!(self.get(*p), Some(Element::Air))),
                )
            }

            std::mem::swap(&mut cur, &mut next);
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
        let result = part_one(&input);
        assert_eq!(result, "64");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(result, "58");
    }
}

// --- Lines bellow do not need to be modified ---

fn main() {
    let input = parse_input(false);

    let start_one = Instant::now();
    let result_one = part_one(&input);
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
