use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Cube = (i32, i32, i32);

// Total surface area (both internal and external) for a droplet.
fn surface_area(droplet: &HashSet<Cube>) -> usize {
    let mut area = 0;

    for &(x, y, z) in droplet {
        area += [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ]
        .iter()
        .map(|&(x_inc, y_inc, z_inc)| {
            let neighbour = (x + x_inc, y + y_inc, z + z_inc);
            usize::from(!droplet.contains(&neighbour))
        })
        .sum::<usize>();
    }

    area
}

fn is_external(
    pos: (i32, i32, i32),
    droplet: &HashSet<Cube>,
    mins: (i32, i32, i32),
    maxs: (i32, i32, i32),
    memo: &mut HashMap<Cube, bool>,
) -> bool {
    // DFS until we either reach a node outside the bounds of the droplet, in which case
    // this is an 'external' node, or until we run out of non-droplet nodes, in which case
    // this is an 'internal' node.
    let mut external = false;
    let mut visited = HashSet::from([pos]);
    let mut queue = vec![pos];
    while !queue.is_empty() {
        let cur_pos = queue.pop().unwrap();

        if droplet.contains(&cur_pos) {
            // Stop when we hit a cube that is part of the droplet.
            continue;
        }

        if memo.contains_key(&cur_pos) {
            external = memo[&cur_pos];
            break;
        }

        if !(((mins.0)..=maxs.0).contains(&cur_pos.0)
            && ((mins.1)..=maxs.1).contains(&cur_pos.1)
            && ((mins.2)..=maxs.2).contains(&cur_pos.2))
        {
            // This position is outside the bounds, therefore must
            // be external, and all the positions we visited to reach
            // here must be too.
            external = true;
            break;
        }

        let neighbours = [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ]
        .iter()
        .map(|&(x_inc, y_inc, z_inc)| (cur_pos.0 + x_inc, cur_pos.1 + y_inc, cur_pos.2 + z_inc))
        .filter(|pos| !visited.contains(pos))
        .collect::<Vec<_>>();

        for n in neighbours {
            visited.insert(n);
            queue.push(n);
        }
    }

    // Update the cache for all the nodes we visited - they are either all external or all internal.
    for p in visited {
        if !droplet.contains(&p) {
            memo.insert(p, external);
        }
    }

    external
}

fn find_droplet_bounds(droplet: &HashSet<Cube>) -> ((i32, i32, i32), (i32, i32, i32)) {
    // Find the bounding box of the droplet. Pick a random point in the droplet to start the fold at.
    let start_point = *droplet.iter().next().unwrap();
    droplet
        .iter()
        .fold((start_point, start_point), |acc, &(x, y, z)| {
            let mins = acc.0;
            let maxs = acc.1;
            let new_mins = (
                cmp::min(mins.0, x),
                cmp::min(mins.1, y),
                cmp::min(mins.2, z),
            );
            let new_maxs = (
                cmp::max(maxs.0, x),
                cmp::max(maxs.1, y),
                cmp::max(maxs.2, z),
            );
            (new_mins, new_maxs)
        })
}

fn external_surface_area(droplet: &HashSet<Cube>) -> usize {
    let (mins, maxs) = find_droplet_bounds(droplet);

    let mut area = 0;
    let mut memo = HashMap::new();
    for &(x, y, z) in droplet {
        area += [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ]
        .iter()
        .map(|&(x_inc, y_inc, z_inc)| {
            let neighbour = (x + x_inc, y + y_inc, z + z_inc);
            usize::from(is_external(neighbour, droplet, mins, maxs, &mut memo))
        })
        .sum::<usize>();
    }

    area
}

fn part1(droplet: &HashSet<Cube>) -> usize {
    surface_area(droplet)
}

fn part2(droplet: &HashSet<Cube>) -> usize {
    external_surface_area(droplet)
}

fn parse_lines(lines: &[String]) -> HashSet<Cube> {
    lines
        .iter()
        .map(|l| {
            let mut parts = l.split(',').map(|p| p.parse::<i32>().unwrap());
            (
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            )
        })
        .collect()
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

fn main() {
    let lines = read_file("input");
    let droplet = parse_lines(&lines);

    let pt1_result = part1(&droplet);
    let pt2_result = part2(&droplet);

    println!("Part 1: {}, Part 2: {}", pt1_result, pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let lines = vec![
            String::from("2,2,2"),
            String::from("1,2,2"),
            String::from("3,2,2"),
            String::from("2,1,2"),
            String::from("2,3,2"),
            String::from("2,2,1"),
            String::from("2,2,3"),
            String::from("2,2,4"),
            String::from("2,2,6"),
            String::from("1,2,5"),
            String::from("3,2,5"),
            String::from("2,1,5"),
            String::from("2,3,5"),
        ];
        let droplet = parse_lines(&lines);
        assert_eq!(part1(&droplet), 64);
    }

    #[test]
    fn pt2_test() {
        let lines = vec![
            String::from("2,2,2"),
            String::from("1,2,2"),
            String::from("3,2,2"),
            String::from("2,1,2"),
            String::from("2,3,2"),
            String::from("2,2,1"),
            String::from("2,2,3"),
            String::from("2,2,4"),
            String::from("2,2,6"),
            String::from("1,2,5"),
            String::from("3,2,5"),
            String::from("2,1,5"),
            String::from("2,3,5"),
        ];
        let droplet = parse_lines(&lines);
        assert_eq!(part2(&droplet), 58);
    }
}
