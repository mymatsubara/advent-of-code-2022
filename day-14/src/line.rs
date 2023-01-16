use crate::{coord::Coord, point::Point};

pub struct Line {
    pub start: Coord,
    pub end: Coord,
}

pub struct LineIter {
    start: Coord,
    end: Coord,
    dir: Point,
    cur: Option<Coord>,
}

impl Line {
    pub fn iter(&self) -> LineIter {
        LineIter::new(self)
    }
}

impl LineIter {
    fn new(line: &Line) -> Self {
        Self {
            start: line.start,
            end: line.end,
            cur: None,
            dir: line.end.diff(line.start).dir(),
        }
    }
}

impl Iterator for LineIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur = match self.cur {
            None => Some(self.start),
            Some(cur) if cur == self.end => return None,
            Some(cur) => Some(cur.add_signed_checked(self.dir).expect("overflow occurred")),
        };

        self.cur
    }
}
