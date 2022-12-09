use std::{collections::HashSet, fs};

fn main() {
    let lines = parse_input();

    let part_one = lines
        .iter()
        .filter_map(|line| get_rucksack_duplicate(line))
        .map(|c| to_priority(&c))
        .sum::<usize>();

    let part_two = lines
        .chunks(3)
        .filter_map(get_group_duplicate)
        .map(|c| to_priority(&c))
        .sum::<usize>();

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
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

fn parse_input() -> Vec<String> {
    let input = fs::read_to_string("input.txt").expect("input.txt file not found");

    input.lines().map(|line| line.to_owned()).collect()
}
