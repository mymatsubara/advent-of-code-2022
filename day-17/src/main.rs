use std::{cmp::max, collections::HashSet, fmt::Display, fs, ops::RangeInclusive, time::Instant};

use day_17::{grid::Grid, point::Point};

fn part_one(input: &[String]) -> String {
    let jet_pattern = Direction::parse_jet_pattern(input.first().expect("empty input"));
    let rock_kinds = rock_kinds();

    let mut chamber = Chamber::new(jet_pattern, 7, 2022 * 4);

    for rock_kind in rock_kinds.iter().cycle().take(2022) {
        chamber.drop_rock(rock_kind);
    }

    chamber.height.to_string()
}

/// It is not feasible to simulate the fall of rocks 1_000_000_000_000.
/// However, we can try to find a fall pattern in which `n` rocks drop will result a increase in height of `y`
/// If we find such fall pattern, we can calculate part 2 analitically
/// 
/// NOTE: the code bellow is crap, but it works lol
fn part_two(input: &[String]) -> String {
    let jet_pattern = Direction::parse_jet_pattern(input.first().expect("empty input"));
    let rock_kinds = rock_kinds();

    // Auxiliar vectors used to find a fall pattern
    let rocks_to_drop = jet_pattern.len() * 10;
    let mut heights = Vec::with_capacity(rocks_to_drop);
    let mut historic: Vec<Vec<(u32, u32)>> = vec![vec![]; jet_pattern.len()];

    let mut chamber = Chamber::new(jet_pattern, 7, (rocks_to_drop * 4) as u32);

    for rock_kind in rock_kinds.iter().cycle().take(rocks_to_drop) {
        heights.push(chamber.height);
        historic[chamber.cur_jet_pattern].push((chamber.rocks_dropped as u32, chamber.height));

        chamber.drop_rock(rock_kind);
    }

    // Find a fall pattern in which a number of dropped `rocks` result in a increase of `height`
    // For each jet pattern we record the height and the number of dropped rocks
    let fall_pattern: FallPattern = historic
        .iter()
        .filter(|e| e.len() > 2) // Ignore jet patterns which do not repeat
        .filter_map(|rocks_heights| {
            // List possible
            let patterns: HashSet<_> = rocks_heights
                .windows(2)
                .map(|e| {
                    let (n1, h1) = e[0];
                    let (n2, h2) = e[1];
                    (n2 - n1, h2 - h1)
                })
                .collect();

            if patterns.len() > 1 {
                return None;
            }

            let (start_rock, start_height) = *rocks_heights.first()?;
            let (rocks, height) = *patterns.iter().next()?;

            let start_rock = start_rock as usize;
            let start_height = start_height as usize;
            let rocks = rocks as usize;
            let height = height as usize;

            Some(FallPattern {
                start_height,
                start_rock,
                rocks,
                height,
                pattern_heights: &heights[start_rock..start_rock + rocks],
                start_heights: &heights[0..start_rock],
            })
        })
        .next()
        .expect("no fall pattern found");

    fall_pattern.calculate_height(1_000_000_000_000).to_string()
}

fn rock_kinds() -> Vec<RockKind> {
    vec![
        // ####
        RockKind::new(
            vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 3, y: 0 },
            ],
            "-",
        ),
        // .#.
        // ###
        // .#.
        RockKind::new(
            vec![
                Point { x: 1, y: 0 },
                Point { x: 0, y: -1 },
                Point { x: 1, y: -1 },
                Point { x: 2, y: -1 },
                Point { x: 1, y: -2 },
            ],
            "+",
        ),
        // ..#
        // ..#
        // ###
        RockKind::new(
            vec![
                Point { x: 2, y: 0 },
                Point { x: 2, y: -1 },
                Point { x: 0, y: -2 },
                Point { x: 1, y: -2 },
                Point { x: 2, y: -2 },
            ],
            "L",
        ),
        // #
        // #
        // #
        // #
        RockKind::new(
            vec![
                Point { x: 0, y: 0 },
                Point { x: 0, y: -1 },
                Point { x: 0, y: -2 },
                Point { x: 0, y: -3 },
            ],
            "|",
        ),
        // ##
        // ##
        RockKind::new(
            vec![
                Point { x: 0, y: 0 },
                Point { x: 0, y: -1 },
                Point { x: 1, y: 0 },
                Point { x: 1, y: -1 },
            ],
            "SQUARE",
        ),
    ]
}

#[derive(Debug)]
struct RockKind {
    points: Vec<Point>,
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
    width: u32,
    height: u32,
    name: String,
}

#[derive(Copy, Clone, Debug)]
struct Rock<'a> {
    pos: Point,
    kind: &'a RockKind,
}

