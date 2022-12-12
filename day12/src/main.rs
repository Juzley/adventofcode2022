use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Point = (usize, usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct OpenPoint {
    score: u32,
    loc: (usize, usize),
}

impl Ord for OpenPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .score
            .cmp(&self.score)
            .then_with(|| other.loc.cmp(&self.loc))
    }
}

impl PartialOrd for OpenPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_lines(
    lines: &Vec<String>,
) -> (
    Vec<Vec<char>>,
    (usize, usize),
    (usize, usize),
    HashSet<Point>,
) {
    let mut start = (0, 0);
    let mut goal = (0, 0);
    let mut all_starts = HashSet::new();
    let map = lines
        .iter()
        .enumerate()
        .map(|(row_index, line)| {
            line.chars()
                .enumerate()
                .map(|(col_index, c)| match c {
                    'S' => {
                        start = (col_index, row_index);
                        all_starts.insert((col_index, row_index));
                        'a'
                    }
                    'E' => {
                        goal = (col_index, row_index);
                        'z'
                    }
                    'a' => {
                        all_starts.insert((col_index, row_index));
                        'a'
                    }
                    _ => c,
                })
                .collect()
        })
        .collect();

    return (map, start, goal, all_starts);
}

fn heuristic(a: Point, goal: Point) -> u32 {
    return ((a.0 as i32 - goal.0 as i32).abs() + (a.1 as i32 - goal.1 as i32).abs()) as u32;
}

fn find_shortest_path(map: &Vec<Vec<char>>, start: Point, goal: Point) -> Option<usize> {
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    // Using A* to find the path from start to goal.
    let mut came_from: HashMap<Point, Point> = HashMap::from([(start, start)]);
    let mut scores: HashMap<Point, u32> = HashMap::from([(start, 0)]);

    let mut queue = BinaryHeap::new();
    queue.push(OpenPoint {
        score: 0,
        loc: start,
    });

    while let Some(OpenPoint {
        score: _,
        loc: current_loc,
    }) = queue.pop()
    {
        if current_loc == goal {
            // Reached the goal, rebuild the path to find number of steps
            let mut distance = 0;
            let mut current = current_loc;
            while came_from[&current] != start {
                current = came_from[&current];
                distance += 1;
            }

            return Some(distance + 1);
        }

        let neighbours: Vec<Point> = [(0, -1), (0, 1), (-1, 0), (1, 0)]
            .iter()
            .map(|dir| (current_loc.0 as i32 + dir.0, current_loc.1 as i32 + dir.1))
            .filter(|loc|
                // Filter out of bounds
                loc.0 >= 0 && loc.0 < width && loc.1 >= 0 && loc.1 < height)
            .map(|(col, row)| (col as usize, row as usize))
            .filter(|(col, row)|
                // Filter climbs that are too steep
                (map[*row][*col] as u32) <= (map[current_loc.1][current_loc.0] as u32) + 1)
            .collect();

        for n in neighbours {
            let new_score = scores[&current_loc] + 1;

            if !scores.contains_key(&n) || new_score < scores[&n] {
                scores.insert(n, new_score);
                queue.push(OpenPoint {
                    score: new_score + heuristic(n, goal),
                    loc: n,
                });
                came_from.insert(n, current_loc);
            }
        }
    }

    // Failed to reach the goal.
    return None;
}

fn part1(map: &Vec<Vec<char>>, start: (usize, usize), goal: (usize, usize)) -> usize {
    return find_shortest_path(map, start, goal).expect("Didn't find path to goal");
}

fn part2(map: &Vec<Vec<char>>, starts: &HashSet<Point>, goal: (usize, usize)) -> usize {
    // Just run pathfinding on each possible start, throwing aways the path details each time.
    // This isn't optimal - we could reuse any the path details from previous runs - but it still
    // executes quickly enough for the given input.
    starts
        .iter()
        .filter_map(|&start| find_shortest_path(map, start, goal))
        .min()
        .expect("Couldn't find minimum path length")
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|l| String::from(l.unwrap().trim()))
        .collect();
    let (map, start, end, all_starts) = parse_lines(&lines);

    let pt1_result = part1(&map, start, end);
    let pt2_result = part2(&map, &all_starts, end);

    println!("Part 1: {}, Part 2: {}", pt1_result, pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let lines = vec![
            String::from("Sabqponm"),
            String::from("abcryxxl"),
            String::from("accszExk"),
            String::from("acctuvwj"),
            String::from("abdefghi"),
        ];

        let (map, start, end, _) = parse_lines(&lines);
        let result = part1(&map, start, end);
        assert_eq!(result, 31);
    }

    #[test]
    fn pt2_test() {
        let lines = vec![
            String::from("Sabqponm"),
            String::from("abcryxxl"),
            String::from("accszExk"),
            String::from("acctuvwj"),
            String::from("abdefghi"),
        ];

        let (map, _, end, starts) = parse_lines(&lines);
        let result = part2(&map, &starts, end);
        assert_eq!(result, 29);
    }
}
