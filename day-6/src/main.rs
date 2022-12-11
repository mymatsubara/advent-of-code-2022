use std::{fs, str, time::Instant};

fn part_one(input: &[String]) -> String {
    let (_, result) = distinct_window(input.first().unwrap(), 4);
    result.to_string()
}

fn part_two(input: &[String]) -> String {
    let (_, result) = distinct_window(input.first().unwrap(), 14);
    result.to_string()
}

fn distinct_window(string: &str, size: usize) -> (&str, usize) {
    let datastream = string.as_bytes();

    let mut start = 0;
    let mut window = &datastream[start..0];

    for end in 1..datastream.len() {
        if let Some(i) = window.iter().position(|c| c == &datastream[end]) {
            start += i + 1;
        };

        window = &datastream[start..end + 1];
        if window.len() == size {
            return (str::from_utf8(window).unwrap(), end + 1);
        }
    }

    (str::from_utf8(window).unwrap(), datastream.len())
}

// VVV --- TESTS --- VVV

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = parse_input(true);
        let result = part_one(&input);
        assert_eq!(result, "7");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(result, "19");
    }
}

// VVV --- Lines bellow do not need to be modified --- VVV

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
