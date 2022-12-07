use std::fs;

fn main() {
    let all_elves_calories = parse_input();

    let mut elves_calories: Vec<u32> = all_elves_calories
        .iter()
        .map(|calories| calories.iter().sum::<u32>())
        .collect();

    elves_calories.sort_by(|a, b| b.cmp(a));

    let part_one = elves_calories.first().cloned().unwrap_or(0);
    let part_two = elves_calories.iter().take(3).sum::<u32>();

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}

fn parse_input() -> Vec<Vec<u32>> {
    let input = fs::read_to_string("input.txt").expect("input.txt file not found");

    let mut elves_calories: Vec<Vec<u32>> = vec![];
    let mut cur_elve_calories: Vec<u32> = vec![];

    for line in input.split('\n') {
        match line.parse::<u32>() {
            Ok(calories) => cur_elve_calories.push(calories),
            Err(_) => {
                elves_calories.push(cur_elve_calories);
                cur_elve_calories = vec![];
            }
        }
    }

    elves_calories
}
