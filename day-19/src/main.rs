#![feature(drain_filter)]
use std::{
    fs,
    ops::{Add, AddAssign, Deref, Sub},
    time::Instant,
};

fn part_one(input: &[String]) -> String {
    const MINUTES: u8 = 24;
    const INITIAL_STATE: State = State {
        resources: Values([0, 0, 0, 0]),
        robots: Values([1, 0, 0, 0]),
    };

    let best_states: Vec<_> = input
        .iter()
        .map(|line| Blueprint::parse(line))
        .map(|b| (b, b.find_best_state(INITIAL_STATE, MINUTES)))
        .collect();

    best_states
        .iter()
        .filter_map(|(blueprint, state)| {
            state.as_ref().map(|state| {
                (blueprint.id * state.resources[Resource::Geode as usize] as u8) as usize
            })
        })
        .sum::<usize>()
        .to_string()
}

fn part_two(input: &[String]) -> String {
    "NOT IMPLEMENTED".to_owned()
}

const RESOURCES_TYPES: usize = 4;

#[derive(Copy, Clone)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Values([i16; RESOURCES_TYPES]);

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    id: u8,
    costs: [Values; RESOURCES_TYPES],
    max_costs: Values,
}

#[derive(Clone, Copy, Debug)]
struct State {
    robots: Values,
    resources: Values,
}

impl Blueprint {
    fn parse(line: &str) -> Blueprint {
        let line = line.trim().trim_start_matches("Blueprint ");
        let (id, line) = line.split_once(": Each ore robot costs ").unwrap();
        let (ore_cost, line) = line.split_once(" ore. Each clay robot costs ").unwrap();
        let (clay_cost, line) = line.split_once(" ore. Each obsidian robot costs ").unwrap();
        let (obsidian_cost_1, line) = line.split_once(" ore and ").unwrap();
        let (obsidian_cost_2, line) = line.split_once(" clay. Each geode robot costs ").unwrap();
        let (geode_cost_1, line) = line.split_once(" ore and ").unwrap();
        let (geode_cost_2, line) = line.split_once(" obsidian.").unwrap();

        let ore_cost = Values([ore_cost.parse().expect("invalid ore cost"), 0, 0, 0]);
        let clay_cost = Values([clay_cost.parse().expect("invalid clay cost"), 0, 0, 0]);
        let obsidian_cost = Values([
            obsidian_cost_1.parse().expect("invalid obsidian cost 1"),
            obsidian_cost_2.parse().expect("invalid obsidian cost 2"),
            0,
            0,
        ]);
        let geode_cost = Values([
            geode_cost_1.parse().expect("invalid geode cost 1"),
            0,
            geode_cost_2.parse().expect("invalid geode cost 2"),
            0,
        ]);

        let costs = [ore_cost, clay_cost, obsidian_cost, geode_cost];

        Blueprint {
            id: id.parse().expect("invalid blueprint id"),
            max_costs: Values(std::array::from_fn(|i| {
                costs.iter().map(|c| c[i]).max().unwrap()
            })),
            costs,
        }
    }

    fn find_best_state(&self, initial_state: State, minutes: u8) -> Option<State> {
        let mut states = vec![initial_state];

        for minute in 0..minutes {
            println!("minutes: {minute}");
            let mut new_states = Vec::with_capacity(states.len());

            // Create new possible states
            for state in states.iter() {
                // Try to build robots
                if let Some(new_state) = state.try_build_and_gather(Resource::Geode, self) {
                    new_states.push(new_state);
                } else if let Some(new_state) = state.try_build_and_gather(Resource::Obsidian, self)
                {
                    new_states.push(new_state);
                } else {
                    for robot_kind in [Resource::Clay, Resource::Ore] {
                        if let Some(new_state) = state.try_build_and_gather(robot_kind, self) {
                            new_states.push(new_state);
                        }
                    }
                }

                // Do not build robot
                let mut new_state = *state;
                new_state.resources += new_state.robots;
                new_states.push(new_state);
            }

            let obsidian_threashold =
                minutes - self.costs[Resource::Geode as usize][Resource::Obsidian as usize] as u8;
            let clay_threshold = obsidian_threashold
                - self.costs[Resource::Obsidian as usize][Resource::Clay as usize] as u8;

            // if minute == obsidian_threashold {
            //     states = new_states
            //         .into_iter()
            //         .filter(|state| state.robots[Resource::Obsidian as usize] > 0)
            //         .collect()
            // } else if minute == clay_threshold {
            //     states = new_states
            //         .into_iter()
            //         .filter(|state| state.robots[Resource::Clay as usize] > 0)
            //         .collect()
            // } else {
            //     states = new_states
            // }

            states = new_states;
        }

        states
            .into_iter()
            .max_by_key(|s| s.resources[Resource::Geode as usize])
    }
}

