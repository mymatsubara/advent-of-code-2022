#[derive(Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl From<(isize, isize)> for Point {
    fn from(p: (isize, isize)) -> Self {
        Self { x: p.0, y: p.1 }
    }
}

impl Point {
    pub fn dir(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
}
