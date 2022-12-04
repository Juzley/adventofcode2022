use regex::{Captures, Regex};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

fn parse_num(caps: &Captures, label: &str) -> u32 {
    return caps.name(label).unwrap().as_str().parse::<u32>().unwrap();
}

fn parse_lines(lines: &Vec<String>) -> Vec<(Range<u32>, Range<u32>)> {
    let re = Regex::new(r"^(?P<s1>\d+)-(?P<e1>\d+),(?P<s2>\d+)-(?P<e2>\d+)").unwrap();
    return lines
        .into_iter()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            return (
                parse_num(&caps, "s1")..parse_num(&caps, "e1"),
                parse_num(&caps, "s2")..parse_num(&caps, "e2"),
            );
        })
        .collect();
}

fn part1(lines: &Vec<String>) -> usize {
    let ranges = parse_lines(lines);
    return ranges
        .iter()
        .filter({
            |(r1, r2)| {
                return (r1.start >= r2.start && r1.end <= r2.end)
                    || (r2.start >= r1.start && r2.end <= r1.end);
            }
        })
        .count();
}

fn part2(lines: &Vec<String>) -> usize {
    let ranges = parse_lines(lines);
    return ranges
        .iter()
        .filter(|(r1, r2)| r1.end >= r2.start && r2.end >= r1.start)
        .count();
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let part1_result = part1(&lines);
    let part2_result = part2(&lines);

    println!("Part 1: {}, Part 2: {}", part1_result, part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let lines = vec![
            String::from("2-4,6-8"),
            String::from("2-3,4-5"),
            String::from("5-7,7-9"),
            String::from("2-8,3-7"),
            String::from("6-6,4-6"),
            String::from("2-6,4-8"),
        ];
        let result = part1(&lines);
        assert_eq!(result, 2);
    }

    #[test]
    fn pt2_test() {
        let lines = vec![
            String::from("2-4,6-8"),
            String::from("2-3,4-5"),
            String::from("5-7,7-9"),
            String::from("2-8,3-7"),
            String::from("6-6,4-6"),
            String::from("2-6,4-8"),
        ];
        let result = part2(&lines);
        assert_eq!(result, 4);
    }
}
