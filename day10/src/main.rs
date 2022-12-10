use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Debug)]
enum Operation {
    AddX(i32),
    Noop,
}

fn parse_op(line: &String) -> Option<Operation> {
    let mut parts = line.split(' ');
    return match parts.next()? {
        "addx" => {
            let val = parts.next()?.parse::<i32>().ok()?;
            Some(Operation::AddX(val))
        }
        "noop" => Some(Operation::Noop),
        _ => None,
    };
}

fn parse_lines(lines: &Vec<String>) -> Vec<Operation> {
    return lines
        .iter()
        .map(|line| parse_op(line).expect("Failed to parse line"))
        .collect();
}

fn pipeline_wait(op: Operation) -> u32 {
    return match op {
        Operation::AddX(_) => 2,
        Operation::Noop => 1,
    };
}

fn execute(ops: &Vec<Operation>) -> [i32; 240] {
    let mut x: i32 = 1;

    let mut current_op = None;
    let mut wait = 0;
    let mut cycles = 0;
    let mut op_index = 0;

    let mut output: [i32; 240] = [0; 240];

    loop {
        cycles += 1;

        // Start of the cycle - start a new op if required.
        if current_op.is_none() {
            if op_index < ops.len() {
                let new_op = ops[op_index];
                op_index += 1;
                wait = pipeline_wait(new_op);
                current_op = Some(new_op);
            }
        }

        // Middle of the cycle.
        output[cycles - 1] = x;
        if cycles == 240 {
            return output;
        }

        // End of the cycle, finalize any op that has finished.
        if let Some(op) = current_op {
            wait -= 1;

            if wait == 0 {
                match op {
                    Operation::AddX(val) => x += val,
                    Operation::Noop => (),
                }
                current_op = None;
            }
        }
    }
}

fn build_image(sprite_pos: [i32; 240]) -> [char; 240] {
    let mut output = ['.'; 240];

    for scan in 0..240 {
        let sprite_center = sprite_pos[scan];
        let sprite_range = (sprite_center - 1)..=(sprite_center + 1);
        let current_col = (scan % 40) as i32;

        if sprite_range.contains(&current_col) {
            output[scan] = '#';
        }
    }

    return output;
}

fn print_screen(screen: [char; 240]) {
    for chars in screen.chunks(40) {
        let line: String = chars.iter().collect();
        println!("{}", line);
    }
}

fn part1(ops: &Vec<Operation>) -> i32 {
    let output = execute(ops);
    let mut result = 0;

    for i in (20..=240).step_by(40) {
        result += i as i32 * output[i - 1];
    }

    return result;
}

fn part2(ops: &Vec<Operation>) -> [char; 240] {
    let sprite_pos = execute(ops);
    return build_image(sprite_pos);
}

fn read_input(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    return reader
        .lines()
        .map(|l| String::from(l.unwrap().trim()))
        .collect();
}

fn main() {
    let lines = read_input("input");
    let ops = parse_lines(&lines);

    let pt1_result = part1(&ops);
    let pt2_screen = part2(&ops);

    println!("Part 1: {}", pt1_result);
    println!("Part 2:");
    print_screen(pt2_screen);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let lines = read_input("test_input");
        let ops = parse_lines(&lines);
        let result = part1(&ops);

        assert_eq!(result, 13140);
    }

    #[test]
    fn pt2_test() {
        let lines = read_input("test_input");
        let ops = parse_lines(&lines);
        let result = part2(&ops);

        // `cargo test -- --nocapture` to see the output
        print_screen(result);
    }
}
