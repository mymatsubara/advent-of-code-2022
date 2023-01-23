use std::ops::{Index, RangeInclusive, Sub};

use crate::point3::Point3;

#[derive(Debug)]
pub struct Ranges3 {
    pub x: RangeInclusive<i32>,
    pub y: RangeInclusive<i32>,
    pub z: RangeInclusive<i32>,
}

#[derive(Debug)]
pub struct Grid3<T> {
    ranges: Ranges3,
    data: Vec<T>,
    width: usize,
    height: usize,
    depth: usize,
}

impl<T: Copy + Default> Grid3<T> {
    pub fn new(ranges: Ranges3) -> Self {
        Grid3::new_set_default(ranges, Default::default())
    }
}

impl<T: Copy> Grid3<T> {
    pub fn new_set_default(ranges: Ranges3, default: T) -> Self {
        let (width, height, depth) = (ranges.x.len(), ranges.y.len(), ranges.z.len());

        Self {
            width,
            height,
            depth,
            ranges,
            data: vec![default; width * height * depth],
        }
    }

    #[inline]
    pub fn get(&self, p: Point3) -> Option<&T> {
        if !self.in_bounds(p) {
            return None;
        }

        Some(self.get_unchecked(p))
    }

    #[inline]
    pub fn get_unchecked(&self, p: Point3) -> &T {
        &self.data[self.index(p)]
    }

    #[inline]
    pub fn get_mut(&mut self, p: Point3) -> Option<&mut T> {
        if !self.in_bounds(p) {
            return None;
        }

        Some(self.get_mut_unchecked(p))
    }

    #[inline]
    pub fn get_mut_unchecked(&mut self, p: Point3) -> &mut T {
        let index = self.index(p);
        &mut self.data[index]
    }

    #[inline]
    fn index(&self, p: Point3) -> usize {
        let ranges = &self.ranges;
        let x = (p.x - ranges.x.start()) as usize;
        let y = (p.y - ranges.y.start()) as usize;
        let z = (p.z - ranges.z.start()) as usize;

        x + y * self.width + z * self.width * self.height
    }

    #[inline]
    fn in_bounds(&self, p: Point3) -> bool {
        let ranges = &self.ranges;
        ranges.x.contains(&p.x) && ranges.y.contains(&p.y) && ranges.z.contains(&p.z)
    }

    pub fn iter_points(&self) -> impl Iterator<Item = Point3> + '_ {
        self.ranges.z.clone().flat_map(move |z| {
            self.ranges
                .y
                .clone()
                .flat_map(move |y| self.ranges.x.clone().map(move |x| Point3 { x, y, z }))
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = (Point3, &T)> + '_ {
        self.iter_points().map(|p| (p, self.get_unchecked(p)))
    }
}

impl<T> Grid3<T> {
    pub fn ranges(&self) -> &Ranges3 {
        &self.ranges
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn depth(&self) -> usize {
        self.depth
    }
}

trait Len {
    fn len(&self) -> usize;
}

impl Len for RangeInclusive<i32> {
    fn len(&self) -> usize {
        (self.end().abs_diff(*self.start()) + 1) as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_usage() {
        let ranges = Ranges3 {
            x: 0..=1,
            y: 0..=1,
            z: 0..=1,
        };

        let mut grid: Grid3<i32> = Grid3::new(ranges);

        assert_eq!(grid.get((0, 0, 0).into()), Some(&0));
        assert_eq!(grid.get((1, 0, 0).into()), Some(&0));
        assert_eq!(grid.get((0, 1, 0).into()), Some(&0));
        assert_eq!(grid.get((1, 1, 0).into()), Some(&0));
        assert_eq!(grid.get((0, 0, 1).into()), Some(&0));
        assert_eq!(grid.get((1, 0, 1).into()), Some(&0));
        assert_eq!(grid.get((1, 1, 1).into()), Some(&0));
        assert_eq!(grid.get((-1, 0, 0).into()), None);
        assert_eq!(grid.get((0, -1, 0).into()), None);
        assert_eq!(grid.get((0, 0, -1).into()), None);

        *grid.get_mut((1, 1, 0).into()).unwrap() = 2;
        assert_eq!(grid.get((1, 1, 0).into()), Some(&2));
    }
}
