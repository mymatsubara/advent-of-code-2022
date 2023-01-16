use std::{fmt::Display, thread, time::Duration};

use crate::{coord::Coord, grid::Grid};

pub struct Canvas<'a, T> {
    grid: &'a Grid<T>,
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl<'a, T: Display> Canvas<'a, T> {
    pub fn display(grid: &'a Grid<T>) -> Self {
        Self { grid }.print_canvas()
    }

    pub fn wait(self, duration: Duration) -> Self {
        thread::sleep(duration);
        self
    }

    pub fn overlay<D: Display>(self, coord: Coord, c: D) -> Self {
        let this = self.seek_coord(coord);
        let c = c.to_string();
        print!("{}", c);
        this.seek_coord_back(coord + (0, 0).into())
    }

    fn seek_coord(self, coord: Coord) -> Self {
        self.seek(Direction::Down, coord.y)
            .seek(Direction::Right, coord.x)
    }

    fn seek_coord_back(self, coord: Coord) -> Self {
        self.seek(Direction::Up, coord.y)
            .seek(Direction::Left, coord.x)
    }

    fn seek(self, direction: Direction, count: usize) -> Self {
        print!(
            "{}",
            if count == 0 {
                String::new()
            } else {
                format!("\x1b[{}{}", count, direction.to_ascii_espace())
            }
        );
        self
    }

    fn print_canvas(self) -> Self {
        print!("\x1b[s");
        println!("{}", self.grid);
        self.seek_start()
    }

    fn seek_start(self) -> Self {
        let height = self.grid.height();
        let width = self.grid.width();
        self.seek(Direction::Up, height + 1)
            .seek(Direction::Left, width + 1)
    }

    pub fn seek_end(grid: &'a Grid<T>) {
        println!("\x1b[{}B", grid.height() + 1)
    }
}

impl Direction {
    fn to_ascii_espace(self) -> char {
        match self {
            Direction::Up => 'A',
            Direction::Down => 'B',
            Direction::Right => 'C',
            Direction::Left => 'D',
        }
    }
}
