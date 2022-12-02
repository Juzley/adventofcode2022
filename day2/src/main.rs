use std::fs::File;
use std::io::{BufRead, BufReader};

// Values are the score for playing that shape.
#[derive(Clone, Copy, PartialEq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Result {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

fn parse_shape(code: char) -> Option<Shape> {
    return match code {
        'A' | 'X' => Some(Shape::Rock),
        'B' | 'Y' => Some(Shape::Paper),
        'C' | 'Z' => Some(Shape::Scissors),
        _ => None,
    };
}

fn parse_result(code: char) -> Option<Result> {
    return match code {
        'X' => Some(Result::Lose),
        'Y' => Some(Result::Draw),
        'Z' => Some(Result::Win),
        _ => None,
    };
}

fn score_turn(them: Shape, us: Shape) -> u32 {
    // Start with the score for playing the shape.
    let base_score = us as u32;

    // Check for a draw
    if us == them {
        return base_score + Result::Draw as u32;
    }

    let result_score = match (us, them) {
        (Shape::Rock, Shape::Scissors) => Result::Win as u32,
        (Shape::Scissors, Shape::Paper) => Result::Win as u32,
        (Shape::Paper, Shape::Rock) => Result::Win as u32,
        _ => Result::Lose as u32,
    };

    return base_score + result_score;
}

fn select_shape(them: Shape, result: Result) -> Shape {
    return match (them, result) {
        (Shape::Rock, Result::Win) => Shape::Paper,
        (Shape::Rock, Result::Draw) => Shape::Rock,
        (Shape::Rock, Result::Lose) => Shape::Scissors,
        (Shape::Paper, Result::Win) => Shape::Scissors,
        (Shape::Paper, Result::Draw) => Shape::Paper,
        (Shape::Paper, Result::Lose) => Shape::Rock,
        (Shape::Scissors, Result::Win) => Shape::Rock,
        (Shape::Scissors, Result::Draw) => Shape::Scissors,
        (Shape::Scissors, Result::Lose) => Shape::Paper,
    };
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    // Parse lines into the individual chars representing each turn.
    let lines: Vec<(char, char)> = reader
        .lines()
        .map(|l| l.unwrap())
        .filter_map(|line| {
            let parts: Vec<char> = line
                .trim()
                .split(' ')
                .filter_map(|s| s.chars().next())
                .collect();

            if parts.len() != 2 {
                return None;
            }
            return Some((parts[0], parts[1]));
        })
        .collect();

    // Part 1
    let mut pt1_score = 0;
    for line in &lines {
        pt1_score += score_turn(parse_shape(line.0).unwrap(), parse_shape(line.1).unwrap());
    }

    let mut pt2_score = 0;
    for line in &lines {
        let them = parse_shape(line.0).unwrap();
        let result = parse_result(line.1).unwrap();

        pt2_score += score_turn(them, select_shape(them, result));
    }

    println!("Part 1: {}, Part 2: {}", pt1_score, pt2_score);
}
