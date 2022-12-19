use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type Point = (i64, i64);

const BOARD_WIDTH: i64 = 7;

#[derive(Copy, Clone, Debug, EnumIter, Eq, Hash, PartialEq)]
enum Rock {
    Wide,
    Cross,
    Corner,
    Tall,
    Square,
}

#[derive(Debug)]
enum Jet {
    Left,
    Right,
}

struct Board {
    filled_points: HashSet<Point>,
    max_heights: [i64; 7],
}

fn get_rock_coords(rock: Rock) -> &'static [Point] {
    static WIDE: [Point; 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
    static CROSS: [Point; 5] = [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)];
    static CORNER: [Point; 5] = [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
    static TALL: [Point; 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];
    static SQUARE: [Point; 4] = [(0, 0), (1, 0), (0, 1), (1, 1)];

    match rock {
        Rock::Wide => &WIDE,
        Rock::Cross => &CROSS,
        Rock::Corner => &CORNER,
        Rock::Tall => &TALL,
        Rock::Square => &SQUARE,
    }
}

fn drop_rock<'a>(
    rock: Rock,
    board: &Board,
    jets: &mut impl Iterator<Item = (usize, &'a Jet)>,
) -> Vec<Point> {
    let start_height = board.max_heights.iter().max().unwrap() + 4;
    let mut origin = (2, start_height);

    let mut dropping = false;
    let mut prev_points = Vec::new();
    loop {
        let new_origin = if dropping {
            // The rock falls
            (origin.0, origin.1 - 1)
        } else {
            // A jet blows the rock
            let dir = jets.next().unwrap();
            match dir.1 {
                Jet::Left => (origin.0 - 1, origin.1),
                Jet::Right => (origin.0 + 1, origin.1),
            }
        };

        let candidate_points: Vec<Point> = get_rock_coords(rock)
            .iter()
            .map(|c| (c.0 + new_origin.0, c.1 + new_origin.1))
            .collect();
        let can_move = candidate_points.iter().all(|c| {
            !(c.0 < 0 || c.0 >= BOARD_WIDTH || c.1 <= 0 || board.filled_points.contains(c))
        });

        if can_move {
            origin = new_origin;
            prev_points = candidate_points;
        } else if dropping {
            return prev_points;
        }

        dropping = !dropping;
    }
}

fn drop_rocks(board: &mut Board, jets: &[Jet], count: i64) {
    let mut jets_iter = jets.iter().cycle().enumerate();
    let mut rocks = Rock::iter().cycle();
    for _ in 0..count {
        let new_points = drop_rock(rocks.next().unwrap(), board, &mut jets_iter);

        for p in new_points {
            board.filled_points.insert(p);
            let col = p.0 as usize;
            board.max_heights[col] = cmp::max(board.max_heights[col], p.1);
        }
    }
}

// Find the lowest point that we can reach in the board from a given starting row.
fn find_lowest_reachable(board: &Board, start_y: i64) -> i64 {
    let mut visited = HashSet::new();
    let mut queue: Vec<(i64, i64)> = vec![(0, start_y)];
    let mut lowest_y = start_y;

    while !queue.is_empty() {
        let pos = queue.pop().unwrap();
        visited.insert(pos);

        let neighbours = [(-1, 0), (0, -1), (1, 0)]
            .iter()
            .map(|&(x_inc, y_inc)| (pos.0 + x_inc, pos.1 + y_inc))
            .filter(|&(x, y)| (0..BOARD_WIDTH).contains(&x) && y > 0);

        for n in neighbours {
            if board.filled_points.contains(&n) {
                lowest_y = cmp::min(n.1, lowest_y);
            } else if !visited.contains(&n) {
                queue.push(n);
            }
        }
    }

    lowest_y
}

// Build a 'hash' we can use to compare board states - print all rows that we could possibly reach with
// a new rock.
fn build_board_hash(board: &Board, highest_y: i64) -> String {
    let lowest_y = find_lowest_reachable(board, highest_y + 1);

    let mut buf = String::with_capacity(((highest_y - lowest_y + 1) * BOARD_WIDTH) as usize);
    for y in (lowest_y..highest_y).rev() {
        for x in 0..BOARD_WIDTH {
            if board.filled_points.contains(&(x, y)) {
                buf.push('#');
            } else {
                buf.push('.');
            }
        }
        buf.push('\n');
    }
    buf
}

