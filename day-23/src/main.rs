use std::{
    collections::{HashMap, HashSet},
    fs,
    time::Instant,
};

use point::Point;

mod point;

fn part_one(input: &[String]) -> String {
    let mut elves = Elves::parse(input);

    for _ in 0..10 {
        elves.do_round();
    }

    elves.empty_groud_tiles().to_string()
}

fn part_two(input: &[String]) -> String {
    let mut elves = Elves::parse(input);

    let mut rounds = 0;
    loop {
        let prev_elves = elves.elves.clone();
        elves.do_round();
        rounds += 1;

        if prev_elves == elves.elves {
            break;
        }
    }

    rounds.to_string()
}

#[derive(Copy, Clone)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

struct Step {
    dir: Direction,
    checks: Vec<Direction>,
}

struct Elves {
    elves: HashSet<Point>,
    steps: Vec<Step>,
    start_step_idx: usize,
}

impl Elves {
    fn parse(input: &[String]) -> Self {
        let elves = input
            .iter()
            .rev()
            .enumerate()
            .flat_map(|(y, line)| {
                line.as_bytes()
                    .iter()
                    .enumerate()
                    .filter(|(_, &c)| c == b'#')
                    .map(move |(x, _)| Point {
                        x: x as i32,
                        y: y as i32,
                    })
            })
            .collect();

        Self {
            elves,
            steps: vec![
                Step {
                    dir: Direction::N,
                    checks: vec![Direction::N, Direction::NE, Direction::NW],
                },
                Step {
                    dir: Direction::S,
                    checks: vec![Direction::S, Direction::SE, Direction::SW],
                },
                Step {
                    dir: Direction::W,
                    checks: vec![Direction::W, Direction::NW, Direction::SW],
                },
                Step {
                    dir: Direction::E,
                    checks: vec![Direction::E, Direction::NE, Direction::SE],
                },
            ],
            start_step_idx: 0,
        }
    }

    fn do_round(&mut self) {
        let mut movements = HashMap::with_capacity(self.elves.len());

        for (cur_pos, proposed) in self
            .elves
            .iter()
            .map(|&elf| (elf, self.propose_next_pos(elf)))
        {
            if let Some(prev_pos) = movements.remove(&proposed) {
                movements.insert(cur_pos, cur_pos);
                movements.insert(prev_pos, prev_pos);
            } else {
                movements.insert(proposed, cur_pos);
            }
        }

        self.elves = movements.keys().copied().collect();
        self.start_step_idx = (1 + self.start_step_idx) % self.steps.len();
    }

    fn propose_next_pos(&self, elf: Point) -> Point {
        if !self.has_neighboor(elf) {
            return elf;
        }

        for step in self.possible_steps() {
            let proposable = step
                .checks
                .iter()
                .map(|&p| elf + p.into())
                .all(|p| !self.elves.contains(&p));

            if proposable {
                return elf + step.dir.into();
            }
        }

        elf
    }

    fn empty_groud_tiles(&self) -> usize {
        let (min, max) = Point::min_max(self.elves.iter()).expect("should contain elves");

        (min.y..=max.y)
            .flat_map(|y| (min.x..=max.x).map(move |x| Point { x, y }))
            .filter(|p| !self.elves.contains(p))
            .count()
    }

    fn has_neighboor(&self, elf: Point) -> bool {
        [
            Direction::E,
            Direction::N,
            Direction::NE,
            Direction::NW,
            Direction::S,
            Direction::SE,
            Direction::SW,
            Direction::W,
        ]
        .into_iter()
        .map(|dir| elf + dir.into())
        .any(|neighboor| self.elves.contains(&neighboor))
    }

    fn possible_steps(&self) -> impl Iterator<Item = &Step> {
        let len = self.steps.len();
        (0..len).map(move |i| &self.steps[(i + self.start_step_idx) % len])
    }
}

impl From<Direction> for Point {
    fn from(val: Direction) -> Self {
        match val {
            Direction::N => Point { x: 0, y: 1 },
            Direction::NE => Point { x: 1, y: 1 },
            Direction::E => Point { x: 1, y: 0 },
            Direction::SE => Point { x: 1, y: -1 },
            Direction::S => Point { x: 0, y: -1 },
            Direction::SW => Point { x: -1, y: -1 },
            Direction::W => Point { x: -1, y: 0 },
            Direction::NW => Point { x: -1, y: 1 },
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
        assert_eq!(result, "110");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(result, "20");
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
