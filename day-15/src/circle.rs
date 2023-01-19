use crate::point::Point;

#[derive(Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: u32,
}

impl Circle {
    pub fn contains(&self, point: Point) -> bool {
        self.center.manhattan_dist(point) <= self.radius
    }
}
