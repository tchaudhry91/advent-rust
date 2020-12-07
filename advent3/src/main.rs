extern crate failure;

use std::fs::File;
use std::io::{prelude::*, BufReader};

use failure::Error;

fn get_expanded_map(filename: String, slope: f64) -> Result<(Vec<bool>, usize, usize), Error> {
    let file = File::open(filename.clone())?;
    let reader = BufReader::new(file);

    let mut tree_map = vec![];

    let mut repeat_len = 0;

    let mut depth = 0;
    // Calculate the depth first
    // Unfortunate that I have read lines to get the total number.
    // Basically need a substitute for wc -l
    for line in reader.lines() {
        repeat_len = line?.chars().count();
        depth += 1;
    }

    // Calculate the needed width
    let width_requirement = slope * (depth as f64);
    let repeat_mul = find_nearest_multiple(width_requirement, repeat_len);
    println!("Depth: {} SingleWidth: {} WidthRequirement: {} RepeatMul: {}", depth, repeat_len, width_requirement, repeat_mul);

    // Re-read file
    // Find a better way to re-use the reader above.
    // Seek complains about moved value
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut line_vec = vec![];
        for c in line?.chars() {
            if c == '.' {
                // No Tree Present
                line_vec.push(false);
                continue;
            }
            // Tree Present
            line_vec.push(true);
        }
        for _ in 1..=repeat_mul {
            tree_map.extend_from_slice(&line_vec);
        }
    }

    Ok((tree_map, (repeat_mul * repeat_len) as usize, depth as usize))
}

fn find_nearest_multiple(val: f64, num: usize) -> usize {
    return (val / num as f64).ceil() as usize;
}

fn traverse_path(right_inc: usize, down_inc: usize, tree_map: Vec<bool>, line_width: usize, depth: usize) -> usize {
    let mut current_coord = 0;
    let mut current_depth = 0;

    let mut trees_hit = 0;

    while current_depth < depth {
        if tree_map[current_coord] {
            trees_hit += 1;
        }
        current_coord = get_next_coordinate_index(current_coord, right_inc, down_inc, line_width);
        current_depth += down_inc;
    }
    trees_hit
}

fn get_next_coordinate_index(current: usize, right_inc: usize, down_inc: usize, line_width:usize) -> usize {
    let next = current + (down_inc * line_width) + right_inc;
    next
}

fn main() {
    let down_inc = 1;
    let right_inc = 3;
    let slope = right_inc as f64 / down_inc as f64;

    // Get a 1D map of requisite size
    let (tree_map, line_width, depth) = get_expanded_map("input.txt".to_string(), slope).expect("Unable to build map");

    // Traverse It
    let trees_hit = traverse_path(right_inc, down_inc, tree_map, line_width, depth);
    println!("Trees Hit: {}", trees_hit);


    // Part - 2
    // Check More Slopes
    let mut trees_hit_mul = 1;
    let (tree_map, line_width, depth) = get_expanded_map("input.txt".to_string(), 1.0).expect("Unable to build map");
    let right_inc = 1;
    let down_inc = 1;
    let trees_hit = traverse_path(right_inc, down_inc, tree_map, line_width, depth);
    trees_hit_mul = trees_hit_mul * trees_hit;

    let (tree_map, line_width, depth) = get_expanded_map("input.txt".to_string(), 3.0).expect("Unable to build map");
    let right_inc = 3;
    let down_inc = 1;
    let trees_hit = traverse_path(right_inc, down_inc, tree_map, line_width, depth);
    trees_hit_mul = trees_hit_mul * trees_hit;

    let (tree_map, line_width, depth) = get_expanded_map("input.txt".to_string(), 5.0).expect("Unable to build map");
    let right_inc = 5;
    let down_inc = 1;
    let trees_hit = traverse_path(right_inc, down_inc, tree_map, line_width, depth);
    trees_hit_mul = trees_hit_mul * trees_hit;

    let (tree_map, line_width, depth) = get_expanded_map("input.txt".to_string(), 7.0).expect("Unable to build map");
    let right_inc = 7;
    let down_inc = 1;
    let trees_hit = traverse_path(right_inc, down_inc, tree_map, line_width, depth);
    trees_hit_mul = trees_hit_mul * trees_hit;

    let (tree_map, line_width, depth) = get_expanded_map("input.txt".to_string(), 0.5).expect("Unable to build map");
    let right_inc = 1;
    let down_inc = 2;
    let trees_hit = traverse_path(right_inc, down_inc, tree_map, line_width, depth);
    trees_hit_mul = trees_hit_mul * trees_hit;

    println!("Multiplaction of Trees Hit: {}", trees_hit_mul);
}
