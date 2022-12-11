use std::{fs, time::Instant};

fn part_one(input: &[String]) -> String {
    let calories = to_calories(input);
    calories.iter().max().unwrap().to_string()
}

fn part_two(input: &[String]) -> String {
    let mut calories = to_calories(input);
    calories.sort_by(|a, b| b.cmp(a));
    calories.iter().take(3).sum::<usize>().to_string()
}

fn to_calories(input: &[String]) -> Vec<usize> {
    input
        .split(|line| line.is_empty())
        .map(|elve| {
            elve.iter()
                .filter_map(|line| line.parse::<usize>().ok())
                .sum::<usize>()
        })
        .collect()
}

// --- TESTS ---

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = parse_input(true);
        let result = part_one(&input);
        assert_eq!(result, "24000");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(result, "45000");
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
