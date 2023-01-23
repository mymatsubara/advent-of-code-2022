use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point3<T = i32> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> From<(T, T, T)> for Point3<T> {
    fn from(p: (T, T, T)) -> Self {
        Self {
            x: p.0,
            y: p.1,
            z: p.2,
        }
    }
}

impl<T: Add> Add for Point3<T> {
    type Output = Point3<T::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: AddAssign> AddAssign for Point3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub> Sub for Point3<T> {
    type Output = Point3<T::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: SubAssign> SubAssign for Point3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Ord + Copy> Point3<T> {
    pub fn min_max(mut iter: impl Iterator<Item = Self>) -> Option<(Self, Self)> {
        let mut min = iter.next()?;
        let mut max = min;

        for p in iter {
            min.x = p.x.min(min.x);
            min.y = p.y.min(min.y);
            min.z = p.z.min(min.z);

            max.x = p.x.max(max.x);
            max.y = p.y.max(max.y);
            max.z = p.z.max(max.z);
        }

        Some((min, max))
    }
}
