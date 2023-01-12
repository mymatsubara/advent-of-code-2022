use std::{fs, time::Instant};

fn part_one(input: &[String]) -> String {
    let cpu = exec_instructions(input);

    let mut cycle = 20;
    let mut result = 0;

    while let Some(register) = cpu.history.get(cycle - 1) {
        result += register * cycle as i32;
        cycle += 40;
    }

    result.to_string()
}

fn part_two(input: &[String]) -> String {
    let cpu = exec_instructions(input);

    let crt = Crt {
        width: 40,
        rows: 6,
        sprite_width: 3,
        off_pixel: '.',
        on_pixel: '#',
    };

    crt.print(cpu.history)
}

fn exec_instructions(input: &[String]) -> Cpu {
    let instructions = input
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| Instruction::parse(line).unwrap());

    let mut cpu = Cpu::new();

    for instruction in instructions {
        cpu.execute(instruction);
    }

    cpu
}

struct Crt {
    width: u32,
    rows: u32,
    sprite_width: u32,
    on_pixel: char,
    off_pixel: char,
}

impl Crt {
    fn print(&self, register: Vec<i32>) -> String {
        let sprite_offset = (self.sprite_width / 2) as i32;
        let mut result = String::with_capacity((self.rows * self.width + self.rows) as usize);

        for row in 0..self.rows {
            for col in 0..self.width {
                match register.get((self.width * row + col) as usize) {
                    Some(sprite_pos) => {
                        let sprite_range = sprite_pos - sprite_offset..=sprite_pos + sprite_offset;
                        let pixel = if sprite_range.contains(&(col as i32)) {
                            self.on_pixel
                        } else {
                            self.off_pixel
                        };
                        result.push(pixel)
                    }
                    None => result.push(self.off_pixel),
                }
            }
            let last_row = row == self.rows - 1;
            if !last_row {
                result.push('\n');
            }
        }

        result
    }
}

struct Cpu {
    cycle: u32,
    register: i32,
    history: Vec<i32>,
}

impl Cpu {
    fn new() -> Self {
        Self {
            cycle: 1,
            register: 1,
            history: vec![1],
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        let consumed_cycles = instruction.cycles();
        self.cycle += consumed_cycles;

        for _ in 1..consumed_cycles {
            self.history.push(self.register);
        }

        self.register += match instruction {
            Instruction::Noop => 0,
            Instruction::Addx(value) => value,
        };

        self.history.push(self.register);
    }
}

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}

impl Instruction {
    fn parse(line: &str) -> Option<Instruction> {
        let line = line.trim();
        if line == "noop" {
            Some(Instruction::Noop)
        } else if let Some((_, arg)) = line.trim().split_once(' ') {
            Some(Instruction::Addx(
                arg.parse().expect("should have numeric arg for addx"),
            ))
        } else {
            None
        }
    }

    fn cycles(&self) -> u32 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
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
        assert_eq!(result, "13140");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(
            result,
            r"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
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
    println!("Part two result:\n{result_two} [time: {:.2?}]", elapsed_two);
}

fn parse_input(test: bool) -> Vec<String> {
    let file = if test { "input.test.txt" } else { "input.txt" };

    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("'{file}' not found"))
        .lines()
        .map(|line| line.trim().to_owned())
        .collect()
}
