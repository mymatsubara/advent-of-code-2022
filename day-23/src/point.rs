use std::{
    ops::{Add, AddAssign, Sub, SubAssign},
    slice::Iter,
};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Default)]
pub struct Point<T = i32> {
    pub x: T,
    pub y: T,
}

impl<T> From<(T, T)> for Point<T> {
    fn from(p: (T, T)) -> Self {
        Self { x: p.0, y: p.1 }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.x - rhs.y,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Point {
    pub fn manhattan_dist(self, other: Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    pub fn min_max<'a>(mut points: impl Iterator<Item = &'a Point>) -> Option<(Self, Self)> {
        let mut min = *points.next()?;
        let mut max = min;

        for point in points {
            if point.x < min.x {
                min.x = point.x;
            } else if point.x > max.x {
                max.x = point.x;
            }

            if point.y < min.y {
                min.y = point.y;
            } else if point.y > max.y {
                max.y = point.y;
            }
        }

        Some((min, max))
    }
}
