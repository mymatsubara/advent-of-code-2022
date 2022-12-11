use std::{fs, time::Instant};

fn part_one(input: &[String]) -> String {
    input
        .iter()
        .map(|line| {
            let shapes: Vec<&str> = line.split(' ').collect();
            let (opponent_symbol, my_symbol) = (shapes[0], shapes[1]);

            Match {
                opponent_shape: map_opponent_shape(opponent_symbol),
                my_shape: map_my_shape(my_symbol),
            }
        })
        .map(|_match| _match.get_points())
        .sum::<u32>()
        .to_string()
}

fn part_two(input: &[String]) -> String {
    input
        .iter()
        .map(|line| {
            let shapes: Vec<&str> = line.split(' ').collect();
            let (opponent_symbol, my_symbol) = (shapes[0], shapes[1]);

            let opponent_shape = map_opponent_shape(opponent_symbol);
            let match_result = map_my_match_result(my_symbol);

            Match::force_result(match_result, opponent_shape)
        })
        .map(|_match| _match.get_points())
        .sum::<u32>()
        .to_string()
}

#[derive(Clone, Copy, Debug)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy, Debug)]
enum MatchResult {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Debug)]
struct Match {
    opponent_shape: Shape,
    my_shape: Shape,
}

impl Match {
    fn get_points(&self) -> u32 {
        self.my_shape as u32 + self.get_result() as u32
    }

    fn get_result(&self) -> MatchResult {
        match self.my_shape {
            Shape::Rock => match self.opponent_shape {
                Shape::Rock => MatchResult::Draw,
                Shape::Paper => MatchResult::Lose,
                Shape::Scissors => MatchResult::Win,
            },
            Shape::Paper => match self.opponent_shape {
                Shape::Rock => MatchResult::Win,
                Shape::Paper => MatchResult::Draw,
                Shape::Scissors => MatchResult::Lose,
            },
            Shape::Scissors => match self.opponent_shape {
                Shape::Rock => MatchResult::Lose,
                Shape::Paper => MatchResult::Win,
                Shape::Scissors => MatchResult::Draw,
            },
        }
    }

    fn force_result(result: MatchResult, opponent_shape: Shape) -> Match {
        let my_shape = match result {
            MatchResult::Lose => match opponent_shape {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            MatchResult::Win => match opponent_shape {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
            MatchResult::Draw => opponent_shape,
        };

        Match {
            opponent_shape,
            my_shape,
        }
    }
}

fn map_opponent_shape(symbol: &str) -> Shape {
    match symbol {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("Unexpected opponent symbol: {symbol}"),
    }
}

fn map_my_shape(symbol: &str) -> Shape {
    match symbol {
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        _ => panic!("Unexpected my symbol: {symbol}"),
    }
}

fn map_my_match_result(symbol: &str) -> MatchResult {
    match symbol {
        "X" => MatchResult::Lose,
        "Y" => MatchResult::Draw,
        "Z" => MatchResult::Win,
        _ => panic!("Unexpected my symbol: {symbol}"),
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
        assert_eq!(result, "15");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(result, "12");
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
