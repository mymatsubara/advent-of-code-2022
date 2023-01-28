use std::{collections::HashSet, fs, time::Instant};

use point::Point;

mod point;

fn part_one(input: &[String]) -> String {
    let mut map = Map::parse(input);

    let result = map
        .find_shortest_way()
        .expect("could not find shortest way");

    result.to_string()
}

fn part_two(input: &[String]) -> String {
    let mut map = Map::parse(input);

    let first_trip = map
        .find_shortest_way()
        .expect("could not find shortest way");
    (map.end, map.start) = (map.start, map.end);

    let second_trip = map.find_shortest_way().unwrap();
    (map.end, map.start) = (map.start, map.end);

    let third_trip = map.find_shortest_way().unwrap();

    (first_trip + second_trip + third_trip).to_string()
}

#[derive(Debug)]
struct Map {
    blizzards: Vec<Blizzard>,
    walls: HashSet<Point>,
    start: Point,
    end: Point,
    max: Point,
}

#[derive(Debug)]
struct Blizzard {
    pos: Point,
    dir: Point,
}

impl Map {
    fn parse(input: &[String]) -> Self {
        let points = input.iter().rev().enumerate().flat_map(|(y, line)| {
            line.as_bytes().iter().enumerate().map(move |(x, char)| {
                (
                    Point {
                        x: x as i32,
                        y: y as i32,
                    },
                    char,
                )
            })
        });

        let blizzards: Vec<_> = points
            .clone()
            .filter_map(|(pos, char)| {
                let dir = match char {
                    b'>' => (1, 0).into(),
                    b'<' => (-1, 0).into(),
                    b'^' => (0, 1).into(),
                    b'v' => (0, -1).into(),
                    _ => return None,
                };

                Some(Blizzard { pos, dir })
            })
            .collect();

        let walls: HashSet<_> = points
            .filter(|(_, char)| **char == b'#')
            .map(|(pos, _)| pos)
            .collect();

        let width = input.first().expect("empty input").len() as i32;
        let height = input.len() as i32;

        Self {
            blizzards,
            walls,
            start: (1, height - 1).into(),
            end: (width - 2, 0).into(),
            max: (width - 1, height - 1).into(),
        }
    }

    fn find_shortest_way(&mut self) -> Option<usize> {
        let mut positions = HashSet::new();
        positions.insert(self.start);

        let mut minutes_passed = 0;
        if self.walls.contains(&self.end) {
            println!("wrong walls")
        }

        loop {
            // dbg!(&positions);

            // No more movement to try
            if positions.is_empty() {
                return None;
            }

            let blizzards_pos: HashSet<Point> = self.blizzards.iter().map(|b| b.pos).collect();

            // Arrived in the end
            if positions.contains(&self.end) {
                return Some(minutes_passed);
            }

            positions = positions
                .into_iter()
                .filter(|pos| !blizzards_pos.contains(pos))
                .flat_map(|pos| {
                    [
                        Point { x: 0, y: 0 },
                        Point { x: 1, y: 0 },
                        Point { x: -1, y: 0 },
                        Point { x: 0, y: 1 },
                        Point { x: 0, y: -1 },
                    ]
                    .into_iter()
                    .map(move |dir| pos + dir)
                })
                .filter(|new_pos| !self.walls.contains(new_pos))
                .filter(|new_pos| {
                    (0..=self.max.x).contains(&new_pos.x) && (0..=self.max.y).contains(&new_pos.y)
                })
                .collect();

            self.blizzards = self.move_blizzards();
            minutes_passed += 1;
        }
    }

    fn move_blizzards(&self) -> Vec<Blizzard> {
        self.blizzards
            .iter()
            .map(|blizzard| Blizzard {
                pos: (blizzard.pos - (1, 1).into() + blizzard.dir)
                    .rem_euclid(self.max - (1, 1).into())
                    + (1, 1).into(),
                dir: blizzard.dir,
            })
            .collect()
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
        assert_eq!(result, "18");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(result, "54");
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
