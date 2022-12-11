use std::{cmp::max, fs, iter, time::Instant};

use regex::Regex;

fn part_one(input: &[String]) -> String {
    let (mut stacks, instructions) = parse_stacks_and_instructions(input);

    for instruction in &instructions {
        instruction.apply_part_one(&mut stacks);
    }

    stacks
        .iter()
        .filter_map(|stack| stack.last())
        .cloned()
        .collect()
}

fn part_two(input: &[String]) -> String {
    let (mut stacks, instructions) = parse_stacks_and_instructions(input);

    for instruction in &instructions {
        instruction.apply_part_two(&mut stacks);
    }

    stacks
        .iter()
        .filter_map(|stack| stack.last())
        .cloned()
        .collect()
}

fn parse_stacks_and_instructions(input: &[String]) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let Some(blank_line_idx) = input.iter().position(|line| line.is_empty()) else {
        panic!("Invalid input: should contain a blank line separating the stacks and the instructions");
    };

    if blank_line_idx == 0 {
        panic!("Invalid input: should not contain any blank before the stacks")
    }

    let stacks_label_idx = blank_line_idx - 1;
    let stacks_label = &input[stacks_label_idx];
    let stacks_idxs: Vec<usize> = stacks_label
        .chars()
        .map(|c| c as u8)
        .enumerate()
        .filter(|(_, c)| *c >= b'1' && *c <= b'9')
        .map(|(i, _)| i)
        .collect();

    let mut stacks: Vec<Vec<char>> = iter::repeat_with(Vec::new)
        .take(stacks_idxs.len())
        .collect();

    for line in input
        .iter()
        .take(stacks_label_idx)
        .rev()
        .map(|line| line.as_bytes())
    {
        for (stack_idx, line_idx) in stacks_idxs.iter().enumerate() {
            let stack_char = line[*line_idx];
            if stack_char != b' ' {
                stacks[stack_idx].push(char::from(stack_char));
            }
        }
    }

    let instruction_regex = Regex::new(r"\D*(\d+)\D*(\d+)\D*(\d+)").expect("Invalid regex");
    let instructions: Vec<Instruction> = input
        .iter()
        .skip(blank_line_idx)
        .filter_map(|line| instruction_regex.captures_iter(line).next())
        .filter_map(|group| {
            Some(Instruction {
                amount: group.get(1)?.as_str().parse::<usize>().unwrap(),
                from: group.get(2)?.as_str().parse::<usize>().unwrap() - 1,
                to: group.get(3)?.as_str().parse::<usize>().unwrap() - 1,
            })
        })
        .collect();

    (stacks, instructions)
}

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn apply_part_one(&self, stacks: &mut [Vec<char>]) {
        self.apply(stacks, true)
    }

    fn apply_part_two(&self, stacks: &mut [Vec<char>]) {
        self.apply(stacks, false)
    }

    fn apply(&self, stacks: &mut [Vec<char>], reverse: bool) {
        let (remain, moved) =
            stacks[self.from].split_at(max(0, stacks[self.from].len() - self.amount));

        let mut new: Vec<char> = moved.to_vec();
        if reverse {
            new.reverse();
        }

        stacks[self.from] = remain.to_vec();
        stacks[self.to].append(&mut new);
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
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(result, "MCD");
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
