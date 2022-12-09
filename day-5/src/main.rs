use std::{cmp::max, fs, iter};

use regex::Regex;

fn main() {
    let (stacks, instructions) = parse_input();

    let mut part_one_stacks = stacks.clone();
    for instruction in &instructions {
        instruction.apply_part_one(&mut part_one_stacks);
    }

    let mut part_two_stacks = stacks;
    for instruction in &instructions {
        instruction.apply_part_two(&mut part_two_stacks);
    }

    let part_one: String = part_one_stacks
        .iter()
        .filter_map(|stack| stack.last())
        .cloned()
        .collect();

    let part_two: String = part_two_stacks
        .iter()
        .filter_map(|stack| stack.last())
        .cloned()
        .collect();

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
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

fn parse_input() -> (Vec<Vec<char>>, Vec<Instruction>) {
    let input = fs::read_to_string("input.txt").expect("input.txt file not found");

    let lines: Vec<&str> = input.lines().collect();

    let Some(blank_line_idx) = lines.iter().position(|line| line.is_empty()) else {
        panic!("Invalid input: should contain a blank line separating the stacks and the instructions");
    };

    if blank_line_idx == 0 {
        panic!("Invalid input: should not contain any blank before the stacks")
    }

    let stacks_label_idx = blank_line_idx - 1;
    let stacks_label = &lines[stacks_label_idx];
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

    for line in lines
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
    let instructions: Vec<Instruction> = lines
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
