use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Valve {
    flow: u32,
    neighbours: Vec<String>,

    // Vector containing all reachable valves, the distance from this valve, and the flow rate of the valve
    paths: HashMap<String, u32>,
}

// Find paths between each node.
fn build_paths(valves: &mut HashMap<String, Valve>) {
    let labels: Vec<String> = valves.keys().cloned().collect();

    for src_label in &labels {
        for dst_label in &labels {
            if src_label == dst_label {
                continue;
            }

            let path = dijkstra(
                &src_label,
                |&label| valves[label].neighbours.iter().map(|l| (l, 1)),
                |&label| label == dst_label,
            );

            if let Some((_, cost)) = path {
                let src_valve = valves.get_mut(src_label).expect("Failed to find valve");
                src_valve.paths.insert(dst_label.clone(), cost);
            }
        }
    }
}

fn max_pressure_worker(
    valves: &HashMap<String, Valve>,
    current_label: &str,
    minutes_remaining: u32,
    remaining_labels: HashSet<&str>,
    memo: &mut HashMap<String, u32>,
) -> u32 {
    let current_node = &valves[current_label];
    if remaining_labels.is_empty() {
        return minutes_remaining * current_node.flow;
    }

    let mut memo_key = remaining_labels
        .iter()
        .sorted()
        .map(|&str| str)
        .collect::<String>();
    memo_key.push_str(minutes_remaining.to_string().as_str());
    memo_key.push_str(current_label);

    let best;
    if memo.contains_key(&memo_key) {
        best = memo[&memo_key];
    } else {
        best = remaining_labels
            .iter()
            .filter_map(|&label| {
                let time_to_open = current_node.paths[label] + 1;
                if time_to_open >= minutes_remaining {
                    None
                } else {
                    let mut new_labels = remaining_labels.clone();
                    new_labels.remove(label);
                    Some(max_pressure_worker(
                        valves,
                        label,
                        minutes_remaining - time_to_open,
                        new_labels,
                        memo,
                    ))
                }
            })
            .max()
            .unwrap_or(0);
        memo.insert(memo_key, best);
    }

    minutes_remaining * current_node.flow + best
}

// Memoized depth-first search to find the highest
fn max_pressure(valves: &HashMap<String, Valve>) -> u32 {
    let mut labels: HashSet<&str> = valves.keys().map(|s| s.as_str()).collect();
    let mut memo = HashMap::new();

    for (k, v) in valves {
        if v.flow == 0 {
            labels.remove(k.as_str());
        }
    }

    max_pressure_worker(valves, "AA", 30, labels, &mut memo)
}

fn max_pressure_double_worker(
    valves: &HashMap<String, Valve>,
    current_label: &str,
    minutes_remaining: u32,
    remaining_labels: HashSet<&str>,
    first_pass: bool,
    memo: &mut HashMap<String, u32>,
) -> u32 {
    let current_node = &valves[current_label];
    if remaining_labels.is_empty() {
        return minutes_remaining * current_node.flow;
    }

    let mut memo_key = remaining_labels
        .iter()
        .sorted()
        .map(|&str| str)
        .collect::<String>();
    memo_key.push_str(minutes_remaining.to_string().as_str());
    memo_key.push_str(current_label);

    let best;
    if memo.contains_key(&memo_key) {
        best = memo[&memo_key];
    } else {
        best = remaining_labels
            .iter()
            .filter_map(|&label| {
                let time_to_open = current_node.paths[label] + 1;
                let mut new_labels = remaining_labels.clone();
                new_labels.remove(label);

                if time_to_open >= minutes_remaining {
                    if first_pass {
                        // Once we've found a complete path with the human,
                        // go again with the remaining labels for the elephant
                        //println!("{:?}", new_labels);
                        Some(max_pressure_double_worker(
                            valves, "AA", 26, new_labels, false, memo,
                        ))
                    } else {
                        // We have paths for both human and elephant, stop here.
                        None
                    }
                } else {
                    Some(max_pressure_double_worker(
                        valves,
                        label,
                        minutes_remaining - time_to_open,
                        new_labels,
                        first_pass,
                        memo,
                    ))
                }
            })
            .max()
            .unwrap_or(0);
        memo.insert(memo_key, best);
    }

    minutes_remaining * current_node.flow + best
}

fn max_pressure_double(valves: &HashMap<String, Valve>) -> u32 {
    let mut labels: HashSet<&str> = valves.keys().map(|s| s.as_str()).collect();
    let mut memo: HashMap<String, u32> = HashMap::new();

    for (k, v) in valves {
        if v.flow == 0 {
            labels.remove(k.as_str());
        }
    }

    max_pressure_double_worker(valves, "AA", 26, labels, true, &mut memo)
}

fn process_input(lines: &[String]) -> HashMap<String, Valve> {
    let mut valves = HashMap::new();
    let re = Regex::new(
        r"Valve (?P<valve>[A-Z]+) has flow rate=(?P<flow>\d+);.*valves? (?P<neighbours>.*)$",
    )
    .expect("Failed to build regex");

    for line in lines {
        let caps = re.captures(line).expect("Failed to match line");
        let label = String::from(caps.name("valve").unwrap().as_str());
        let flow = caps
            .name("flow")
            .and_then(|m| m.as_str().parse::<u32>().ok())
            .unwrap();
        let neighbours = caps
            .name("neighbours")
            .unwrap()
            .as_str()
            .split(", ")
            .map(String::from)
            .collect();
        let valve = Valve {
            flow,
            neighbours,
            paths: HashMap::new(),
        };
        valves.insert(label, valve);
    }

    build_paths(&mut valves);
    valves
}

fn read_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|l| String::from(l.unwrap().trim()))
        .filter(|l| !l.is_empty())
        .collect();
    lines
}

fn part1(valves: &HashMap<String, Valve>) -> u32 {
    max_pressure(valves)
}

fn part2(valves: &HashMap<String, Valve>) -> u32 {
    max_pressure_double(valves)
}

fn main() {
    let lines = read_file("input");
    let valves = process_input(&lines);

    let pt1_result = part1(&valves);
    let pt2_result = part2(&valves);

    println!("Part 1: {}, Part 2: {}", pt1_result, pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let lines = read_file("test_input");
        let valves = process_input(&lines);
        let result = part1(&valves);
        assert_eq!(result, 1651);
    }
}
