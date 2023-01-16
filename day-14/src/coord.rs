use std::ops::{Add, AddAssign, Sub};

use crate::point::Point;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.y,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.x;
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Coord {
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        if rhs.x > self.x || rhs.y > self.y {
            return None;
        }

        Some(self - rhs)
    }

    pub fn add_signed_checked(self, other: Point) -> Option<Self> {
        Some(Self {
            x: add_signed_checked(self.x, other.x)?,
            y: add_signed_checked(self.y, other.y)?,
        })
    }

    pub fn diff(self, other: Coord) -> Point {
        Point {
            x: self.x as isize - other.x as isize,
            y: self.y as isize - other.y as isize,
        }
    }
}

fn add_signed_checked(a: usize, b: isize) -> Option<usize> {
    if b >= 0 {
        a.checked_add(b as _)
    } else {
        a.checked_sub(-b as _)
    }
}

impl From<(usize, usize)> for Coord {
    fn from(coords: (usize, usize)) -> Self {
        Self {
            x: coords.0,
            y: coords.1,
        }
    }
}
