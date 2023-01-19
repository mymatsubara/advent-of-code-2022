use std::{collections::HashMap, fs, time::Instant};

use day_16::BitSet;

fn part_one(input: &[String]) -> String {
    let volcano = Volcano::parse(input);

    volcano
        .max_pressure(State {
            minutes_left: (30, 0),
            valve_idx: (volcano.start_index, volcano.start_index),
            visited: BitSet::new(),
        })
        .to_string()
}

fn part_two(input: &[String]) -> String {
    let volcano = Volcano::parse(input);

    volcano
        .max_pressure(State {
            minutes_left: (26, 26),
            valve_idx: (volcano.start_index, volcano.start_index),
            visited: BitSet::new(),
        })
        .to_string()
}

#[derive(Debug)]
struct Valve {
    flow_rate: u32,
    distances: Vec<u32>,
    name: String,
}

#[derive(Debug)]
struct Volcano {
    valves: Vec<Valve>,
    start_index: usize,
    pressurized_indexes: Vec<usize>,
}

#[derive(Debug, Copy, Clone)]
struct State {
    valve_idx: (usize, usize),
    visited: BitSet,
    minutes_left: (u32, u32),
}

impl Volcano {
    fn max_pressure(&self, mut state: State) -> u32 {
        let (my_minutes_left, elefant_minutes_left) = state.minutes_left;
        let (my_valve, elefant_valve) = state.valve_idx;
        let my_turn = my_minutes_left >= elefant_minutes_left;

        let valve_idx = if my_turn { my_valve } else { elefant_valve };
        let minutes_left = my_minutes_left.max(elefant_minutes_left);

        let cur_valve = self.valves.get(valve_idx).unwrap();
        let mut max_pressure = 0;

        // Try to visite all pressurized valves which were not yet visited
        for next_valve_idx in self
            .pressurized_indexes
            .iter()
            .filter(|idx| !state.visited.contains(**idx))
            .copied()
        {
            let minutes_spent = cur_valve.distances[next_valve_idx] + 1;

            // Go to next valve only if there is minutes left
            let Some(minutes_left) = minutes_left.checked_sub(minutes_spent) else {
                continue;
            };

            let flow_rate = self.valves[next_valve_idx].flow_rate;
            let pressure_gain = flow_rate * minutes_left;

            let mut visited = state.visited.clone();
            visited.set(next_valve_idx);
            let new_state = State {
                visited,
                valve_idx: if my_turn {
                    (next_valve_idx, elefant_valve)
                } else {
                    (my_valve, next_valve_idx)
                },
                minutes_left: if my_turn {
                    (minutes_left, elefant_minutes_left)
                } else {
                    (my_minutes_left, minutes_left)
                },
            };

            // Calculate how much pressure we are gonna get taking this branch
            max_pressure = max_pressure.max(pressure_gain + self.max_pressure(new_state));
        }

        max_pressure
    }

    fn parse(input: &[String]) -> Volcano {
        let valves_len = input.len();

        // Create valves with flow_rate and zero costs
        let valves: HashMap<_, _> = input
            .iter()
            .map(|line| {
                let line = line.trim_start_matches("Valve ");
                let (valve_name, line) = line
                    .split_once(" has flow rate=")
                    .expect("invalid valve name");

                let (flow_rate, line) = line.split_once("; ").expect("invalid flow rate");
                let neighboors: Vec<String> = line
                    .split_whitespace()
                    .skip(4)
                    .map(|s| s.trim_end_matches(",").to_string())
                    .collect();
                assert!(neighboors.len() > 0);

                (
                    valve_name,
                    (
                        Valve {
                            flow_rate: flow_rate.parse().expect("flow rate is not numeric"),
                            distances: vec![u32::MAX; valves_len],
                            name: valve_name.to_string(),
                        },
                        neighboors,
                    ),
                )
            })
            .collect();

        // Get valves indexes
        let name_to_index: HashMap<String, usize> = valves
            .keys()
            .enumerate()
            .map(|(index, key)| (key.to_string(), index))
            .collect();

        let start_index = *name_to_index.get("AA").expect("should have valve named AA");
        let neighboors: Vec<Vec<usize>> = valves
            .values()
            .enumerate()
            .map(|(_, (_, neighboors))| {
                neighboors
                    .iter()
                    .map(|name| *name_to_index.get(name).unwrap())
                    .collect()
            })
            .collect();

        let valves: Vec<_> = valves.into_iter().map(|(_, (valve, _))| valve).collect();

        let mut volcano = Volcano {
            start_index,
            pressurized_indexes: valves
                .iter()
                .enumerate()
                .filter(|(_, valve)| valve.flow_rate > 0)
                .map(|(i, _)| i)
                .collect(),
            valves,
        };

        volcano.calc_distances(neighboors);

        volcano
    }

    fn calc_distances(&mut self, neighboors: Vec<Vec<usize>>) {
        for i in 0..self.valves.len() {
            let valve = &mut self.valves[i];

            // Distance to itself is zero
            valve.distances[i] = 0;

            // Calculate neightboors minimun distances using DFS
            Self::calc_distances_rec(valve, &neighboors, i, 1);
        }
    }

    fn calc_distances_rec(
        valve: &mut Valve,
        neighboors: &Vec<Vec<usize>>,
        valve_idx: usize,
        cost: u32,
    ) {
        for neighboor_idx in neighboors[valve_idx].iter().copied() {
            let neighboor_cost = &mut valve.distances[neighboor_idx];

            if *neighboor_cost > cost {
                *neighboor_cost = cost;
                Self::calc_distances_rec(valve, neighboors, neighboor_idx, cost + 1);
            }
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
        assert_eq!(result, "1651");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(result, "1707");
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
