use std::{
    fmt::{Debug, Display},
    fs,
    time::Instant,
};

use point::Point;
use wrap_row::WrapRow;

mod point;
mod wrap_row;

fn part_one(input: &[String]) -> String {
    let mut map = Map::parse(input);
    let instructions = Instruction::parse(input.last().unwrap());

    for instruction in instructions {
        map.apply(instruction, Problem::Part1);
    }

    let x1 = map.cur.y * 1000;
    let x2 = map.cur.x * 4;
    let x3 = map.direction as isize;
    (x1 + x2 + x3).to_string()
}

fn part_two(input: &[String]) -> String {
    let mut map = Map::parse(input);
    let instructions = Instruction::parse(input.last().unwrap());

    for instruction in instructions {
        map.apply(instruction, Problem::Part2);
    }

    let x1 = map.cur.y * 1000;
    let x2 = map.cur.x * 4;
    let x3 = map.direction as isize;
    (x1 + x2 + x3).to_string()
}

#[derive(Debug)]
enum TurnDirection {
    Right,
    Left,
}

#[derive(Debug)]
enum Instruction {
    Walk(u32),
    Turn(TurnDirection),
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

enum Element {
    Tile,
    Wall,
}

enum Problem {
    Part1,
    Part2,
}

#[derive(Debug)]
struct Map {
    rows: WrapRow<WrapRow<Element>>,
    cols: WrapRow<WrapRow<Element>>,
    cur: Point,
    direction: Direction,
}

impl Map {
    fn parse(input: &[String]) -> Self {
        let input: Vec<_> = input
            .iter()
            .take_while(|line| !line.is_empty())
            .map(|line| line.as_bytes())
            .collect();

        let rows: Vec<_> = input
            .iter()
            .map(|line| {
                let start = line.iter().position(|c| *c != b' ').expect("row");
                let row: Vec<_> = line.iter().skip(start).map(|c| Element::from(*c)).collect();

                WrapRow::new(start + 1, row)
            })
            .collect();
        let rows = WrapRow::new(1, rows);

        let cols_count = input.iter().map(|line| line.len()).max().unwrap();
        let cols: Vec<_> = (0..cols_count)
            .map(|col| {
                let col = input.iter().filter_map(|line| line.get(col));

                let start = col.clone().position(|c| *c != b' ').unwrap();
                let col: Vec<_> = col
                    .skip(start)
                    .take_while(|c| **c != b' ')
                    .map(|c| Element::from(*c))
                    .collect();

                WrapRow::new(start + 1, col)
            })
            .collect();

        let cols = WrapRow::new(1, cols);

        Self {
            cur: (rows.get(1).start() as isize, 1).into(),
            direction: Direction::Right,
            cols,
            rows,
        }
    }

    fn apply(&mut self, instruction: Instruction, problem: Problem) {
        match instruction {
            Instruction::Turn(TurnDirection::Right) => {
                self.direction = match self.direction {
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                }
            }
            Instruction::Turn(TurnDirection::Left) => {
                self.direction = match self.direction {
                    Direction::Right => Direction::Up,
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Down => Direction::Right,
                }
            }
            Instruction::Walk(units) => {
                for _ in 0..units {
                    let (next_dir, next_pos) = match problem {
                        Problem::Part1 => self.next_pos_part_1(),
                        Problem::Part2 => self.next_pos_part_2(),
                    };

                    if matches!(self.get(next_pos), Element::Wall) {
                        break;
                    }

                    self.cur = next_pos;
                    self.direction = next_dir;
                }
            }
        }
    }

    fn get(&self, point: Point) -> &Element {
        self.rows.get(point.y).get(point.x)
    }

    fn next_pos_part_1(&self) -> (Direction, Point) {
        let mov = match self.direction {
            Direction::Right | Direction::Down => 1,
            Direction::Left | Direction::Up => -1,
        };

        let mut cur = self.cur;

        match self.direction {
            Direction::Right | Direction::Left => {
                let row = self.rows.get(cur.y);
                let next_x = row.wrap(cur.x + mov) as isize;
                cur.x = next_x;
            }
            Direction::Up | Direction::Down => {
                let col = self.cols.get(cur.x);
                let next_y = col.wrap(cur.y + mov) as isize;

                cur.y = next_y;
            }
        };

        (self.direction, cur)
    }

