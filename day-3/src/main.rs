use std::{collections::HashSet, fs, time::Instant};

fn part_one(input: &[String]) -> String {
    input
        .iter()
        .filter_map(|line| get_rucksack_duplicate(line))
        .map(|c| to_priority(&c))
        .sum::<usize>()
        .to_string()
}

fn part_two(input: &[String]) -> String {
    input
        .chunks(3)
        .filter_map(get_group_duplicate)
        .map(|c| to_priority(&c))
        .sum::<usize>()
        .to_string()
}

fn to_priority(c: &char) -> usize {
    let n = *c as u8;
    let result = if n <= b'Z' {
        n - b'A' + 27
    } else {
        n - b'a' + 1
    };

    result as usize
}

fn get_rucksack_duplicate(rucksack: &str) -> Option<char> {
    let (first_rucksack, second_rucksack) = rucksack.split_at(rucksack.len() / 2);
    let first_set: HashSet<char> = HashSet::from_iter(first_rucksack.chars());
    let second_set = HashSet::from_iter(second_rucksack.chars());

    first_set
        .intersection(&second_set)
        .into_iter()
        .next()
        .cloned()
}

fn get_group_duplicate(rucksacks: &[String]) -> Option<char> {
    if rucksacks.is_empty() {
        return None;
    }

    let mut duplicates: HashSet<char> = HashSet::from_iter(rucksacks.first().unwrap().chars());

    for rucksack in rucksacks.iter().skip(1) {
        let set = HashSet::from_iter(rucksack.chars().collect::<Vec<char>>());
        duplicates = HashSet::from_iter(duplicates.intersection(&set).copied());
    }

    duplicates.into_iter().next()
}

// --- TESTS ---

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = parse_input(true);
        let result = part_one(&input);
        assert_eq!(result, "157");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(result, "70");
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
