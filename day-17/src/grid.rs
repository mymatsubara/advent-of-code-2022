use std::{
    fmt::{self, Display},
    ops::RangeInclusive,
};

use crate::point::Point;

pub struct Grid<T> {
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
    width: u32,
    height: u32,
    data: Vec<T>,
}

impl<T: Default + Clone> Grid<T> {
    pub fn new(x_range: RangeInclusive<i32>, y_range: RangeInclusive<i32>) -> Self {
        Grid::new_default(x_range, y_range, Default::default())
    }
}

impl<T: Clone> Grid<T> {
    pub fn new_default(
        x_range: RangeInclusive<i32>,
        y_range: RangeInclusive<i32>,
        default: T,
    ) -> Self {
        if x_range.is_empty() || y_range.is_empty() {
            panic!("grid cannot have empty ranges");
        }

        let width = x_range.end().abs_diff(*x_range.start()) + 1;
        let height = y_range.end().abs_diff(*y_range.start()) + 1;

        Self {
            data: vec![default; (width * height) as _],
            width,
            height,
            x_range,
            y_range,
        }
    }
}

impl<T> Grid<T> {
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    pub fn x_range(&self) -> &RangeInclusive<i32> {
        &self.x_range
    }

    #[inline]
    pub fn y_range(&self) -> &RangeInclusive<i32> {
        &self.y_range
    }

    #[inline]
    pub fn in_bounds(&self, p: Point) -> bool {
        self.x_range.contains(&p.x) && self.y_range.contains(&p.y)
    }

    #[inline]
    pub fn get(&self, p: Point) -> Option<&T> {
        if !self.in_bounds(p) {
            return None;
        }

        Some(self.get_unchecked(p))
    }

    #[inline]
    pub fn get_unchecked(&self, p: Point) -> &T {
        &self.data[self.index(p)]
    }

    #[inline]
    pub fn get_mut(&mut self, p: Point) -> Option<&mut T> {
        if !self.in_bounds(p) {
            return None;
        }

        Some(self.get_mut_unchecked(p))
    }

    #[inline]
    pub fn get_mut_unchecked(&mut self, p: Point) -> &mut T {
        let index = self.index(p);
        &mut self.data[index]
    }

    #[inline]
    pub fn rows(&self) -> impl DoubleEndedIterator<Item = &[T]> {
        self.data.chunks(self.width as _)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Point, &T)> {
        let x_range = self.x_range.clone();
        let y_range = self.y_range.clone();

        y_range
            .clone()
            .flat_map(move |y| x_range.clone().map(move |x| Point { x, y }))
            .map(|point| (point, self.get_unchecked(point)))
    }

    #[inline]
    fn index(&self, p: Point) -> usize {
        let rel_x = (p.x - self.x_range.start()) as usize;
        let rel_y = (p.y - self.y_range.start()) as usize;

        rel_x + rel_y * self.width as usize
    }
}

impl<T: Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.rows().rev() {
            writeln!(
                f,
                "{}",
                row.iter().map(|e| e.to_string()).collect::<String>()
            )?
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get() {
        let x_range = -10..=10;
        let y_range = -5..=5;

        let grid: Grid<u8> = Grid::new(x_range.clone(), y_range.clone());

        // Inbound values
        for x in x_range.clone() {
            for y in y_range.clone() {
                assert_eq!(grid.get((x, y).into()), Some(&0));
            }
        }

        // Outbound values
        assert_eq!(
            grid.get((x_range.start() - 1, y_range.start() - 1).into()),
            None
        );
        assert_eq!(
            grid.get((x_range.end() + 1, y_range.end() + 1).into()),
            None
        );
    }

    #[test]
    fn other_tests() {
        let mut grid: Grid<u8> = Grid::new(0..=1, 0..=1);

        // Setting values
        // Grid:
        // [1, 2]
        // [3, 4]
        *grid.get_mut((0, 0).into()).unwrap() = 1;
        *grid.get_mut((1, 0).into()).unwrap() = 2;
        *grid.get_mut((0, 1).into()).unwrap() = 3;
        *grid.get_mut((1, 1).into()).unwrap() = 4;

        // Get unchecked
        assert_eq!(grid.get_unchecked((0, 0).into()), &1);
        assert_eq!(grid.get_unchecked((1, 0).into()), &2);
        assert_eq!(grid.get_unchecked((0, 1).into()), &3);
        assert_eq!(grid.get_unchecked((1, 1).into()), &4);

        // Iterator
        assert_eq!(
            grid.iter().collect::<Vec<_>>(),
            vec![
                ((0, 0).into(), &1),
                ((1, 0).into(), &2),
                ((0, 1).into(), &3),
                ((1, 1).into(), &4)
            ]
        );

        // Printing
        assert_eq!(format!("{grid}"), "34\n12\n")
    }
}
