use std::collections::HashSet;

use advent_of_code::helpers;
use log::{debug, info};


fn main() {
    helpers::init();
    info!("Start day 10 challenge...");

    let map = parse();

    info!("Solution for PART ONE is {}", part_one(map.clone()));
    info!("Solution for PART TWO is {}", part_two());
}

fn part_one(map: Vec<Vec<i32>>) -> usize {
    // A hiking trail is any path that starts at height 0, ends at height 9, and always increases by a height of exactly 1 at each step. Hiking trails never include diagonal steps - only up, down, left, or right (from the perspective of the map).
    // A trailhead is any position that starts one or more hiking trails - here, these positions will always have height 0
    let mut trailheads: Vec<(i32, i32)> = Vec::new(); 
    let mut return_value = 0;

    // Get all trailhead coordinates
    for (row, row_vec) in map.iter().enumerate() {
        for (col, &height) in row_vec.iter().enumerate() {
            if height == 0 {
                trailheads.push((row as i32, col as i32));
            }
        }
    }
    debug!("Number of trailheads found: {}", trailheads.len());

    // Iterate over trailheads
    for trailhead in trailheads {
        let mut result: HashSet<(i32, i32)> = HashSet::new(); 
        let mut stack: Vec<(i32, i32)> = vec![trailhead];

        // Implement DFS to find all legitimate paths
        while let Some((row, col)) = stack.pop() {
            if map[row as usize][col as usize] == 9 { 
                result.insert((row, col));
                continue; 
            };

        // Add all surrounding positions with +1 height
        for position in [(row+1, col), (row-1, col), (row, col+1), (row, col-1)] {
                if in_bounds(position, map.len(), map[0].len()) && map[position.0 as usize][position.1 as usize] == map[row as usize][col as usize] + 1 {
                    stack.push(position);
                }
            }
        }

        return_value += result.len();
    }

    return_value
}

fn part_two() -> usize {
    let result = 0;
    result 
}

fn parse() -> Vec<Vec<i32>> {
    // The topographic map indicates the height at each position using a scale from 0 (lowest) to 9 (highest)
    // Parse input as matrix
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    for line in include_bytes!("../input.txt").split(|&byte| byte == b'\n') {
        let newline: Vec<i32> = Vec::from(line).iter().map(|&byte| (byte as char).to_digit(10).unwrap() as i32).collect();
        matrix.push(newline);
    }

    debug!("Parsed Matrix {:?}", matrix);
    matrix
}

fn in_bounds(p: (i32, i32), row_length: usize, col_length: usize) -> bool {
    p.0 >= 0 && p.0 < row_length as i32 && p.1 >= 0 && p.1 < col_length as i32
}