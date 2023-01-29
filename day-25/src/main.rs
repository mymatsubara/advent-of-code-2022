use std::{fs, time::Instant};

fn part_one(input: &[String]) -> String {
    let fuel_requirement: isize = input
        .iter()
        .map(|snafu| SNAFU(snafu.to_string()).to_decimal())
        .sum();

    SNAFU::from_decimal(fuel_requirement).0
}

fn part_two(input: &[String]) -> String {
    "NOT IMPLEMENTED".to_owned()
}

struct SNAFU(String);

impl SNAFU {
    fn from_decimal(mut decimal: isize) -> Self {
        let mut snafu = String::new();

        loop {
            let reminder = decimal % 5;

            let digit = match reminder {
                0..=2 => reminder.to_string(),
                3 => "=".to_string(),
                4 => "-".to_string(),
                _ => panic!("invalid reminder"),
            };

            snafu = digit + &snafu;

            if reminder > 2 {
                decimal += 5;
            }

            decimal /= 5;

            if decimal == 0 {
                break;
            }
        }

        SNAFU(snafu)
    }

    fn to_decimal(&self) -> isize {
        self.0
            .as_bytes()
            .iter()
            .rev()
            .enumerate()
            .map(|(i, digit)| {
                SNAFU::digit_to_decimal(digit).expect("invalid SNAFU digit") * 5_isize.pow(i as u32)
            })
            .sum()
    }

    fn digit_to_decimal(digit: &u8) -> Option<isize> {
        let decimal = match digit {
            b'=' => -2,
            b'-' => -1,
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            _ => return None,
        };

        Some(decimal)
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
        assert_eq!(result, "2=-1=0");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(result, "NOT IMPLEMENTED");
    }

    #[test]
    fn snafu() {
        assert_eq!(SNAFU::from_decimal(0).0, "0");
        assert_eq!(SNAFU::from_decimal(1).0, "1");
        assert_eq!(SNAFU::from_decimal(2).0, "2");
        assert_eq!(SNAFU::from_decimal(3).0, "1=");
        assert_eq!(SNAFU::from_decimal(4).0, "1-");
        assert_eq!(SNAFU::from_decimal(5).0, "10");

        let snafu = SNAFU::from_decimal(314159265);
        assert_eq!(snafu.0, "1121-1110-1=0");
        assert_eq!(snafu.to_decimal(), 314159265);

        let snafu = SNAFU::from_decimal(12345);
        assert_eq!(snafu.0, "1-0---0");
        assert_eq!(snafu.to_decimal(), 12345);

        let snafu = SNAFU::from_decimal(2022);
        assert_eq!(snafu.0, "1=11-2");
        assert_eq!(snafu.to_decimal(), 2022);

        let snafu = SNAFU::from_decimal(20);
        assert_eq!(snafu.0, "1-0");
        assert_eq!(snafu.to_decimal(), 20);
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
