use std::{
    collections::HashSet,
    fmt::Display,
    fs,
    num::NonZeroUsize,
    ops::{Add, AddAssign, Sub},
    time::Instant,
};

fn part_one(input: &[String]) -> String {
    let instructions = Instruction::parse(input);
    let mut rope = Rope::new(NonZeroUsize::new(2).unwrap());

    for instruction in instructions {
        rope.exec_instruction(instruction);
    }

    rope.tail_visited.len().to_string()
}

fn part_two(input: &[String]) -> String {
    let instructions = Instruction::parse(input);
    let mut rope = Rope::new(NonZeroUsize::new(10).unwrap());

    for instruction in instructions {
        rope.exec_instruction(instruction);
    }

    rope.tail_visited.len().to_string()
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn parse(letter: &str) -> Option<Direction> {
        let direction = match letter {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => return None,
        };

        Some(direction)
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    count: u32,
}

impl Instruction {
    fn parse(input: &[String]) -> Vec<Instruction> {
        input
            .iter()
            .filter_map(|line| Instruction::from_line(line))
            .collect()
    }

    fn from_line(line: &str) -> Option<Instruction> {
        match line.split_once(' ') {
            Some((dir_letter, count)) => Some(Instruction {
                direction: Direction::parse(dir_letter)?,
                count: count.trim().parse().ok()?,
            }),
            None => None,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Default, Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl From<Direction> for Vec2 {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Right => Vec2 { x: 1, y: 0 },
            Direction::Left => Vec2 { x: -1, y: 0 },
            Direction::Up => Vec2 { x: 0, y: 1 },
            Direction::Down => Vec2 { x: 0, y: -1 },
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Default)]
struct Rope {
    nodes: Vec<Vec2>,
    tail_visited: HashSet<Vec2>,
}

impl Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let y: Vec<_> = self.nodes.iter().map(|node| node.y).collect();
        let x: Vec<_> = self.nodes.iter().map(|node| node.x).collect();

        let min_y: i32 = *y.iter().min().unwrap();
        let max_y: i32 = *y.iter().max().unwrap();
        let dim_y = max_y - min_y + 2;

        let min_x: i32 = *x.iter().min().unwrap();
        let max_x: i32 = *x.iter().max().unwrap();
        let dim_x = max_x - min_x + 2;

        let mut grid: Vec<Vec<String>> =
            vec![vec![".".to_string(); dim_x as usize]; dim_y as usize];

        for (i, node) in self.nodes.iter().enumerate() {
            let row = (node.y - min_y) as usize;
            let col = (node.x - min_x) as usize;
            grid[row][col] = i.to_string();
        }

        for row in grid.iter().rev() {
            let row = row.join("");
            writeln!(f, "{row}").unwrap();
        }

        Ok(())
    }
}

impl Rope {
    fn new(nodes: NonZeroUsize) -> Rope {
        Rope {
            nodes: vec![Vec2 { x: 0, y: 0 }; nodes.get()],
            ..Default::default()
        }
    }

    fn exec_instruction(&mut self, instruction: Instruction) {
        for _ in 0..instruction.count {
            self.move_head(instruction.direction);
            self.tail_visited.insert(self.tail());
            // println!("Direction: {:?}\n{}", instruction.direction, self);
        }
    }

    fn tail(&self) -> Vec2 {
        *self.nodes.last().unwrap()
    }

    fn move_head(&mut self, direction: Direction) {
        self.nodes[0] += Vec2::from(direction);

        for i in 1..self.nodes.len() {
            let head = self.nodes[i - 1];
            let tail = &mut self.nodes[i];

            let diff = head - *tail;
            let (dx, dy) = match (diff.x, diff.y) {
                // Copied from https://fasterthanli.me/series/advent-of-code-2022/part-9#part-2-solution-in-your-browser
                // overlapping
                (0, 0) => (0, 0),
                // touching up/left/down/right
                (0, 1) | (1, 0) | (0, -1) | (-1, 0) => (0, 0),
                // touching diagonally
                (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (0, 0),
                // need to move up/left/down/right
                (0, 2) => (0, 1),
                (0, -2) => (0, -1),
                (2, 0) => (1, 0),
                (-2, 0) => (-1, 0),
                // need to move to the right diagonally
                (2, 1) => (1, 1),
                (2, -1) => (1, -1),
                // need to move to the left diagonally
                (-2, 1) => (-1, 1),
                (-2, -1) => (-1, -1),
                // need to move up/down diagonally
                (1, 2) => (1, 1),
                (-1, 2) => (-1, 1),
                (1, -2) => (1, -1),
                (-1, -2) => (-1, -1),
                // 🆕 need to move diagonally
                (-2, -2) => (-1, -1),
                (-2, 2) => (-1, 1),
                (2, -2) => (1, -1),
                (2, 2) => (1, 1),
                _ => panic!("unhandled case: tail - head = {diff:?}"),
            };

            *tail += Vec2 { x: dx, y: dy };
        }
    }
}

// --- TESTS ---

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = parse_input("input.test.txt");
        let result = part_one(&input);
        assert_eq!(result, "13");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input("input-2.test.txt");
        let result = part_two(&input);
        assert_eq!(result, "36");
    }
}

// --- Lines bellow do not need to be modified ---

fn main() {
    let input = parse_input("input.txt");

    let start_one = Instant::now();
    let result_one = part_one(&input);
    let elapsed_one = start_one.elapsed();

    let start_two = Instant::now();
    let result_two = part_two(&input);
    let elapsed_two = start_two.elapsed();

    println!("Part one result: {result_one} [time: {:.2?}]", elapsed_one);
    println!("Part two result: {result_two} [time: {:.2?}]", elapsed_two);
}

fn parse_input(file: &str) -> Vec<String> {
    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("'{file}' not found"))
        .lines()
        .map(|line| line.trim().to_owned())
        .collect()
}
