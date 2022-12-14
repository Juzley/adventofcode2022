use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Point = (i32, i32);
type Map = HashSet<Point>;

fn parse_lines(lines: &Vec<String>) -> Map {
    let mut map = HashSet::new();

    for l in lines {
        let line = l.replace("-> ", "");

        let mut prev_point: Option<Point> = None;
        for chunk in line.split(' ') {
            let mut parts = chunk
                .split(',')
                .filter_map(|part| return part.parse::<i32>().ok());
            let x: i32 = parts.next().expect("Incomplete coord");
            let y: i32 = parts.next().expect("Incomplete coord");

            let new = (x, y);
            if let Some(prev) = prev_point {
                let x_inc: i32 = (new.0 - prev.0).clamp(-1, 1);
                let y_inc: i32 = (new.1 - prev.1).clamp(-1, 1);
                let mut cur = prev;

                loop {
                    map.insert(cur);
                    if cur == new {
                        break;
                    }

                    cur = (cur.0 + x_inc, cur.1 + y_inc);
                }
            }

            prev_point = Some(new);
        }
    }

    map
}

fn run_bottomless_sim(mut map: Map) -> u32 {
    let lowest_y_coord = map.iter().max_by(|(_, ay), (_, by)| ay.cmp(by)).unwrap().1;
    let mut grain_count = 0;

    loop {
        let mut coord = (500, 0);

        loop {
            // Move the grain down if it can.
            let mut moved = false;

            for dir in [(0, 1), (-1, 1), (1, 1)] {
                let candidate = (coord.0 + dir.0, coord.1 + dir.1);

                if !map.contains(&candidate) {
                    // We found a place this grain can go, move it down.
                    coord = candidate;
                    moved = true;
                    break;
                }
            }

            // If we didn't move the grain down, stop it here
            if !moved {
                grain_count += 1;
                map.insert(coord);
                break;
            }

            // If the grain has dropped off the edge, we've reached the end of the simulation.
            if coord.1 > lowest_y_coord {
                // The grain has dropped off the edge, stop the simulation.
                return grain_count;
            }
        }
    }
}

fn run_bottomed_sim(mut map: Map) -> u32 {
    let lowest_y_coord = map.iter().max_by(|(_, ay), (_, by)| ay.cmp(by)).unwrap().1 + 1;
    let mut grain_count = 0;
    let start = (500, 0);

    loop {
        let mut coord = start;

        loop {
            // Move the grain down if it can.
            let mut moved = false;

            if coord.1 < lowest_y_coord {
                for dir in [(0, 1), (-1, 1), (1, 1)] {
                    let candidate = (coord.0 + dir.0, coord.1 + dir.1);

                    if !map.contains(&candidate) {
                        // We found a place this grain can go, move it down.
                        coord = candidate;
                        moved = true;
                        break;
                    }
                }
            }

            // If we didn't move the grain down, stop it here
            if !moved {
                grain_count += 1;

                if coord == start {
                    // We didn't move the grain from the start, stop the sim.
                    return grain_count;
                }

                map.insert(coord);
                break;
            }
        }
    }
}

fn part1(map: &Map) -> u32 {
    run_bottomless_sim(map.clone())
}

fn part2(map: &Map) -> u32 {
    run_bottomed_sim(map.clone())
}

fn read_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|l| String::from(l.unwrap().trim()))
        .filter(|l| !l.is_empty())
        .collect();
    return lines;
}

fn main() {
    let lines = read_file("input");
    let map = parse_lines(&lines);

    let pt1_result = part1(&map);
    let pt2_result = part2(&map);

    println!("Part 1: {}, Part 2: {}", pt1_result, pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let lines = vec![
            String::from("498,4 -> 498,6 -> 496,6"),
            String::from("503,4 -> 502,4 -> 502,9 -> 494,9"),
        ];

        let map = parse_lines(&lines);
        let result = part1(&map);
        assert_eq!(result, 24);
    }

    #[test]
    fn pt2_test() {
        let lines = vec![
            String::from("498,4 -> 498,6 -> 496,6"),
            String::from("503,4 -> 502,4 -> 502,9 -> 494,9"),
        ];

        let map = parse_lines(&lines);
        let result = part2(&map);
        assert_eq!(result, 93);
    }
}
