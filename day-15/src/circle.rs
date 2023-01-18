use crate::point::Point;

pub struct Circle {
    pub center: Point,
    pub radius: usize,
}

impl Circle {
    pub fn contains(&self, point: Point) -> bool {
        self.center.manhattan_dist(point) <= self.radius
    }
}