fn find_height(board: &mut Board, jets: &[Jet], drop_count: i64) -> i64 {
    // Previous states at the start of dropping a rock - maps rock type and starting jet index to the number of
    // rocks that have been dropped.
    let mut start_states: HashMap<(Rock, i64), i64> = HashMap::new();
    let mut start_state_cycle_start = 0;
    let mut start_state_cycle_length = 0;
    let mut start_state_cycle_start_height = 0;
    let mut cycle_heights: Vec<i64> = vec![0];

    // Hash of the board state at the start of the start cycle.
    let mut start_state_cycle_board_hash: Option<String> = None;
    let mut overall_cycle_length = 0;
    let mut overall_cycle_height = 0;

    let rocks = Rock::iter().cycle();
    let mut jets_iter = jets.iter().cycle().enumerate().peekable();
    let mut drops = 0;

    for rock in rocks {
        // Check for cycles.
        let max_height = board.max_heights.iter().max().unwrap();
        let next_jet_idx = (jets_iter.peek().unwrap().0 % jets.len()) as i64;
        if start_state_cycle_start == 0 {
            // Check for a cycle in start states.
            if let Some(&prev_drop_count) = start_states.get(&(rock, next_jet_idx)) {
                // We have seen this start state before, so we know there is a cycle in start states.
                start_state_cycle_start = drops;
                start_state_cycle_start_height = *max_height;
                start_state_cycle_length = drops - prev_drop_count;
                start_state_cycle_board_hash = Some(build_board_hash(board, *max_height));
            } else {
                start_states.insert((rock, next_jet_idx), drops);
            }
        } else {
            cycle_heights.push(*max_height - start_state_cycle_start_height);

            if overall_cycle_length == 0
                && (drops - start_state_cycle_start) % start_state_cycle_length == 0
            {
                // Check for a cycle in the overall state (start state + board layout)
                let new_hash = build_board_hash(board, *max_height);
                if *start_state_cycle_board_hash.as_ref().unwrap() == new_hash {
                    overall_cycle_length = drops - start_state_cycle_start;
                    overall_cycle_height =
                        cycle_heights.last().unwrap() - cycle_heights.first().unwrap();
                    break;
                }
            }
        }

        // Drop the rock.
        let new_points = drop_rock(rock, board, &mut jets_iter);
        for p in new_points {
            board.filled_points.insert(p);
            let col = p.0 as usize;
            board.max_heights[col] = cmp::max(board.max_heights[col], p.1);
        }

        drops += 1;
    }

    // We found a cycle:
    //   - It starts at `start_state_cycle_start`,
    //   - At which point the height of the board was `start_state_cycle_start_height`
    //   - It is of length `overall_cycle_length`
    //   - The height added each cycle is `overall cycle heights`
    //   - And the height added during the cycle at each step of the cycle since the cycle
    //     started is in `cycle_heights`.
    println!(
        "Found cycle: start {}, length {}, starting height {}, cycle height increment {}",
        start_state_cycle_start,
        overall_cycle_length,
        start_state_cycle_start_height,
        overall_cycle_height
    );

    // How many complete cycles we'll do.
    let cycle_count = (drop_count - start_state_cycle_start) / overall_cycle_length;
    let cycle_rem = ((drop_count - start_state_cycle_start) % overall_cycle_length) as usize;

    start_state_cycle_start_height + (cycle_count * overall_cycle_height) + cycle_heights[cycle_rem]
}

fn part1(jets: &[Jet]) -> i64 {
    let mut board = Board {
        filled_points: HashSet::new(),
        max_heights: [0; 7],
    };
    drop_rocks(&mut board, jets, 2022);

    *board.max_heights.iter().max().unwrap()
}

fn part2(jets: &[Jet]) -> i64 {
    let mut board = Board {
        filled_points: HashSet::new(),
        max_heights: [0; 7],
    };
    find_height(&mut board, jets, 1000000000000)
}

fn parse_input(raw_jet: &str) -> Vec<Jet> {
    raw_jet
        .chars()
        .map(|c| match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => panic!("Unexpected input char {}", c),
        })
        .collect()
}

fn main() {
    let file = File::open("input").unwrap();
    let mut buffer = BufReader::new(file);
    let mut raw_jets = String::new();
    buffer
        .read_line(&mut raw_jets)
        .expect("Failed to read input");
    let jets = parse_input(raw_jets.as_str());

    let pt1_result = part1(&jets);
    let pt2_result = part2(&jets);

    println!("Part 1: {}, Part 2: {}", pt1_result, pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let jets = parse_input(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        assert_eq!(part1(&jets), 3068);
    }

    #[test]
    fn pt2_test() {
        let jets = parse_input(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        assert_eq!(part2(&jets), 1514285714288);
    }
}