    fn next_pos_part_2(&self) -> (Direction, Point) {
        let cur = self.cur;

        match (self.direction, self.cur) {
            // 1
            (
                Direction::Down,
                Point {
                    x: 101..=150,
                    y: 50,
                },
            ) => (
                Direction::Left,
                Point {
                    x: 100,
                    y: cur.x - 50,
                },
            ),
            // 2
            (
                Direction::Down,
                Point {
                    x: 51..=100,
                    y: 150,
                },
            ) => (
                Direction::Left,
                Point {
                    x: 50,
                    y: cur.x + 100,
                },
            ),
            // 3
            (Direction::Down, Point { x: 1..=50, y: 200 }) => (
                Direction::Down,
                Point {
                    x: cur.x + 100,
                    y: 1,
                },
            ),

            // 4
            (Direction::Up, Point { x: 1..=50, y: 101 }) => (
                Direction::Right,
                Point {
                    x: 51,
                    y: cur.x + 50,
                },
            ),
            // 5
            (Direction::Up, Point { x: 51..=100, y: 1 }) => (
                Direction::Right,
                Point {
                    x: 1,
                    y: cur.x + 100,
                },
            ),
            // 6
            (Direction::Up, Point { x: 101..=150, y: 1 }) => (
                Direction::Up,
                Point {
                    x: cur.x - 100,
                    y: 200,
                },
            ),

            // 7
            (Direction::Left, Point { x: 1, y: 151..=200 }) => (
                Direction::Down,
                Point {
                    x: cur.y - 100,
                    y: 1,
                },
            ),
            // 8
            (Direction::Left, Point { x: 1, y: 101..=150 }) => (
                Direction::Right,
                Point {
                    x: 51,
                    y: (cur.y - 151).abs(),
                },
            ),
            // 9
            (Direction::Left, Point { x: 51, y: 51..=100 }) => (
                Direction::Down,
                Point {
                    x: cur.y - 50,
                    y: 101,
                },
            ),
            // 10
            (Direction::Left, Point { x: 51, y: 1..=50 }) => (
                Direction::Right,
                Point {
                    x: 1,
                    y: (cur.y - 151).abs(),
                },
            ),

            // 11
            (Direction::Right, Point { x: 150, y: 1..=50 }) => (
                Direction::Left,
                Point {
                    x: 100,
                    y: 151 - cur.y,
                },
            ),
            // 12
            (
                Direction::Right,
                Point {
                    x: 100,
                    y: 51..=100,
                },
            ) => (
                Direction::Up,
                Point {
                    x: cur.y + 50,
                    y: 50,
                },
            ),
            // 13
            (
                Direction::Right,
                Point {
                    x: 100,
                    y: 101..=150,
                },
            ) => (
                Direction::Left,
                Point {
                    x: 150,
                    y: 151 - cur.y,
                },
            ),
            // 14
            (
                Direction::Right,
                Point {
                    x: 50,
                    y: 151..=200,
                },
            ) => (
                Direction::Up,
                Point {
                    x: cur.y - 100,
                    y: 150,
                },
            ),
            _ => self.next_pos_part_1(),
        }
    }
}

impl Instruction {
    fn parse(line: &str) -> Vec<Self> {
        let mut instructions: Vec<Self> = Vec::new();
        let mut line = line.as_bytes();

        loop {
            let turn_idx = line
                .iter()
                .position(|c| c.is_ascii_alphabetic())
                .unwrap_or(line.len());

            let walk_units: u32 = std::str::from_utf8(&line[..turn_idx])
                .unwrap()
                .parse()
                .unwrap();
            let walk_instruction = Self::Walk(walk_units);
            instructions.push(walk_instruction);

            if turn_idx == line.len() {
                break;
            }

            let turn_direction = match line[turn_idx] {
                b'R' => TurnDirection::Right,
                b'L' => TurnDirection::Left,
                c => panic!("invalid direction: '{}' (ascii: {})", char::from(c), c),
            };
            let turn_instruction = Self::Turn(turn_direction);
            instructions.push(turn_instruction);

            line = &line[turn_idx + 1..];
        }

        instructions
    }
}

impl From<u8> for Element {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Tile,
            b'#' => Self::Wall,
            _ => panic!(
                "invalid element character: '{}' (asci: {})",
                char::from(value),
                value
            ),
        }
    }
}

impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Element::Tile => '.',
            Element::Wall => '#',
        };
        write!(f, "{}", char)
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
        assert_eq!(result, "6032");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(result, "5031");
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
        .map(|line| line.to_owned())
        .collect()
}
