#[derive(Debug, Copy, Clone)]
pub struct BitSet {
    set: u64,
}

impl BitSet {
    #[inline]
    pub fn new() -> Self {
        Self { set: 0 }
    }

    #[inline]
    pub fn set(&mut self, index: usize) {
        Self::check_bounds(index);
        self.set |= 1 << index;
    }

    #[inline]
    pub fn remove(&mut self, index: usize) {
        Self::check_bounds(index);
        self.set &= !(1 << index);
    }

    #[inline]
    pub fn contains(&self, index: usize) -> bool {
        Self::check_bounds(index);
        self.set & (1 << index) != 0
    }

    #[inline]
    fn check_bounds(index: usize) {
        if index >= 64 {
            panic!("bitset out of bounds");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bit_set() {
        let mut set = BitSet::new();

        assert_eq!(set.contains(0), false);
        assert_eq!(set.contains(63), false);
        assert_eq!(set.contains(30), false);

        set.set(0);
        set.set(63);
        assert_eq!(set.contains(0), true);
        assert_eq!(set.contains(63), true);
        assert_eq!(set.contains(30), false);

        set.set(0);
        set.set(63);
        assert_eq!(set.contains(0), true);
        assert_eq!(set.contains(63), true);
        assert_eq!(set.contains(30), false);

        set.remove(0);
        set.remove(63);
        set.remove(30);
        assert_eq!(set.contains(0), false);
        assert_eq!(set.contains(63), false);
        assert_eq!(set.contains(30), false);
    }
}
