#![feature(drain_filter)]
use std::{
    cmp::Ordering,
    collections::HashMap,
    fs,
    ops::{Add, AddAssign, Deref, Sub, SubAssign},
    time::Instant,
};

fn part_one(input: &[String]) -> String {
    const MINUTES: u8 = 24;
    const INITIAL_STATE: (Robots, Resources) = (Values([1, 0, 0, 0]), Values([0, 0, 0, 0]));

    let best_states: Vec<_> = input
        .iter()
        .map(|line| Blueprint::parse(line))
        .map(|b| (b, b.find_best_state(INITIAL_STATE, MINUTES)))
        .collect();

    best_states
        .iter()
        .map(|(blueprint, resources)| {
            (blueprint.id * resources[Resource::Geode as usize] as u8) as usize
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Values([i16; RESOURCES_TYPES]);

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    id: u8,
    costs: [Values; RESOURCES_TYPES],
}

type Robots = Values;
type Resources = Values;

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

        Blueprint {
            id: id.parse().expect("invalid blueprint id"),
            costs: [ore_cost, clay_cost, obsidian_cost, geode_cost],
        }
    }

    fn find_best_state(&self, initial_state: (Robots, Resources), minutes: u8) -> Values {
        let mut best_states: HashMap<Robots, Resources> = HashMap::new();
        best_states.insert(initial_state.0, initial_state.1);

        for _ in 0..minutes {
            let mut new_states = Vec::with_capacity(best_states.len() * 5);

            for (robots, resources) in best_states.iter() {
                for robot_kind in [
                    Resource::Geode,
                    Resource::Obsidian,
                    Resource::Clay,
                    Resource::Ore,
                ] {
                    if let Some(new_state) =
                        self.try_build_robot_and_gather(robot_kind, *robots, *resources)
                    {
                        new_states.push(new_state);
                    }
                }

                // Do not build robot
                new_states.push((*robots, *resources + *robots))
            }

            // Update best states
            for (robots, resources) in new_states {
                best_states
                    .entry(robots)
                    .and_modify(|best_resouces| {
                        if resources > *best_resouces {
                            *best_resouces = resources;
                        }
                    })
                    .or_insert(resources);
            }
        }

        *best_states.values().max().unwrap()
    }

    fn try_build_robot_and_gather(
        &self,
        kind: Resource,
        mut robots: Robots,
        mut resources: Resources,
    ) -> Option<(Robots, Resources)> {
        // Robot cost
        resources -= self.costs[kind as usize];

        if resources.into_iter().any(|r| r < 0) {
            return None;
        }

        // Previous robots gains
        resources += robots;

        // Add the new robot
        robots.0[kind as usize] += 1;

        Some((robots, resources))
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

impl SubAssign for Values {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..self.len() {
            self.0[i] -= rhs.0[i];
        }
    }
}

impl Ord for Values {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Values {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.0
                .iter()
                .zip(other.0.iter())
                .rev()
                .map(|(a, b)| a.cmp(b))
                .find(|c| !matches!(c, Ordering::Equal))
                .unwrap_or(Ordering::Equal),
        )
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

    #[test]
    fn values_ordering() {
        let v1 = Values([1, 2, 3, 4]);
        let v2 = Values([4, 3, 2, 1]);

        assert!(v1 > v2);
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
