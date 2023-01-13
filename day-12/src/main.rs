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

    climb
        .costs
        .get(climb.end)
        .unwrap()
        .expect("should have reached the end")
        .to_string()
}

fn part_two(input: &[String]) -> String {
    "NOT IMPLEMENTED".to_owned()
}

struct Climb {
    heights: Grid<u8>,
    start: Point,
    end: Point,
    costs: Grid<Option<u16>>,
    max_climb: u8,
}

#[derive(Debug)]
struct Grid<T> {
    width: usize,
    height: usize,
    vec: Vec<T>,
}
#[derive(Debug, Copy, Clone, Default)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        Some(Self {
            x: self.x.checked_sub(rhs.x)?,
            y: self.y.checked_sub(rhs.y)?,
        })
    }
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

impl<T: Default + Copy> Grid<T> {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            vec: vec![Default::default(); width * height],
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
}

impl Climb {
    fn new(grid: Grid<u8>, start: Point, end: Point, max_step: u8) -> Self {
        Self {
            costs: Grid::new(grid.width, grid.height),
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

    fn calc_costs(&mut self) {
        self.visit_neightboors_rec(0, self.start);
    }

    fn visit_neightboors_rec(&mut self, cost: u16, p: Point) {
        match self.costs.get_mut(p) {
            Some(Some(old_cost)) => {
                if cost >= *old_cost {
                    return;
                } else {
                    *old_cost = cost;
                }
            }
            Some(old_cost) => *old_cost = Some(cost),
            None => return,
        };

        let neightboor_cost = cost + 1;
        let cur_height = self.heights.get(p).unwrap();
        let height_threshold = cur_height + self.max_climb;

        // Try to visit bottom neightboor
        let neightboor = p + Point { x: 0, y: 1 };
        if let Some(neightboor_height) = self.heights.get(neightboor) {
            if *neightboor_height <= height_threshold {
                self.visit_neightboors_rec(neightboor_cost, neightboor)
            }
        }

        // Try to visit right neightboor
        let neightboor = p + Point { x: 1, y: 0 };
        if let Some(neightboor_height) = self.heights.get(neightboor) {
            if *neightboor_height <= height_threshold {
                self.visit_neightboors_rec(neightboor_cost, neightboor)
            }
        }

        // Try to visit top neightboor
        if let Some(neightboor) = p.checked_sub(Point { x: 0, y: 1 }) {
            let neightboor_height = self.heights.get(neightboor).unwrap();
            if *neightboor_height <= height_threshold {
                self.visit_neightboors_rec(neightboor_cost, neightboor)
            }
        }

        // Try to visit left neightboor
        if let Some(neightboor) = p.checked_sub(Point { x: 1, y: 0 }) {
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
        assert_eq!(result, "NOT IMPLEMENTED");
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
