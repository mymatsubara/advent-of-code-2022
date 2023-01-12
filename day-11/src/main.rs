use std::{
    fmt::{self},
    fs,
    time::Instant,
};

fn part_one(input: &[String]) -> String {
    const ROUNDS: u8 = 20;
    const RELIEF_FACTOR: Item = 3;
    let mut monkey_group = MonkeyGroup::parse(input, RELIEF_FACTOR);

    for _ in 0..ROUNDS {
        monkey_group.exec_round();
    }

    monkey_group.monkey_business().to_string()
}

fn part_two(input: &[String]) -> String {
    const ROUNDS: u16 = 10_000;
    const RELIEF_FACTOR: Item = 1;
    let mut monkey_group = MonkeyGroup::parse(input, RELIEF_FACTOR);

    for _ in 0..ROUNDS {
        monkey_group.exec_round();
    }

    monkey_group.monkey_business().to_string()
}

type Item = u64;

#[derive(Debug)]
struct MonkeyGroup {
    monkeys: Vec<Monkey>,
    inspections: Vec<u32>,
    relief_factor: Item,
    divisor_product: Item,
}

impl MonkeyGroup {
    fn parse(input: &[String], relief_factor: Item) -> Self {
        let monkeys: Vec<Monkey> = input
            .chunks(7)
            .map(|monkey_lines| Monkey::parse(monkey_lines).expect("invalid monkey input"))
            .collect();

        Self {
            inspections: vec![0; monkeys.len()],
            divisor_product: monkeys.iter().map(|m| m.divisor).product(),
            monkeys,
            relief_factor,
        }
    }

    fn exec_round(&mut self) {
        let monkeys = &mut self.monkeys;
        for i in 0..monkeys.len() {
            let monkey = &monkeys[i];
            self.inspections[i] += monkey.items.len() as u32;

            for j in 0..monkey.items.len() {
                let monkey = &monkeys[i];
                let item = monkey.items[j];

                // Bless fasterthanli 🙏 (https://fasterthanli.me/series/advent-of-code-2022/part-11#math-check)
                let item = ((monkey.operation)(item) / self.relief_factor) % self.divisor_product;
                let throw_to = (monkey.throw_to)(item);
                monkeys[throw_to].items.push(item);
            }

            monkeys[i].items.clear();
        }
    }

    fn monkey_business(&self) -> u64 {
        let mut inspections = self.inspections.clone();
        inspections.sort();
        let len = inspections.len();

        inspections[len - 1] as u64 * inspections[len - 2] as u64
    }
}

struct Monkey {
    items: Vec<Item>,
    operation: Box<dyn Fn(Item) -> Item>,
    throw_to: Box<dyn Fn(Item) -> usize>,
    divisor: Item,
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .finish()
    }
}

impl Monkey {
    fn parse(lines: &[String]) -> Option<Self> {
        let items: Vec<Item> = lines
            .get(1)?
            .trim()
            .trim_start_matches("Starting items: ")
            .split(',')
            .map(|n| n.trim().parse().expect("should be a number"))
            .collect();

        let (operation_str, value) = lines
            .get(2)?
            .trim()
            .trim_start_matches("Operation: new = old ")
            .split_once(' ')?;
        let operation: Box<dyn Fn(Item) -> Item> = match operation_str {
            "+" => match value.parse::<Item>() {
                Ok(value) => Box::new(move |n| n + value),
                Err(_) => Box::new(|n| n + n),
            },
            "*" => match value.parse::<Item>() {
                Ok(value) => Box::new(move |n| n * value),
                Err(_) => Box::new(|n| n * n),
            },
            operation => panic!("Invalid operation: {operation}"),
        };

        let divisor: Item = lines
            .get(3)?
            .trim()
            .trim_start_matches("Test: divisible by ")
            .parse()
            .ok()?;
        let true_branch: usize = lines
            .get(4)?
            .trim()
            .trim_start_matches("If true: throw to monkey ")
            .parse()
            .ok()?;
        let false_branch: usize = lines
            .get(5)?
            .trim()
            .trim_start_matches("If false: throw to monkey ")
            .parse()
            .ok()?;

        let next_monkey = Box::new(move |n| {
            if n % divisor == 0 {
                true_branch
            } else {
                false_branch
            }
        });

        Some(Monkey {
            items,
            throw_to: next_monkey,
            operation,
            divisor,
        })
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
        assert_eq!(result, "10605");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(result, "2713310158");
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
