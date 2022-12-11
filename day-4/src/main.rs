use std::{
    cmp::{max, min},
    fs,
    time::Instant,
};

fn part_one(input: &[String]) -> String {
    let ranges = map_to_ranges(input);
    ranges
        .iter()
        .filter(|(range1, range2)| range1.fully_contains(range2) || range2.fully_contains(range1))
        .count()
        .to_string()
}

fn part_two(input: &[String]) -> String {
    let ranges = map_to_ranges(input);
    ranges
        .iter()
        .filter_map(|(range1, range2)| range1.intersection(range2))
        .count()
        .to_string()
}

fn map_to_ranges(input: &[String]) -> Vec<(Range, Range)> {
    input
    .iter()
    .filter_map(|line| line.split_once(','))
    .filter_map(|(first, second)| { 
        let (Some(range1), Some(range2)) = (Range::from(first, '-'), Range::from(second, '-')) else {
            return None;
        };
        Some(( range1, range2 ))
    }).collect()
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
        let (start, end) = (max(self.start, other.start), min(self.end, other.end));
        if start <= end {
            Some(Range { start, end })
        } else {
            None
        }
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

// --- TESTS ---

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = parse_input(true);
        let result = part_one(&input);
        assert_eq!(result, "2");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(result, "4");
    }
}

// --- Lines bellow do not need to be modified ---

fn main() {
    let input = parse_input(false);

    let start_one = Instant::now();
    let result_one = part_one(&input);
    let elapsed_one = start_one.elapsed();

    let start_two = Instant::now();
    let result_two = part_two(&input);
    let elapsed_two = start_two.elapsed();

    println!("Part one result: {result_one} [time: {:.2?}]", elapsed_one);
    println!("Part two result: {result_two} [time: {:.2?}]", elapsed_two);
}

fn parse_input(test: bool) -> Vec<String> {
    let file = if test { "input.test.txt" } else { "input.txt" };

    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("'{file}' not found"))
        .lines()
        .map(|line| line.to_owned())
        .collect()
}
