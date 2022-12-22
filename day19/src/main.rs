use regex::{Captures, Regex};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug)]
struct Cost {
    ore: u32,
    clay: u32,
    obsidian: u32,
}

#[derive(Clone, Copy, Debug)]
struct Blueprint {
    index: u32,
    ore_robot: Cost,
    clay_robot: Cost,
    obsidian_robot: Cost,
    geode_robot: Cost,
}

impl Blueprint {
    fn max_costs(&self) -> Cost {
        Cost {
            ore: *[
                self.ore_robot.ore,
                self.clay_robot.ore,
                self.obsidian_robot.ore,
                self.geode_robot.ore,
            ]
            .iter()
            .max()
            .unwrap(),
            clay: *[
                self.ore_robot.clay,
                self.clay_robot.clay,
                self.obsidian_robot.clay,
                self.geode_robot.clay,
            ]
            .iter()
            .max()
            .unwrap(),
            obsidian: *[
                self.ore_robot.obsidian,
                self.clay_robot.obsidian,
                self.obsidian_robot.obsidian,
                self.geode_robot.obsidian,
            ]
            .iter()
            .max()
            .unwrap(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct State {
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
}

impl State {
    fn new() -> Self {
        State {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        }
    }

    // The number of a given price we can afford.
    fn can_afford(&self, cost: Cost) -> bool {
        self.ore >= cost.ore && self.clay >= cost.clay && self.obsidian >= cost.obsidian
    }

    fn mine(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geodes += self.geode_robots;
    }

    fn build(&mut self, cost: Cost) {
        self.ore -= cost.ore;
        self.clay -= cost.clay;
        self.obsidian -= cost.obsidian;
    }
}

fn run_state(state: State, prev_state: Option<&State>, blueprint: &Blueprint, minutes: u32) -> u32 {
    if minutes == 1 {
        let mut new_state = state;
        new_state.mine();
        return new_state.geodes;
    }

    let mut branches = vec![];

    // If we could build a particular robot last minute, but chose not to, then it doesn't make sense to continue down a branch where we build one this minute,
    // as there's no way it can lead to a better solution than just building that robot earlier.
    let built_robot = prev_state
        .map(|p| {
            p.geode_robots != state.geode_robots
                || p.obsidian_robots != state.obsidian_robots
                || p.clay_robots != state.clay_robots
                || p.ore_robots != state.ore_robots
        })
        .unwrap_or(false);
    let ignored_geode_robot = prev_state
        .map(|p| p.can_afford(blueprint.geode_robot) && !built_robot)
        .unwrap_or(false);
    let ignored_obsidian_robot = prev_state
        .map(|p| p.can_afford(blueprint.obsidian_robot) && !built_robot)
        .unwrap_or(false);
    let ignored_clay_robot = prev_state
        .map(|p| p.can_afford(blueprint.clay_robot) && !built_robot)
        .unwrap_or(false);
    let ignored_ore_robot = prev_state
        .map(|p| p.can_afford(blueprint.ore_robot) && !built_robot)
        .unwrap_or(false);

    // If we are producing as much of a particular resource as it costs to buy the most expensive usage of that resource, there is no point building any more.
    let max_obsidion_production = state.obsidian_robots >= blueprint.max_costs().obsidian;
    let max_clay_production = state.clay_robots >= blueprint.max_costs().clay;
    let max_ore_production = state.ore_robots >= blueprint.max_costs().ore;

    if state.can_afford(blueprint.geode_robot) && !ignored_geode_robot {
        let mut new_state = state;
        new_state.mine();
        new_state.build(blueprint.geode_robot);
        new_state.geode_robots += 1;
        branches.push(run_state(new_state, Some(&state), blueprint, minutes - 1));
    }

    if state.can_afford(blueprint.obsidian_robot)
        && !ignored_obsidian_robot
        && !max_obsidion_production
    {
        let mut new_state = state;
        new_state.mine();
        new_state.build(blueprint.obsidian_robot);
        new_state.obsidian_robots += 1;
        branches.push(run_state(new_state, Some(&state), blueprint, minutes - 1));
    }

    if state.can_afford(blueprint.clay_robot) && !ignored_clay_robot && !max_clay_production {
        let mut new_state = state;
        new_state.mine();
        new_state.build(blueprint.clay_robot);
        new_state.clay_robots += 1;
        branches.push(run_state(new_state, Some(&state), blueprint, minutes - 1));
    }

    if state.can_afford(blueprint.ore_robot) && !ignored_ore_robot && !max_ore_production {
        let mut new_state = state;
        new_state.mine();
        new_state.build(blueprint.ore_robot);
        new_state.ore_robots += 1;
        branches.push(run_state(new_state, Some(&state), blueprint, minutes - 1));
    }

    let mut new_state = state;
    new_state.mine();
    branches.push(run_state(new_state, Some(&state), blueprint, minutes - 1));

    *branches.iter().max().unwrap()
}

fn part1(blueprints: &[Blueprint]) -> u32 {
    blueprints
        .iter()
        .map(|b| b.index * run_state(State::new(), None, b, 24))
        .sum()
}

fn part2(blueprints: &[Blueprint]) -> u32 {
    blueprints
        .iter()
        .take(3)
        .map(|b| run_state(State::new(), None, b, 32))
        .product()
}

fn parse_num<T: std::str::FromStr>(caps: &Captures, label: &str) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    caps.name(label)
        .unwrap()
        .as_str()
        .parse::<T>()
        .expect("Failed to parse coordinate")
}

fn parse_line(line: &str) -> Blueprint {
    let re = Regex::new(
        r"Blueprint (?P<index>\d+): Each ore robot costs (?P<ore_robot_ore>\d+) ore. Each clay robot costs (?P<clay_robot_ore>\d+) ore. Each obsidian robot costs (?P<obs_robot_ore>\d+) ore and (?P<obs_robot_clay>\d+) clay. Each geode robot costs (?P<geode_robot_ore>\d+) ore and (?P<geode_robot_obs>\d+) obsidian.").expect("Failed to compile regex");

    let caps = re.captures(line).expect("Failed to match line");
    Blueprint {
        index: parse_num(&caps, "index"),
        ore_robot: Cost {
            ore: parse_num(&caps, "ore_robot_ore"),
            clay: 0,
            obsidian: 0,
        },
        clay_robot: Cost {
            ore: parse_num(&caps, "clay_robot_ore"),
            clay: 0,
            obsidian: 0,
        },
        obsidian_robot: Cost {
            ore: parse_num(&caps, "obs_robot_ore"),
            clay: parse_num(&caps, "obs_robot_clay"),
            obsidian: 0,
        },
        geode_robot: Cost {
            ore: parse_num(&caps, "geode_robot_ore"),
            clay: 0,
            obsidian: parse_num(&caps, "geode_robot_obs"),
        },
    }
}

fn read_input(filename: &str) -> Vec<Blueprint> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| parse_line(l.unwrap().trim()))
        .collect()
}

fn main() {
    let blueprints = read_input("input");
    let pt1_result = part1(&blueprints);
    let pt2_result = part2(&blueprints);
    println!("Part 1: {}, Part 2: {}", pt1_result, pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let blueprints = vec![
            Blueprint {
                index: 1,
                ore_robot: Cost {
                    ore: 4,
                    clay: 0,
                    obsidian: 0,
                },
                clay_robot: Cost {
                    ore: 2,
                    clay: 0,
                    obsidian: 0,
                },
                obsidian_robot: Cost {
                    ore: 3,
                    clay: 14,
                    obsidian: 0,
                },
                geode_robot: Cost {
                    ore: 2,
                    clay: 0,
                    obsidian: 7,
                },
            },
            Blueprint {
                index: 2,
                ore_robot: Cost {
                    ore: 2,
                    clay: 0,
                    obsidian: 0,
                },
                clay_robot: Cost {
                    ore: 3,
                    clay: 0,
                    obsidian: 0,
                },
                obsidian_robot: Cost {
                    ore: 3,
                    clay: 8,
                    obsidian: 0,
                },
                geode_robot: Cost {
                    ore: 3,
                    clay: 0,
                    obsidian: 12,
                },
            },
        ];
        assert_eq!(part1(&blueprints), 33);
    }
}
