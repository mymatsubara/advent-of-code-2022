use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
    time::Instant,
};

fn part_one(input: &[String]) -> String {
    let monkeys = Monkeys::parse(input);

    monkeys.yell("root").expect("monkey not found").to_string()
}

fn part_two(input: &[String]) -> String {
    let mut monkeys = Monkeys::parse(input);
    monkeys.transform_root_to_sub(input);

    let mut humn = monkeys.yell("humn").unwrap();
    let mut prev_root = monkeys.yell("root").unwrap();
    let mut steps = 1;
    let target = 0;

    // Using exponetial backoff to find `humn` value
    loop {
        let walk_humn = humn + steps;
        monkeys.set_value("humn", walk_humn);
        let root = monkeys.yell("root").unwrap();

        if root == target {
            return monkeys.yell("humn").unwrap().to_string();
        }

        // Passed by target
        if (min(root, prev_root)..=max(root, prev_root)).contains(&target) {
            // Change step direction
            steps = -steps.signum();
            humn = walk_humn;
        } else {
            steps *= 2;
        }

        prev_root = root;
    }
}

type MonkeyFn = Box<dyn Fn(&Monkeys) -> i64>;

struct Monkeys {
    monkeys: HashMap<String, MonkeyFn>,
}

impl Monkeys {
    fn parse(input: &[String]) -> Self {
        let mut monkeys: HashMap<String, MonkeyFn> = HashMap::new();

        input
            .iter()
            .map(|line| {
                let (name, expr) = line.split_once(':').expect("input line should contain :");
                let mut expr = expr.trim().split_whitespace();

                let a = expr.next().unwrap();
                if let Ok(number) = a.parse::<i64>() {
                    return (name, Box::new(move |_: &Monkeys| number) as MonkeyFn);
                }

                let op = expr.next().expect("input line should contain operation");
                let b = expr
                    .next()
                    .expect("input line should contain second argument");

                let a = a.to_string();
                let b = b.to_string();

                let func: MonkeyFn = match op {
                    "+" => Box::new(move |m| m.monkeys[&a](m) + m.monkeys[&b](m)),
                    "-" => Box::new(move |m| m.monkeys[&a](m) - m.monkeys[&b](m)),
                    "*" => Box::new(move |m| m.monkeys[&a](m) * m.monkeys[&b](m)),
                    "/" => Box::new(move |m| m.monkeys[&a](m) / m.monkeys[&b](m)),
                    _ => panic!("unexpected monkey operation"),
                };

                (name, func)
            })
            .for_each(|(name, func)| {
                monkeys.insert(name.to_string(), func);
            });

        Self { monkeys }
    }

    fn transform_root_to_sub(&mut self, input: &[String]) {
        let new_root = input
            .iter()
            .find_map(|line| {
                let (name, expr) = line.split_once(':').expect("input line should contain :");
                if name != "root" {
                    return None;
                }

                let mut expr = expr.trim().split_whitespace();
                let a = expr.next().unwrap().to_string();
                let _ = expr.next().unwrap();
                let b = expr.next().unwrap().to_string();

                let func: MonkeyFn = Box::new(move |m| m.monkeys[&a](m) - m.monkeys[&b](m));
                Some(func)
            })
            .unwrap();

        self.monkeys.insert("root".to_string(), new_root);
    }

    fn set_value(&mut self, monkey_name: &str, value: i64) {
        self.monkeys
            .insert(monkey_name.to_string(), Box::new(move |_| value));
    }

    fn yell(&self, monkey_name: &str) -> Option<i64> {
        Some(self.monkeys.get(monkey_name)?(self))
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
        assert_eq!(result, "152");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(result, "301");
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
