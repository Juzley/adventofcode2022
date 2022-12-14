mod elem;

use crate::elem::Elem;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_line(line: &String, divider: bool) -> Option<Elem> {
    let mut list_stack = Vec::new();
    let mut num_buf = String::new();

    // skip the first character, which is opening the root char.
    let mut chars = line.chars();

    loop {
        let mut c = chars.next()?;

        // Look for numbers, add complete ones to the currently open list.
        while c.is_numeric() {
            num_buf.push(c);
            c = chars.next()?;
        }

        if !num_buf.is_empty() {
            let num = num_buf.parse::<u32>().ok()?;
            let current_elem: &mut Elem = list_stack.last_mut()?;
            current_elem.list.as_mut()?.push(Elem {
                list: None,
                number: Some(num),
                divider: divider,
            });
            num_buf.clear();
        }

        match c {
            '[' => list_stack.push(Elem {
                list: Some(Vec::new()),
                number: None,
                divider: divider,
            }),
            ']' => {
                let popped = list_stack.pop()?;

                if list_stack.is_empty() {
                    return Some(popped);
                } else {
                    let parent = list_stack.last_mut()?;
                    parent.list.as_mut()?.push(popped);
                }
            }
            ',' => (),
            _ => return None,
        }
    }
}

fn parse_line_or_panic(line: &String, divider: bool) -> Elem {
    parse_line(line, divider).expect("Failed to parse line")
}

fn parse_lines(lines: &Vec<String>) -> Vec<Elem> {
    lines
        .iter()
        .map(|l| parse_line_or_panic(&l, false))
        .collect()
}

fn part1(elems: &Vec<Elem>) -> usize {
    return elems
        .chunks(2)
        .map(|pair| pair[0].cmp(&pair[1]))
        .enumerate()
        .map(|(i, c)| if c == Ordering::Less { i + 1 } else { 0 })
        .sum();
}

fn part2(elems: &Vec<Elem>) -> usize {
    let mut sorted = elems.clone();
    sorted.push(parse_line_or_panic(&String::from("[[2]]"), true));
    sorted.push(parse_line_or_panic(&String::from("[[6]]"), true));
    sorted.sort();

    sorted
        .iter()
        .enumerate()
        .map(|(i, elem)| if elem.divider { i + 1 } else { 1 })
        .product()
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
    let elems = parse_lines(&lines);

    let pt1_result = part1(&elems);
    let pt2_result = part2(&elems);

    println!("Part 1: {}, Part 2: {}", pt1_result, pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let lines = read_file("test_input");
        let elems = parse_lines(&lines);

        let result = part1(&elems);
        assert_eq!(result, 13);
    }

    #[test]
    fn pt2_test() {
        let lines = read_file("test_input");
        let elems = parse_lines(&lines);

        let result = part2(&elems);
        assert_eq!(result, 140);
    }
}
