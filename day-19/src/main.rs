#![feature(drain_filter)]
use std::{
    fs,
    ops::{Add, AddAssign, Deref, Mul, Sub},
    time::Instant,
};

fn part_one(input: &[String]) -> String {
    const INITIAL_STATE: State = State {
        minutes_left: 24,
        resources: Values([0, 0, 0, 0]),
        robots: Values([1, 0, 0, 0]),
    };

    let best_states: Vec<_> = input
        .iter()
        .map(|line| Blueprint::parse(line))
        .map(|b| (b, b.find_best_state(INITIAL_STATE)))
        .collect();

    best_states
        .iter()
        .map(|(blueprint, state)| {
            blueprint.id as usize * state.resources[Resource::Geode as usize] as usize
        })
        .sum::<usize>()
        .to_string()
}

fn part_two(input: &[String]) -> String {
    const INITIAL_STATE: State = State {
        minutes_left: 32,
        resources: Values([0, 0, 0, 0]),
        robots: Values([1, 0, 0, 0]),
    };

    let best_states: Vec<_> = input
        .iter()
        .take(3)
        .map(|line| Blueprint::parse(line))
        .map(|b| b.find_best_state(INITIAL_STATE))
        .collect();

    best_states
        .iter()
        .map(|state| state.resources[Resource::Geode as usize] as usize)
        .product::<usize>()
        .to_string()
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
struct Values([u8; RESOURCES_TYPES]);

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    id: u8,
    costs: [Values; RESOURCES_TYPES],
    max_costs: Values,
}

#[derive(Clone, Copy, Debug)]
struct State {
    minutes_left: u8,
    robots: Values,
    resources: Values,
}

impl Blueprint {
    fn find_best_state(&self, initial_state: State) -> State {
        initial_state
            .branch(self)
            .map(|branch| self.find_best_state(branch))
            .max_by_key(|state| state.resources[Resource::Geode as usize])
            .unwrap_or(initial_state)
    }

    fn parse(line: &str) -> Blueprint {
        let line = line.trim().trim_start_matches("Blueprint ");
        let (id, line) = line.split_once(": Each ore robot costs ").unwrap();
        let (ore_cost, line) = line.split_once(" ore. Each clay robot costs ").unwrap();
        let (clay_cost, line) = line.split_once(" ore. Each obsidian robot costs ").unwrap();
        let (obsidian_cost_1, line) = line.split_once(" ore and ").unwrap();
        let (obsidian_cost_2, line) = line.split_once(" clay. Each geode robot costs ").unwrap();
        let (geode_cost_1, line) = line.split_once(" ore and ").unwrap();
        let (geode_cost_2, _) = line.split_once(" obsidian.").unwrap();

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
}

impl State {
    fn branch(self, blueprint: &Blueprint) -> impl Iterator<Item = Self> {
        [
            self.try_build(Resource::Geode, blueprint),
            self.try_build(Resource::Obsidian, blueprint),
            self.try_build(Resource::Clay, blueprint),
            self.try_build(Resource::Ore, blueprint),
        ]
        .into_iter()
        .flatten()
    }

    fn try_build(&self, kind: Resource, blueprint: &Blueprint) -> Option<State> {
        let kind = kind as usize;
        let cost = blueprint.costs[kind];

        // Check if it makes sense to build such robot
        if kind != Resource::Geode as usize && self.robots[kind] >= blueprint.max_costs[kind] {
            return None;
        }

        // Check if we have the robots necessary to build what we want
        // (eg. if we want to build an geode robot we need obsidian and ore robots)
        if cost
            .iter()
            .enumerate()
            .filter(|(_, cost)| **cost > 0)
            .any(|(resource, _)| self.robots[resource] == 0)
        {
            return None;
        }

        // We can safelly build the robot (based on: https://github.com/Crazytieguy/advent-of-code/blob/master/2022/src/bin/day19/main.rs)
        (0..self.minutes_left)
            .rev()
            .zip(0..)
            .find_map(|(minutes_left, minutes_waiting)| {
                let resources = self.resources + self.robots * minutes_waiting;

                match resources.checked_sub(cost) {
                    Some(resources_after_build) => {
                        let mut new_robots = self.robots;
                        new_robots.0[kind] += 1;

                        Some(State {
                            minutes_left,
                            resources: resources_after_build + self.robots,
                            robots: new_robots,
                        })
                    }
                    None if minutes_left == 0 => Some(State {
                        minutes_left,
                        resources: resources + self.robots,
                        robots: self.robots,
                    }),
                    _ => None,
                }
            })
    }
}

impl Deref for Values {
    type Target = [u8; RESOURCES_TYPES];

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

impl Mul<u8> for Values {
    type Output = Self;

    fn mul(mut self, rhs: u8) -> Self::Output {
        for i in 0..self.len() {
            self.0[i] *= rhs;
        }

        self
    }
}

impl Values {
    fn checked_sub(mut self, rhs: Self) -> Option<Self> {
        for i in 0..self.len() {
            self.0[i] = self[i].checked_sub(rhs[i])?;
        }

        Some(self)
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
        assert_eq!(result, (56 * 62).to_string());
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
