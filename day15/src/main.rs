use regex::{Captures, Regex};
use std::cmp::{max, Ordering};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

type Point = (i32, i32);

#[derive(Debug)]
struct Sensor {
    loc: Point,
    beacon: Point,
}

fn parse_num(caps: &Captures, label: &str) -> i32 {
    caps.name(label)
        .unwrap()
        .as_str()
        .parse::<i32>()
        .expect("Failed to parse coordinate")
}

fn parse_lines(lines: &[String]) -> Vec<Sensor> {
    let re = Regex::new(r"^Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)").expect("Failed to build regex");

    lines
        .iter()
        .map(|l| {
            let caps = re.captures(l).expect("Failed to match line");
            Sensor {
                loc: (parse_num(&caps, "sx"), parse_num(&caps, "sy")),
                beacon: (parse_num(&caps, "bx"), parse_num(&caps, "by")),
            }
        })
        .collect()
}

fn coverage_for_sensor(
    sensor: &Sensor,
    target_row: i32,
    skip_beacon: bool,
) -> Option<RangeInclusive<i32>> {
    let beacon_distance =
        (sensor.beacon.0 - sensor.loc.0).abs() + (sensor.beacon.1 - sensor.loc.1).abs();
    let row_distance = (sensor.loc.1 - target_row).abs();

    // Row width is the width of the target row that is covered by this sensor. If it isn't positive
    // that means the sensor is closer, or as close, to the beacon than the target row, so doesn't
    // cover any of that row. Note that this is actually half the actual row width.
    let row_width = beacon_distance - row_distance;
    if row_width <= 0 {
        return None;
    }

    let mut start = sensor.loc.0 - row_width;
    let mut end = sensor.loc.0 + row_width;

    // If the beacon is in this row, exclude it.
    if skip_beacon && sensor.beacon.1 == target_row {
        if sensor.beacon.0 == start {
            start += 1;
        } else {
            end -= 1;
        }
    }

    Some(start..=end)
}

// Count the number of definitely-empty spaces in a line
fn count_empty(sensors: &[Sensor], target_row: i32) -> i32 {
    let mut ranges = vec![];

    for sensor in sensors {
        if let Some(range) = coverage_for_sensor(sensor, target_row, true) {
            ranges.push(range);
        }
    }

    // Sort the ranges so that we can find if there any overlaps.
    ranges.sort_by(|a, b| {
        let cmp = a.start().cmp(b.start());

        if cmp != Ordering::Equal {
            cmp
        } else {
            a.end().cmp(b.end())
        }
    });

    let mut empty_count = 0;
    let mut prev_range: Option<RangeInclusive<i32>> = None;
    for cur in ranges {
        if let Some(prev) = prev_range {
            if cur.end() <= prev.end() {
                // Completely overlapped by the previous range, skip it.
            } else if cur.start() <= prev.end() {
                // Partial overlap with previous range - add just the non-overlapping bits.
                empty_count += cur.end() - prev.end();
            } else {
                // No overlap with previous range - add the full range.
                empty_count += cur.end() - cur.start() + 1;
            }
        } else {
            empty_count += cur.end() - cur.start() + 1;
        }

        prev_range = Some(cur);
    }

    empty_count
}

// For part 2, find if a given row has a gap where in coverage where a beacon could be.
fn find_hidden_beacon_in_row(
    sensors: &[Sensor],
    target_row: i32,
    cols: &RangeInclusive<i32>,
) -> Option<i32> {
    let mut ranges = vec![];

    for sensor in sensors {
        if let Some(range) = coverage_for_sensor(sensor, target_row, false) {
            if range.end() >= cols.start() && range.start() <= cols.end() {
                ranges.push(range);
            }
        }
    }

    ranges.sort_by(|a, b| {
        let cmp = a.start().cmp(b.start());

        if cmp != Ordering::Equal {
            cmp
        } else {
            a.end().cmp(b.end())
        }
    });

    // Throughout this we assume that there will only be a single location that
    // a hidden beacon could be.
    let mut maybe_max: Option<i32> = None;
    for range in ranges {
        if let Some(cur_max) = maybe_max {
            if *range.start() > cur_max + 1 {
                // There is a gap between the ranges, the hidden beacon must be in the cap.
                return Some(cur_max + 1);
            }
            maybe_max = Some(max(cur_max, *range.end()));
        } else {
            if range.start() > cols.start() {
                // If the first range doesn't cover the beginning of the row, the
                // hidden beacon must be at the start of the row.
                return Some(*cols.start());
            }
            maybe_max = Some(*range.end());
        }
    }

    if let Some(cur_max) = maybe_max {
        if cur_max < *cols.end() {
            // We didn't reach the end of the row, the hidden beacon must
            // be at the end of the row.
            return Some(*cols.end());
        }
    }

    None
}

fn find_hidden_beacon(
    sensors: &[Sensor],
    rows: RangeInclusive<i32>,
    cols: RangeInclusive<i32>,
) -> Option<Point> {
    for row in rows {
        if let Some(x) = find_hidden_beacon_in_row(sensors, row, &cols) {
            return Some((x, row));
        }
    }

    None
}

fn find_tuning_freq(
    sensors: &[Sensor],
    rows: RangeInclusive<i32>,
    cols: RangeInclusive<i32>,
) -> Option<i64> {
    find_hidden_beacon(sensors, rows, cols).map(|loc| loc.0 as i64 * 4000000 + loc.1 as i64)
}

fn part1(sensors: &[Sensor]) -> i32 {
    count_empty(sensors, 2000000)
}

fn part2(sensors: &[Sensor]) -> i64 {
    find_tuning_freq(sensors, 0..=4000000, 0..=4000000).expect("Failed to find hidden beacon")
}

fn read_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|l| String::from(l.unwrap().trim()))
        .filter(|l| !l.is_empty())
        .collect();
    lines
}

fn main() {
    let lines = read_file("input");
    let sensors = parse_lines(&lines);

    let pt1_result = part1(&sensors);
    let pt2_result = part2(&sensors);

    println!("Part 1: {}, Part 2: {}", pt1_result, pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let lines = read_file("test_input");
        let sensors = parse_lines(&lines);
        let result = count_empty(&sensors, 10);
        assert_eq!(result, 26);
    }

    #[test]
    fn pt2_test() {
        let lines = read_file("test_input");
        let sensors = parse_lines(&lines);
        let result = find_tuning_freq(&sensors, 0..=20, 0..=20);
        assert_eq!(result, Some(56000011));
    }
}