#[derive(Debug, Clone, Copy)]
enum Element {
    Air,
    Rock,
}

#[derive(Copy, Clone)]
enum Direction {
    Right,
    Left,
}

struct Chamber {
    rocks_dropped: usize,
    width: u32,
    height: u32,
    grid: Grid<Element>,
    jet_pattern: Vec<Direction>,
    cur_jet_pattern: usize,
}

#[derive(Debug)]
struct FallPattern<'a> {
    start_height: usize,
    start_rock: usize,
    height: usize,
    rocks: usize,
    pattern_heights: &'a [u32],
    start_heights: &'a [u32],
}

impl<'a> FallPattern<'a> {
    fn calculate_height(&self, rocks_dropped: usize) -> usize {
        if rocks_dropped < self.start_rock {
            return self.start_heights[rocks_dropped] as usize;
        }

        let repeat = (rocks_dropped - self.start_rock) / self.rocks;
        let remainder = (rocks_dropped - self.start_rock) % self.rocks;

        repeat * self.height + self.pattern_heights[remainder] as usize
    }
}

impl Chamber {
    fn new(jet_pattern: Vec<Direction>, width: u32, height: u32) -> Chamber {
        let mut grid = Grid::new_default(0..=width as i32 - 1, 0..=height as i32 - 1, Element::Air);

        // Set rock floor
        for x in 0..width {
            *grid.get_mut((x as _, 0).into()).unwrap() = Element::Rock;
        }

        Chamber {
            height: 0,
            cur_jet_pattern: 0,
            rocks_dropped: 0,
            width,
            grid,
            jet_pattern,
        }
    }

    fn drop_rock(&mut self, kind: &RockKind) {
        let mut rock = Rock {
            pos: (2, self.height as i32 + 4 - kind.y_range.start()).into(),
            kind,
        };

        loop {
            if let Some(moved) = self.apply_jet(rock) {
                rock = moved;
            }

            match self.try_move_down(rock) {
                Some(moved) => rock = moved,
                None => {
                    for rock_point in rock.points() {
                        let grid_element = self
                            .grid
                            .get_mut(rock_point)
                            .expect("rock should be inside the grid");
                        debug_assert!(matches!(grid_element, Element::Air));
                        *grid_element = Element::Rock;
                    }

                    break;
                }
            }
        }

        self.rocks_dropped += 1;
        self.height = max((rock.pos.y + rock.kind.y_range.end()) as u32, self.height);
    }

    fn apply_jet<'a>(&mut self, mut rock: Rock<'a>) -> Option<Rock<'a>> {
        rock.pos.x += match self.next_jet() {
            Direction::Right => 1,
            Direction::Left => -1,
        };

        if self.is_valid_rock(&rock) {
            Some(rock)
        } else {
            None
        }
    }

    fn try_move_down<'a>(&self, mut rock: Rock<'a>) -> Option<Rock<'a>> {
        rock.pos.y -= 1;

        if self.is_valid_rock(&rock) {
            Some(rock)
        } else {
            None
        }
    }

    fn is_valid_rock(&self, &rock: &Rock) -> bool {
        rock.points()
            .all(|p| matches!(self.grid.get(p), Some(Element::Air)))
    }

    fn next_jet(&mut self) -> Direction {
        let direction = self.jet_pattern.get(self.cur_jet_pattern).unwrap();

        self.cur_jet_pattern += 1;
        self.cur_jet_pattern %= self.jet_pattern.len();

        *direction
    }
}

impl RockKind {
    fn new(body: Vec<Point>, name: &str) -> Self {
        let (min, max) = Point::min_max(body.iter()).unwrap_or_default();
        let x_range = min.x..=max.x;
        let y_range = min.y..=max.y;

        let width = x_range.end().abs_diff(*x_range.start()) + 1;
        let height = y_range.end().abs_diff(*y_range.start()) + 1;

        Self {
            points: body,
            width,
            height,
            x_range,
            y_range,
            name: name.to_string(),
        }
    }
}

impl<'a> Rock<'a> {
    fn points(&self) -> impl Iterator<Item = Point> + '_ {
        self.kind.points.iter().map(|p| self.pos + *p)
    }
}

impl Direction {
    fn parse_jet_pattern(input: &str) -> Vec<Direction> {
        input
            .chars()
            .map(|c| match c {
                '>' => Direction::Right,
                '<' => Direction::Left,
                _ => panic!("invalid input"),
            })
            .collect()
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Element::Rock => '#',
            Element::Air => '.',
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
        assert_eq!(result, "3068");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(result, "1514285714288");
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
