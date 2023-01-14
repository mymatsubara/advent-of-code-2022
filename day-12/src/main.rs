use std::{
    fmt::{self, Display},
    fs,
    ops::Add,
    time::Instant,
};

fn part_one(input: &[String]) -> String {
    const MAX_CLIMB: u8 = 1;
    let mut climb = Climb::parse(input, MAX_CLIMB).unwrap();

    climb.calc_costs();

    climb.costs.get(climb.end).unwrap().to_string()
}

fn part_two(input: &[String]) -> String {
    const MAX_CLIMB: u8 = 1;
    let mut climb = Climb::parse(input, MAX_CLIMB).unwrap();
    let possible_starts: Vec<Point> = climb
        .heights
        .iter_coords()
        .filter(|p| *climb.heights.get(*p).unwrap() == 0)
        .filter(|p| {
            climb
                .heights
                .neightboors(*p)
                .any(|n| *climb.heights.get(n).unwrap() == 1)
        })
        .collect();

    let result = possible_starts
        .iter()
        .filter_map(|&start| {
            climb.start = start;
            climb.calc_costs()
        })
        .min()
        .unwrap();

    // result.to_string()
    result.to_string()
}

struct Climb {
    heights: Grid<u8>,
    start: Point,
    end: Point,
    costs: Grid<u16>,
    max_climb: u8,
}

#[derive(Debug)]
struct Grid<T> {
    width: usize,
    height: usize,
    vec: Vec<T>,
    default: T,
}
#[derive(Debug, Copy, Clone, Default)]
struct Point {
    x: usize,
    y: usize,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: fmt::Debug> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            writeln!(
                f,
                "{:?}",
                self.vec
                    .iter()
                    .skip(self.width * row)
                    .take(self.width)
                    .collect::<Vec<_>>()
            )?;
        }

        Ok(())
    }
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
    None,
}

struct NeightboorIter {
    height: usize,
    width: usize,
    point: Point,
    direction: Direction,
}

impl NeightboorIter {
    fn new<T>(grid: &Grid<T>, point: Point) -> Self {
        Self {
            height: grid.height,
            width: grid.width,
            direction: Direction::Right,
            point,
        }
    }
}

impl Iterator for NeightboorIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let Point { x, y } = self.point;

        loop {
            match self.direction {
                Direction::Right => {
                    self.direction = Direction::Left;
                    if x >= self.width - 1 {
                        continue;
                    } else {
                        return Some(Point { x: x + 1, y });
                    }
                }
                Direction::Left => {
                    self.direction = Direction::Up;
                    if x == 0 {
                        continue;
                    } else {
                        return Some(Point { x: x - 1, y });
                    }
                }
                Direction::Up => {
                    self.direction = Direction::Down;
                    if y == 0 {
                        continue;
                    } else {
                        return Some(Point { x, y: y - 1 });
                    }
                }
                Direction::Down => {
                    self.direction = Direction::None;
                    if y >= self.height - 1 {
                        continue;
                    } else {
                        return Some(Point { x, y: y + 1 });
                    }
                }
                Direction::None => {
                    return None;
                }
            }
        }
    }
}

impl<T: Default + Copy> Grid<T> {
    fn new(width: usize, height: usize) -> Self {
        Self::new_with_default(width, height, Default::default())
    }

    fn new_with_default(width: usize, height: usize, default: T) -> Self {
        Self {
            width,
            height,
            vec: vec![default; width * height],
            default,
        }
    }

    fn contains(&self, p: Point) -> bool {
        p.x <= self.width && p.y <= self.height
    }

    fn get(&self, p: Point) -> Option<&T> {
        if self.contains(p) {
            self.vec.get(p.y * self.width + p.x)
        } else {
            None
        }
    }

    fn get_mut(&mut self, p: Point) -> Option<&mut T> {
        if self.contains(p) {
            self.vec.get_mut(p.y * self.width + p.x)
        } else {
            None
        }
    }

    fn neightboors(&self, p: Point) -> NeightboorIter {
        NeightboorIter::new(self, p)
    }

    fn iter_coords(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.height).flat_map(|y| (0..self.width).map(move |x| Point { x, y }))
    }

    fn clear(&mut self) {
        self.vec = vec![self.default; self.width * self.height];
    }
}

impl Climb {
    fn new(grid: Grid<u8>, start: Point, end: Point, max_step: u8) -> Self {
        Self {
            costs: Grid::new_with_default(grid.width, grid.height, u16::MAX),
            max_climb: max_step,
            heights: grid,
            start,
            end,
        }
    }

    fn parse(input: &[String], max_step: u8) -> Option<Self> {
        let height = input.len();
        let width = input.first()?.trim().len();

        let mut grid = Grid::new(width, height);
        let mut start = Default::default();
        let mut end = Default::default();

        for y in 0..height {
            for x in 0..width {
                let point = Point { x, y };
                let mut input_cell = input[y].trim().as_bytes()[x];

                if input_cell == b'S' {
                    start = point;
                    input_cell = b'a';
                } else if input_cell == b'E' {
                    end = point;
                    input_cell = b'z';
                }

                let cell = grid.get_mut(point).unwrap();
                *cell = input_cell - b'a';
            }
        }

        Some(Climb::new(grid, start, end, max_step))
    }

    fn calc_costs(&mut self) -> Option<u16> {
        self.costs.clear();

        self.visit_neightboors_rec(0, self.start);

        self.costs.get(self.end).copied()
    }

    fn visit_neightboors_rec(&mut self, cost: u16, p: Point) {
        match self.costs.get_mut(p) {
            Some(old_cost) => {
                if cost >= *old_cost {
                    return;
                } else {
                    *old_cost = cost;
                }
            }
            None => return,
        };

        let neightboor_cost = cost + 1;
        let cur_height = self.heights.get(p).unwrap();
        let height_threshold = cur_height + self.max_climb;

        for neightboor in self.heights.neightboors(p) {
            let neightboor_height = self.heights.get(neightboor).unwrap();
            if *neightboor_height <= height_threshold {
                self.visit_neightboors_rec(neightboor_cost, neightboor)
            }
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
        assert_eq!(result, "31");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(result, "29");
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
