use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn calc_priority(c: char) -> u32 {
    return match c {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => 0,
    };
}

fn part1(lines: &Vec<Vec<char>>) -> u32 {
    let mut sum = 0;
    for line in lines {
        let compartments: Vec<HashSet<char>> = line
            .chunks(line.len() / 2)
            .map(|l| l.iter().cloned().collect())
            .collect();
        let c = *compartments[0]
            .intersection(&compartments[1])
            .next()
            .unwrap();
        sum += calc_priority(c);
    }

    return sum;
}

fn part2(lines: &Vec<Vec<char>>) -> u32 {
    let chunks = lines.chunks(3);

    let mut sum = 0;
    for chunk in chunks {
        let chunk_sets: Vec<HashSet<char>> = chunk
            .iter()
            .map(|l| HashSet::from_iter(l.iter().copied()))
            .collect();
        let first: HashSet<char> = chunk_sets[0]
            .intersection(&chunk_sets[1])
            .copied()
            .collect();
        let c = *first.intersection(&chunk_sets[2]).next().unwrap();

        sum += calc_priority(c);
    }

    return sum;
}

fn main() {
    println!("Hello, world!");

    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let pt1_result = part1(&lines);
    let pt2_result = part2(&lines);

    println!("Part 1: {}, Part 2: {}", pt1_result, pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let lines = vec![
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp").chars().collect(),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")
                .chars()
                .collect(),
            String::from("PmmdzqPrVvPwwTWBwg").chars().collect(),
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn")
                .chars()
                .collect(),
            String::from("ttgJtRGJQctTZtZT").chars().collect(),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw").chars().collect(),
        ];
        let result = part1(&lines);
        assert_eq!(result, 157);
    }

    #[test]
    fn pt2_test() {
        let lines = vec![
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp").chars().collect(),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")
                .chars()
                .collect(),
            String::from("PmmdzqPrVvPwwTWBwg").chars().collect(),
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn")
                .chars()
                .collect(),
            String::from("ttgJtRGJQctTZtZT").chars().collect(),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw").chars().collect(),
        ];
        let result = part2(&lines);
        assert_eq!(result, 70);
    }
}
