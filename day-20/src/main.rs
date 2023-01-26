use std::{collections::HashSet, fs, time::Instant};

fn part_one(input: &[String]) -> String {
    let encrypted = parse(input);

    let mixed = mix(encrypted);

    let zero_idx = mixed.iter().position(|(_, value)| *value == 0).unwrap();

    let v1 = mixed[(zero_idx + 1000) % mixed.len()].1;
    let v2 = mixed[(zero_idx + 2000) % mixed.len()].1;
    let v3 = mixed[(zero_idx + 3000) % mixed.len()].1;

    (v1 + v2 + v3).to_string()
}

fn part_two(input: &[String]) -> String {
    const DECRYPTION_KEY: i64 = 811_589_153;
    const MIXES_COUNT: u8 = 10;

    let encrypted = parse(input);
    let mut mixed: Vec<_> = encrypted
        .into_iter()
        .map(|(idx, value)| (idx, value * DECRYPTION_KEY))
        .collect();

    for _ in 0..MIXES_COUNT {
        mixed = mix(mixed);
    }

    let zero_idx = mixed.iter().position(|(_, value)| *value == 0).unwrap();

    let v1 = mixed[(zero_idx + 1000) % mixed.len()].1;
    let v2 = mixed[(zero_idx + 2000) % mixed.len()].1;
    let v3 = mixed[(zero_idx + 3000) % mixed.len()].1;

    (v1 + v2 + v3).to_string()
}

fn mix(encrypted: Vec<(usize, i64)>) -> Vec<(usize, i64)> {
    let mut mixed = encrypted;
    let len = mixed.len() as i64;

    for n in 0..mixed.len() {
        let old_idx = mixed.iter().position(|(idx, _)| n == *idx).unwrap();
        let offset = mixed[old_idx].1;

        let new_idx = old_idx as i64 + offset;
        let new_idx = new_idx.rem_euclid(len - 1);

        // Move element from `old_idx` to `new_idx`
        let removed = mixed.remove(old_idx);
        mixed.insert(new_idx as usize, removed);
    }

    mixed
}

fn parse(input: &[String]) -> Vec<(usize, i64)> {
    input
        .iter()
        .map(|line| {
            line.trim()
                .parse::<i64>()
                .expect("input line should have i64 numbers")
        })
        .enumerate()
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
        assert_eq!(result, "3");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(result, "1623178306");
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
        .map(|line| line.trim().to_owned())
        .collect()
}
