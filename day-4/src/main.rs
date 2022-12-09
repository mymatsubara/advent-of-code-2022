use std::{ fs, cmp::{min, max}};

fn main() {
    let lines = parse_input();

    let ranges: Vec<(Range, Range)> = lines
    .iter()
    .filter_map(|line| line.split_once(','))
    .filter_map(|(first, second)| { 
        let (Some(range1), Some(range2)) = (Range::from(first, '-'), Range::from(second, '-')) else {
            return None;
        };
        Some(( range1, range2 ))
    }).collect();

    let part_one = ranges
    .iter()
    .filter(|(range1, range2)|  range1.fully_contains(range2) || range2.fully_contains(range1) )
    .count();

    let part_two = ranges.iter()
    .filter_map(|(range1, range2)| range1.intersection(range2))
    .count();

    println!("Part one: {}", part_one);
    println!("Part two: {:?}", part_two);
}

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn fully_contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn intersection(&self, other: &Range) -> Option<Range> {
        let ( start, end ) = ( max(self.start, other.start), min(self.end, other.end) );
        if start <= end { Some(Range { start, end}) } else { None }
    }

    fn from(string: &str, delimiter: char) -> Option<Range> {
        let Some((first, second)) = string.split_once(delimiter) else { 
            return None;
        };

        let (Ok(start), Ok(end)) = (first.parse::<usize>(), second.parse::<usize>()) else {
            return None;
        };

        Some(Range { start, end })
    }
}

fn parse_input() -> Vec<String> {
    let input = fs::read_to_string("input.txt").expect("input.txt file not found");

    input.lines().map(|line| line.to_owned()).collect()
}
