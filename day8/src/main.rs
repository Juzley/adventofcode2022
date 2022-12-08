use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

#[derive(Clone, Copy, Debug)]
struct Visibility {
    north: usize,
    east: usize,
    south: usize,
    west: usize,
}

fn parse_lines(lines: &Vec<String>) -> Vec<Vec<usize>> {
    return lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("Failed to parse tree height") as usize)
                .collect()
        })
        .collect();
}

// Find the distance from tree at a given coord, to the nearest tree that blocks it.
fn find_blocking_tree(
    nearest_trees: [Option<usize>; 10],
    height: usize,
    position: usize,
) -> Option<usize> {
    let mut distance: Option<usize> = None;

    // We have the nearest tree to us for each height from 0-10. Loop through the ones that are
    // of equal height or taller to the current tree, and find the nearest.
    for i in height..10 {
        distance = match (nearest_trees[i as usize], distance) {
            // We already found a blocking tree, and we have a new candidate - find out if it
            // is nearer.
            (Some(other_position), Some(cur_distance)) => {
                let new_distance = (position as i32 - other_position as i32).abs() as usize;
                Some(cmp::min(new_distance, cur_distance))
            }

            // We have a tree that this height, and haven't found another tree yet, so this is
            // the closest current blocking tree.
            (Some(other_position), None) => {
                Some((position as i32 - other_position as i32).abs() as usize)
            }

            // No tree of this height, carry on.
            (None, cur_distance) => cur_distance,
        }
    }

    return distance;
}

// Calculate the scenic scores for each tree, and find the max over the entire forest.
fn find_max_scenic_score(trees: &Vec<Vec<usize>>) -> usize {
    let height = trees.len();
    let width = trees[0].len();

    let mut visibility: Vec<Visibility> = iter::repeat_with(|| Visibility {
        north: 0,
        east: 0,
        south: 0,
        west: 0,
    })
    .take(width * height)
    .collect();

    // Keep track of the indices of the most recent tree of each size that we have
    // seen while traversing in a given direction. For each tree we visit we can then
    // look for any taller trees that would block visibility, and find the closest one.
    // This allows us to calculate the max scene score in linear time.
    let mut nearest_trees: [Option<usize>; 10];

    for col_index in 0..width {
        // Work down each column, calculating the northwards visibility
        nearest_trees = [None; 10];
        for row_index in 0..height {
            let tree_height = trees[row_index][col_index];
            let blocking = find_blocking_tree(nearest_trees, tree_height, row_index);
            nearest_trees[tree_height] = Some(row_index);

            // If we didn't find a blocking tree, visibility is the number of trees to the
            // edge of the forest, which is the position.
            visibility[row_index * width + col_index].north = blocking.unwrap_or(row_index);
        }

        // Work up each column, calculating the southwards visibility
        nearest_trees = [None; 10];
        for row_index in (0..height).rev() {
            let tree_height = trees[row_index][col_index];
            let blocking = find_blocking_tree(nearest_trees, tree_height, row_index);

            // If we didn't find a blocking tree, visibility is the number of trees to the
            // edge of the forest, which can be calculated from the position
            visibility[row_index * width + col_index].south =
                blocking.unwrap_or(height - row_index - 1);

            nearest_trees[tree_height] = Some(row_index);
        }
    }

    for row_index in 0..height {
        // Work left to right along each row, calculating westwards visibility
        nearest_trees = [None; 10];
        for col_index in 0..width {
            let tree_height = trees[row_index][col_index];
            let blocking = find_blocking_tree(nearest_trees, tree_height, col_index);

            // Visibility calc as for the northward case, but with column index.
            visibility[row_index * width + col_index].west = blocking.unwrap_or(col_index);

            nearest_trees[tree_height] = Some(col_index);
        }

        // Work right to left along each row, calculating eastwards visibility
        nearest_trees = [None; 10];
        for col_index in (0..width).rev() {
            let tree_height = trees[row_index][col_index];
            let blocking = find_blocking_tree(nearest_trees, tree_height, col_index);

            // Visilibity calc as for the southwards case, but with column index.
            visibility[row_index * width + col_index].east =
                blocking.unwrap_or(width - col_index - 1);

            nearest_trees[tree_height] = Some(col_index);
        }
    }

    return visibility
        .iter()
        .map(|v| v.north * v.east * v.south * v.west)
        .max()
        .expect("Failed to find max score");
}

// Helper function for count_visible
fn update_visible(visible: &mut bool, max_height: Option<usize>, height: usize) -> Option<usize> {
    if let Some(h) = max_height {
        if height > h {
            *visible = true;
            return Some(height);
        }
    } else {
        *visible = true;
        return Some(height);
    }

    return max_height;
}

// Count the number of trees visible from the outside of the forest
fn count_visible(trees: &Vec<Vec<usize>>) -> usize {
    let height = trees.len();
    let width = trees[0].len();

    // Whether the tree at the given coordinate is visible.
    let mut visible: Vec<bool> = iter::repeat_with(|| false).take(width * height).collect();

    // The maximum height we've seen in each column.
    let mut max_height: Option<usize>;

    for col_index in 0..width {
        // Work down each column
        max_height = None;
        for row_index in 0..height {
            max_height = update_visible(
                &mut visible[row_index * width + col_index],
                max_height,
                trees[row_index][col_index],
            );
        }

        // Work up each column
        max_height = None;
        for row_index in (0..height).rev() {
            max_height = update_visible(
                &mut visible[row_index * width + col_index],
                max_height,
                trees[row_index][col_index],
            );
        }
    }

    for row_index in 0..height {
        // Work along each column left to right
        max_height = None;
        for col_index in 0..width {
            max_height = update_visible(
                &mut visible[row_index * width + col_index],
                max_height,
                trees[row_index][col_index],
            );
        }

        // Work along each column right to left
        max_height = None;
        for col_index in (0..width).rev() {
            max_height = update_visible(
                &mut visible[row_index * width + col_index],
                max_height,
                trees[row_index][col_index],
            );
        }
    }

    return visible.iter().filter(|&v| *v).count();
}

fn part1(trees: &Vec<Vec<usize>>) -> usize {
    return count_visible(&trees);
}

fn part2(trees: &Vec<Vec<usize>>) -> usize {
    return find_max_scenic_score(&trees);
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|l| String::from(l.unwrap().trim()))
        .collect();
    let trees = parse_lines(&lines);

    let pt1_result = part1(&trees);
    let pt2_result = part2(&trees);

    println!("Part 1: {}, Part 2: {}", pt1_result, pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let lines = vec![
            String::from("30373"),
            String::from("25512"),
            String::from("65332"),
            String::from("33549"),
            String::from("35390"),
        ];

        let trees = parse_lines(&lines);
        let result = part1(&trees);
        assert_eq!(result, 21);
    }

    #[test]
    fn pt2_test() {
        let lines = vec![
            String::from("30373"),
            String::from("25512"),
            String::from("65332"),
            String::from("33549"),
            String::from("35390"),
        ];

        let trees = parse_lines(&lines);
        let result = part2(&trees);
        assert_eq!(result, 8);
    }
}
