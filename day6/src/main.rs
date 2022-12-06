mod input;

use std::collections::HashSet;

fn find_marker(buf: &str, marker_size: usize) -> Option<usize> {
    let mut count = marker_size;
    for window in buf.as_bytes().windows(marker_size) {
        let set: HashSet<u8> = window.iter().cloned().collect();
        if set.len() == marker_size {
            return Some(count);
        }

        count += 1;
    }

    return None;
}

fn part1(input: &str) -> usize {
    return find_marker(input, 4).unwrap();
}

fn part2(input: &str) -> usize {
    return find_marker(input, 14).unwrap();
}

fn main() {
    let pt1_result = part1(input::INPUT);
    let pt2_result = part2(input::INPUT);
    println!("Part 1: {}, Part 2: {}", pt1_result, pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn pt2_test() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
