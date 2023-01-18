use std::{
    cmp::{max, min},
    collections::HashSet,
    fmt::Display,
    fs,
    time::{Duration, Instant},
};

use day_14::{canvas::Canvas, coord::Coord, grid::Grid, line::Line, point::Point};

fn part_one(input: &[String]) -> String {
    const ANIMATION: bool = false;
    let mut cave = Cave::parse(input, false);

    loop {
        if !cave.step() {
            break;
        }

        if ANIMATION {
            Canvas::display(&cave.grid)
                .overlay(
                    cave.sand_falling.unwrap_or(cave.sand_source),
                    Element::Sand.to_string(),
                )
                .wait(Duration::from_millis(10));
        }
    }

    // println!("{}", cave.grid);

    cave.grid
        .iter()
        .filter(|(_, element)| matches!(element, Element::Sand))
        .count()
        .to_string()
}

fn part_two(input: &[String]) -> String {
    const ANIMATION: bool = false;
    let mut cave = Cave::parse(input, true);

    loop {
        if !cave.step() {
            break;
        }

        if ANIMATION {
            Canvas::display(&cave.grid)
                .overlay(
                    cave.sand_falling.unwrap_or(cave.sand_source),
                    Element::Sand.to_string(),
                )
                .wait(Duration::from_millis(10));
        }
    }

    // println!("{}", cave.grid);

    cave.grid
        .iter()
        .filter(|(_, element)| matches!(element, Element::Sand))
        .count()
        .to_string()
}

#[derive(Clone, Copy)]
enum Element {
    Air,
    Rock,
    Sand,
    SandSource,
}

struct Cave {
    sand_falling: Option<Coord>,
    sand_source: Coord,
    grid: Grid<Element>,
    moves: [Point; 3],
}

impl Cave {
    fn step(&mut self) -> bool {
        match self.sand_falling {
            Some(cur_pos) => {
                for (i, next_pos) in self
                    .moves
                    .iter()
                    .map(|&movement| cur_pos.add_signed_checked(movement))
                    .enumerate()
                {
                    if let None = next_pos {
                        return false;
                    }

                    let element = self.grid.get_mut(next_pos.unwrap());

                    match element {
                        None => return false,
                        Some(Element::Air) => {
                            self.sand_falling = next_pos;
                            break;
                        }
                        Some(Element::SandSource) => {
                            panic!("sand cannot fall on top of sand source")
                        }
                        Some(_) if i == self.moves.len() - 1 => {
                            *self.grid.get_mut(cur_pos).unwrap() = Element::Sand;
                            self.sand_falling = None;
                        }
                        _ => (),
                    }
                }
            }
            None if matches!(self.grid.get(self.sand_source).unwrap(), Element::Sand) => {
                return false
            }
            None => self.sand_falling = Some(self.sand_source),
        };

        true
    }

    fn parse(input: &[String], has_floor: bool) -> Self {
        let lines_coords = Self::parse_coords(input);
        let coords = lines_coords.iter().flat_map(|l| l.iter());
        let x: Vec<_> = coords.clone().map(|coord| coord.x).collect();
        let y: Vec<_> = coords.clone().map(|coord| coord.y).collect();

        let max_x = *x.iter().max().expect("empty input");
        let mut min_x = *x.iter().min().expect("empty input");

        let max_y = *y.iter().max().unwrap();
        let min_y = 0;

        let mut height = max_y - min_y + 1;
        let mut width = max_x - min_x + 1;
        let sand_x = 500;

        let mut lines = Self::to_lines(lines_coords);
        if has_floor {
            height += 2;
            width = max(width, height * 2);
            min_x = min(min_x, sand_x - width / 2);

            let start = (min_x, min_y + height - 1).into();
            let floor = Line {
                start: start,
                end: (start.x + width - 1, start.y).into(),
            };
            lines.push(floor);
        }

        let mut grid = Grid::new_default(width, height, Element::Air);
        for coord in Self::to_rock_coords(lines) {
            *grid.get_mut(coord - (min_x, min_y).into()).unwrap() = Element::Rock;
        }

        let sand_source = (sand_x - min_x, min_y).into();
        *grid.get_mut(sand_source).unwrap() = Element::SandSource;

        Self {
            grid,
            sand_source,
            sand_falling: None,
            moves: [(0, 1).into(), (-1, 1).into(), (1, 1).into()],
        }
    }

    fn parse_coords(input: &[String]) -> Vec<Vec<Coord>> {
        input
            .iter()
            .map(|line| {
                line.split("->")
                    .map(|pair| {
                        pair.trim()
                            .split_once(',')
                            .expect("pair should be separated by comma")
                    })
                    .map(|(x, y)| {
                        (
                            x.parse().expect("x should be usize"),
                            y.parse().expect("y should be an usize"),
                        )
                            .into()
                    })
                    .collect()
            })
            .collect()
    }

    fn to_lines(coords: Vec<Vec<Coord>>) -> Vec<Line> {
        coords
            .iter()
            .flat_map(|rock_structure| {
                rock_structure.windows(2).map(|line| Line {
                    start: line[0],
                    end: line[1],
                })
            })
            .collect()
    }

    fn to_rock_coords(lines: Vec<Line>) -> HashSet<Coord> {
        lines.iter().flat_map(|line| line.iter()).collect()
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Self::Air => '.',
            Self::Rock => '#',
            Self::Sand => 'o',
            Self::SandSource => '+',
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
        assert_eq!(result, "24");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(result, "93");
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
