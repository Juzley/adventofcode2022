use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

#[derive(Copy, Clone, Debug)]
enum Operations {
    Up(u8),
    Right(u8),
    Down(u8),
    Left(u8),
}

fn parse_lines(lines: &Vec<String>) -> Vec<Operations> {
    return lines
        .iter()
        .map(|line| {
            let mut parts = line.split(' ');

            let dir = parts.next().expect("Invalid line");
            let dist = parts
                .next()
                .and_then(|s| s.parse::<u8>().ok())
                .expect("Invalid line");
            let result = match dir {
                "U" => Some(Operations::Up(dist)),
                "R" => Some(Operations::Right(dist)),
                "D" => Some(Operations::Down(dist)),
                "L" => Some(Operations::Left(dist)),
                _ => None,
            };

            result.expect("Invalid command")
        })
        .collect();
}

fn move_tail(head_pos: (i32, i32), tail_pos: (i32, i32)) -> (i32, i32) {
    let disp = (head_pos.0 - tail_pos.0, head_pos.1 - tail_pos.1);

    // If the tail is touching the head, it stays where it is.
    if disp.0.abs() <= 1 && disp.1.abs() <= 1 {
        return tail_pos;
    }

    return (
        tail_pos.0 + disp.0.clamp(-1, 1),
        tail_pos.1 + disp.1.clamp(-1, 1),
    );
}

fn count_tail_positions(ops: &Vec<Operations>, knot_count: usize) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

    let mut locations: Vec<(i32, i32)> = iter::repeat_with(|| (0, 0)).take(knot_count).collect();

    for op in ops {
        let ((x_inc, y_inc), distance) = match op {
            Operations::Up(distance) => ((0, 1), distance),
            Operations::Right(distance) => ((1, 0), distance),
            Operations::Down(distance) => ((0, -1), distance),
            Operations::Left(distance) => ((-1, 0), distance),
        };

        for _ in 0..*distance {
            locations[0].0 += x_inc;
            locations[0].1 += y_inc;

            for i in 1..knot_count {
                locations[i] = move_tail(locations[i - 1], locations[i]);
            }

            visited.insert(locations[knot_count - 1]);
        }
    }

    return visited.len();
}

fn part1(ops: &Vec<Operations>) -> usize {
    return count_tail_positions(ops, 2);
}

fn part2(ops: &Vec<Operations>) -> usize {
    return count_tail_positions(ops, 10);
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|l| String::from(l.unwrap().trim()))
        .collect();
    let ops = parse_lines(&lines);

    let pt1_result = part1(&ops);
    let pt2_result = part2(&ops);

    println!("Part 1: {}, Part 2: {}", pt1_result, pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let lines = vec![
            String::from("R 4"),
            String::from("U 4"),
            String::from("L 3"),
            String::from("D 1"),
            String::from("R 4"),
            String::from("D 1"),
            String::from("L 5"),
            String::from("R 2"),
        ];
        let ops = parse_lines(&lines);
        let result = part1(&ops);

        assert_eq!(result, 13);
    }

    #[test]
    fn pt2_test_a() {
        let lines = vec![
            String::from("R 4"),
            String::from("U 4"),
            String::from("L 3"),
            String::from("D 1"),
            String::from("R 4"),
            String::from("D 1"),
            String::from("L 5"),
            String::from("R 2"),
        ];
        let ops = parse_lines(&lines);
        let result = part2(&ops);

        assert_eq!(result, 1);
    }

    #[test]
    fn pt2_test_b() {
        let lines = vec![
            String::from("R 5"),
            String::from("U 8"),
            String::from("L 8"),
            String::from("D 3"),
            String::from("R 17"),
            String::from("D 10"),
            String::from("L 25"),
            String::from("U 20"),
        ];
        let ops = parse_lines(&lines);
        let result = part2(&ops);

        assert_eq!(result, 36);
    }
}
