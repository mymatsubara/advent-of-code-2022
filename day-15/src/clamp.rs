use std::ops::{Range, RangeInclusive};

pub trait Clamp<I, T = Self> {
    fn clamp(self, range: &RangeInclusive<I>) -> T;
}

impl<I: Ord + Copy> Clamp<I> for RangeInclusive<I> {
    fn clamp(self, range: &RangeInclusive<I>) -> Self {
        *range.start().max(self.start())..=*range.end().min(self.end())
    }
}

impl<I: Ord + Copy> Clamp<I> for I {
    fn clamp(self, range: &RangeInclusive<I>) -> Self {
        if self < *range.start() {
            *range.start()
        } else if self > *range.end() {
            *range.end()
        } else {
            self
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn clamp_range() {
        assert_eq!((-10..=10).clamp(&(0..=5)), 0..=5);
        assert_eq!((1..=10).clamp(&(0..=5)), 1..=5);
        assert_eq!((-10..=4).clamp(&(0..=5)), 0..=4);
        assert_eq!((1..=4).clamp(&(0..=5)), 1..=4);
        assert_eq!((0..=5).clamp(&(0..=5)), 0..=5);
    }
}
