use std::fs;

fn main() {
    let (part_one_matches, part_two_matches) = parse_input();

    let part_one = part_one_matches
        .iter()
        .map(|it| it.get_points())
        .sum::<u32>();

    let part_two = part_two_matches
        .iter()
        .map(|it| it.get_points())
        .sum::<u32>();

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
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

fn parse_input() -> (Vec<Match>, Vec<Match>) {
    let input = fs::read_to_string("input.txt").expect("input.txt file not found");

    let lines: Vec<&str> = input.lines().collect();

    let part_one_matches: Vec<Match> = lines
        .iter()
        .map(|line| {
            let shapes: Vec<&str> = line.split(' ').collect();
            let (opponent_symbol, my_symbol) = (shapes[0], shapes[1]);

            Match {
                opponent_shape: map_opponent_shape(opponent_symbol),
                my_shape: map_my_shape(my_symbol),
            }
        })
        .collect();

    let part_two_matches: Vec<Match> = lines
        .iter()
        .map(|line| {
            let shapes: Vec<&str> = line.split(' ').collect();
            let (opponent_symbol, my_symbol) = (shapes[0], shapes[1]);

            let opponent_shape = map_opponent_shape(opponent_symbol);
            let match_result = map_my_match_result(my_symbol);

            Match::force_result(match_result, opponent_shape)
        })
        .collect();

    (part_one_matches, part_two_matches)
}
