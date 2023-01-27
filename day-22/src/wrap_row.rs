#[derive(Debug)]
pub struct WrapRow<T> {
    data: Vec<T>,
    start: usize,
}

impl<T> WrapRow<T> {
    pub fn new(start: usize, data: Vec<T>) -> Self {
        Self { start, data }
    }
}

impl<T> WrapRow<T> {
    pub fn wrap(&self, idx: isize) -> usize {
        let base_idx = (idx - self.start as isize).rem_euclid(self.data.len() as isize) as usize;
        self.start + base_idx
    }

    pub fn get(&self, idx: isize) -> &T {
        let data_idx = (idx - self.start as isize).rem_euclid(self.data.len() as isize);
        &self.data[data_idx as usize]
    }

    pub fn start(&self) -> usize {
        self.start
    }
}
