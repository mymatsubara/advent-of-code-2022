use std::fmt::{self, Display};

use crate::{coord::Coord, point::Point};

pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Default + Clone> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Grid::new_default(width, height, Default::default())
    }
}

impl<T: Clone> Grid<T> {
    pub fn new_default(width: usize, height: usize, default: T) -> Self {
        Self {
            data: vec![default; width * height],
            width,
            height,
        }
    }
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn in_bounds(&self, c: Coord) -> bool {
        c.x < self.width && c.y < self.height
    }

    pub fn get(&self, c: Coord) -> Option<&T> {
        if !self.in_bounds(c) {
            return None;
        }

        Some(self.unchecked_get(c))
    }

    fn unchecked_get(&self, c: Coord) -> &T {
        &self.data[c.y * self.width + c.x]
    }

    pub fn get_mut(&mut self, c: Coord) -> Option<&mut T> {
        if !self.in_bounds(c) {
            return None;
        }

        Some(self.unchecked_get_mut(c))
    }

    fn unchecked_get_mut(&mut self, c: Coord) -> &mut T {
        &mut self.data[c.y * self.width + c.x]
    }

    pub fn walk(&self, c: Coord, p: Point) -> Option<Coord> {
        if let Some(target) = c.add_signed_checked(p) {
            if self.in_bounds(target) {
                return Some(target);
            }
        }

        None
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.data.chunks(self.width)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Coord, &T)> {
        (0..self.width)
            .flat_map(|x| (0..self.height).map(move |y| Coord { x, y }))
            .map(|coord| (coord, self.unchecked_get(coord)))
    }
}

impl<T: Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.rows() {
            writeln!(
                f,
                "{}",
                row.iter().map(|e| e.to_string()).collect::<String>()
            )?
        }

        Ok(())
    }
}