impl State {
    fn try_build_and_gather(mut self, kind: Resource, blueprint: &Blueprint) -> Option<State> {
        let kind = kind as usize;
        self.resources = self.resources - blueprint.costs[kind];

        if self.resources.into_iter().any(|r| r < 0)
            || (kind != Resource::Geode as usize
                && self.robots.0[kind] >= blueprint.max_costs[kind])
        {
            return None;
        }

        // Gather resources
        self.resources += self.robots;

        // Build robot
        self.robots.0[kind] += 1;

        Some(self)
    }
}

impl Deref for Values {
    type Target = [i16; RESOURCES_TYPES];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Add for Values {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Values(std::array::from_fn(|i| self[i] + rhs[i]))
    }
}

impl AddAssign for Values {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..self.len() {
            self.0[i] += rhs.0[i];
        }
    }
}

impl Sub for Values {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Values(std::array::from_fn(|i| self[i] - rhs[i]))
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
        assert_eq!(result, "33");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(result, "NOT IMPLEMENTED");
    }

    // #[test]
    // fn state_logic() {
    //     let blueprint = Blueprint {
    //         id: 1,
    //         costs: [
    //             Values([4, 0, 0, 0]),
    //             Values([2, 0, 0, 0]),
    //             Values([3, 14, 0, 0]),
    //             Values([2, 0, 7, 0]),
    //         ],
    //     };

    //     let mut state = State {
    //         resources: Values([5, 0, 0, 0]),
    //         robots: Values([1, 0, 0, 0]),
    //     };
    //     assert!(state.try_build_robot(Resource::Ore, &blueprint));
    //     assert_eq!(state.resources, Values([1, 0, 0, 0]));
    //     assert_eq!(state.robots, Values([2, 0, 0, 0]));

    //     let mut state = State {
    //         resources: Values([2, 0, 0, 0]),
    //         robots: Values([1, 0, 0, 0]),
    //     };
    //     assert!(!state.try_build_robot(Resource::Ore, &blueprint));
    //     assert_eq!(state.resources, Values([2, 0, 0, 0]));
    //     assert_eq!(state.robots, Values([1, 0, 0, 0]));

    //     // Clay
    //     let mut state = State {
    //         resources: Values([3, 0, 0, 0]),
    //         robots: Values([1, 0, 1, 0]),
    //     };
    //     assert!(state.try_build_robot(Resource::Clay, &blueprint));
    //     assert_eq!(state.resources, Values([1, 0, 0, 0]));
    //     assert_eq!(state.robots, Values([1, 1, 1, 0]));

    //     // Obsidian
    //     let mut state = State {
    //         resources: Values([5, 19, 3, 0]),
    //         robots: Values([0, 3, 2, 1]),
    //     };
    //     assert!(state.try_build_robot(Resource::Obsidian, &blueprint));
    //     assert_eq!(state.resources, Values([2, 5, 3, 0]));
    //     assert_eq!(state.robots, Values([0, 3, 3, 1]));

    //     // Geode
    //     let mut state = State {
    //         resources: Values([5, 2, 10, 0]),
    //         robots: Values([1, 2, 0, 4]),
    //     };
    //     assert!(state.try_build_robot(Resource::Geode, &blueprint));
    //     assert_eq!(state.resources, Values([3, 2, 3, 0]));
    //     assert_eq!(state.robots, Values([1, 2, 0, 5]));
    // }
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
