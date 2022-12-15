use regex::{Captures, Regex};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, PartialEq)]
enum StackMoveOrder {
    Reversed,
    Ordered,
}

struct Move {
    src: usize,
    dst: usize,
    count: usize,
}

#[derive(Clone)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn exec_move(&mut self, src: usize, dst: usize, count: usize, order: StackMoveOrder) {
        let mut moving: Vec<char> = Vec::with_capacity(count);
        for _ in 0..count {
            moving.push(self.stacks[src - 1].pop().unwrap())
        }

        if order == StackMoveOrder::Reversed {
            for c in moving {
                self.stacks[dst - 1].push(c);
            }
        } else {
            for &c in moving.iter().rev() {
                self.stacks[dst - 1].push(c);
            }
        }
    }

    fn move_op(&mut self, op: &Move, order: StackMoveOrder) {
        self.exec_move(op.src, op.dst, op.count, order);
    }

    fn move_ops(&mut self, ops: &Vec<Move>, order: StackMoveOrder) {
        for op in ops {
            self.move_op(op, order);
        }
    }

    fn tops(&self) -> String {
        return self
            .stacks
            .iter()
            .map(|stack| stack.last().map(|&c| c).unwrap_or(' '))
            .collect();
    }
}

fn parse_num(caps: &Captures, label: &str) -> usize {
    return caps.name(label).unwrap().as_str().parse::<usize>().unwrap();
}

fn parse_ops(lines: &Vec<String>) -> Vec<Move> {
    let re = Regex::new(r"^move (?P<count>\d+) from (?P<src>\d+) to (?P<dst>\d+)").unwrap();
    return lines
        .iter()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            let src = parse_num(&caps, "src");
            let dst = parse_num(&caps, "dst");
            let count = parse_num(&caps, "count");
            Move {
                src: src,
                dst: dst,
                count: count,
            }
        })
        .collect();
}

fn part1(stacks: &mut Stacks, ops: &Vec<Move>) -> String {
    stacks.move_ops(&ops, StackMoveOrder::Reversed);
    return stacks.tops();
}

fn part2(stacks: &mut Stacks, ops: &Vec<Move>) -> String {
    stacks.move_ops(&ops, StackMoveOrder::Ordered);
    return stacks.tops();
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let ops = parse_ops(&lines);

    // Hardcoding the stacks layout here, so we're assuming it is removed from
    // the input file.
    let stacks = Stacks {
        stacks: vec![
            vec!['Z', 'P', 'M', 'H', 'R'],
            vec!['P', 'C', 'J', 'B'],
            vec!['S', 'N', 'H', 'G', 'L', 'C', 'D'],
            vec!['F', 'T', 'M', 'D', 'Q', 'S', 'R', 'L'],
            vec!['F', 'S', 'P', 'Q', 'B', 'T', 'Z', 'M'],
            vec!['T', 'F', 'S', 'Z', 'B', 'G'],
            vec!['N', 'R', 'V'],
            vec!['P', 'G', 'L', 'T', 'R', 'D', 'V', 'C', 'M'],
            vec!['W', 'Q', 'N', 'J', 'F', 'M', 'L'],
        ],
    };

    // Clone the stacks so we get the same start for part 1 and 2.
    let mut pt1_stacks = stacks.clone();
    let pt1_result = part1(&mut pt1_stacks, &ops);
    let mut pt2_stacks = stacks.clone();
    let pt2_result = part2(&mut pt2_stacks, &ops);

    println!("Part 1: {}, Part 2: {}", pt1_result, pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let mut stacks = Stacks {
            stacks: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
        };
        let lines: Vec<String> = vec![
            String::from("move 1 from 2 to 1"),
            String::from("move 3 from 1 to 3"),
            String::from("move 2 from 2 to 1"),
            String::from("move 1 from 1 to 2"),
        ];

        let ops = parse_ops(&lines);
        let result = part1(&mut stacks, &ops);
        assert_eq!(result.as_str(), "CMZ");
    }

    #[test]
    fn pt2_test() {
        let mut stacks = Stacks {
            stacks: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
        };
        let lines: Vec<String> = vec![
            String::from("move 1 from 2 to 1"),
            String::from("move 3 from 1 to 3"),
            String::from("move 2 from 2 to 1"),
            String::from("move 1 from 1 to 2"),
        ];

        let ops = parse_ops(&lines);
        let result = part2(&mut stacks, &ops);
        assert_eq!(result.as_str(), "MCD");
    }
}
