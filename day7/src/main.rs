use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const MAX_SIZE: usize = 100000;
const DISK_SIZE: usize = 70000000;
const REQUIRED_SIZE: usize = 30000000;

fn handle_cd(dir: &str, cur_dirs: &mut Vec<String>) {
    if dir == "/" {
        cur_dirs.clear();
        cur_dirs.push(String::from("/"));
    } else if dir == ".." {
        cur_dirs.pop();
    } else {
        cur_dirs.push(String::from(dir));
    }
}

fn handle_file(line: &str, cur_dirs: &Vec<String>, sizes: &mut HashMap<String, usize>) {
    let size = line
        .split(' ')
        .next()
        .and_then(|v| v.parse::<usize>().ok())
        .expect("Failed to parse file size");

    let mut full_dir = String::new();
    for dir in cur_dirs {
        full_dir.push_str(dir.as_str());
        match sizes.get_mut(&full_dir) {
            Some(s) => *s += size,
            None => {
                sizes.insert(full_dir.clone(), size);
            }
        }
    }
}

fn dir_sizes(lines: &Vec<String>) -> HashMap<String, usize> {
    let mut cur_dirs = vec![String::from("/")];
    let mut sizes = HashMap::from([(String::from("/"), 0)]);

    for line in lines {
        match &line[..4] {
            "$ cd" => handle_cd(&line[5..], &mut cur_dirs),
            "$ ls" => (),
            "dir " => (),
            _ => handle_file(&line, &cur_dirs, &mut sizes),
        }
    }

    return sizes;
}

fn part1(lines: &Vec<String>) -> usize {
    let sizes = dir_sizes(lines);
    let mut result = 0;

    for (_, size) in sizes {
        if size <= MAX_SIZE {
            result += size;
        }
    }

    return result;
}

fn part2(lines: &Vec<String>) -> usize {
    let sizes = dir_sizes(lines);
    let total_size = sizes["/"];
    let min_free = REQUIRED_SIZE - (DISK_SIZE - total_size);
    let mut possible_dirs: Vec<usize> = sizes
        .iter()
        .filter_map(|(_, &s)| if s >= min_free { Some(s) } else { None })
        .collect();
    possible_dirs.sort();
    return possible_dirs[0];
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|l| String::from(l.unwrap().trim()))
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
            String::from("$ cd /"),
            String::from("$ ls"),
            String::from("dir a"),
            String::from("14848514 b.txt"),
            String::from("8504156 c.dat"),
            String::from("dir d"),
            String::from("$ cd a"),
            String::from("$ ls"),
            String::from("dir e"),
            String::from("29116 f"),
            String::from("2557 g"),
            String::from("62596 h.lst"),
            String::from("$ cd e"),
            String::from("$ ls"),
            String::from("584 i"),
            String::from("$ cd .."),
            String::from("$ cd .."),
            String::from("$ cd d"),
            String::from("$ ls"),
            String::from("4060174 j"),
            String::from("8033020 d.log"),
            String::from("5626152 d.ext"),
            String::from("7214296 k"),
        ];

        let result = part1(&lines);
        assert_eq!(result, 95437);
    }

    #[test]
    fn pt2_test() {
        let lines = vec![
            String::from("$ cd /"),
            String::from("$ ls"),
            String::from("dir a"),
            String::from("14848514 b.txt"),
            String::from("8504156 c.dat"),
            String::from("dir d"),
            String::from("$ cd a"),
            String::from("$ ls"),
            String::from("dir e"),
            String::from("29116 f"),
            String::from("2557 g"),
            String::from("62596 h.lst"),
            String::from("$ cd e"),
            String::from("$ ls"),
            String::from("584 i"),
            String::from("$ cd .."),
            String::from("$ cd .."),
            String::from("$ cd d"),
            String::from("$ ls"),
            String::from("4060174 j"),
            String::from("8033020 d.log"),
            String::from("5626152 d.ext"),
            String::from("7214296 k"),
        ];

        let result = part2(&lines);
        assert_eq!(result, 24933642);
    }
}
