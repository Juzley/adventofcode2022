use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut elves = Vec::new();
    let mut cals = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            elves.push(cals);
            cals = 0;
        } else {
            cals += line.parse::<i64>().unwrap();
        }
    }
    elves.sort_by(|a, b| b.cmp(a));
        
    println!("Part 1: {}, Part 2: {}", elves[0], elves[0] + elves[1] + elves[2]);
}
