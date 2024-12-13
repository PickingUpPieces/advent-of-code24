use std::collections::HashSet;

use advent_of_code::helpers;
use log::{debug, info};

fn main() {
    helpers::init();
    info!("Start day 12 challenge...");

    info!("Solution for PART ONE is {}", part_one());
    info!("Solution for PART TWO is {}", part_two());
}

fn part_one() -> usize {
    // Store all discovered new regions
    let graph = parse();

    let mut discovered: Vec<(usize, usize)> = Vec::new(); 
    discovered.push((0,0));
    let mut visited: HashSet<(usize, usize)> = HashSet::new(); 
    let mut cost: usize = 0;
    // Start with the zero coordinate

    // Iterate over discovered
    while !discovered.is_empty() {
        let tile = discovered.pop().unwrap();
        // Check if tile is already part of a region
        if visited.contains(&tile) { continue; }
        debug!("Starting to discover region {}", graph[tile.0][tile.1]);

        // Do DFS to discover region
        let mut tiles: Vec<(usize, usize)> = vec![tile];
        let mut region_tiles = 0;
        let mut fences = 0;

        while let Some((row, col)) = tiles.pop() {
            if visited.contains(&(row, col)) { continue; }
            // Iterate over all adjacent tiles
            let mut tile_fences= 4;

            for delta in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                if let Some(tile) = in_bounds(delta, (row, col), &graph) {
                    if graph[tile.0][tile.1] == graph[row][col] {
                        // Same region since char is the same
                        tile_fences -= 1;
                        tiles.push(tile);
                    } else {
                        if !visited.contains(&tile) {
                            discovered.push(tile)
                        }
                    }
                }
            }
            region_tiles += 1;
            fences += tile_fences;
            visited.insert((row, col));
        }

        debug!("Finished discovering region {} with area {} and fences {}", graph[tile.0][tile.1], region_tiles, fences);
        cost += region_tiles * fences;
    }

    cost
}

fn part_two() -> usize {
    0
}

fn parse() -> Vec<Vec<char>> {
    // The topographic map indicates the height at each position using a scale from 0 (lowest) to 9 (highest)
    // Parse input as matrix
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in include_bytes!("../input.txt").split(|&byte| byte == b'\n') {
        let newline: Vec<char> = Vec::from(line).iter().map(|&byte| byte as char).collect();
        matrix.push(newline);
    }

    debug!("Parsed Matrix {:?}", matrix);
    matrix
}

fn in_bounds(delta: (i32, i32), p: (usize, usize), graph: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let new_point = (p.0 as i32 + delta.0, p.1 as i32 + delta.1);
    if new_point.0 >= 0 && new_point.1 >= 0 && new_point.0  < graph.len() as i32 && new_point.1 < graph[0].len() as i32 {
        Some((new_point.0 as usize, new_point.1 as usize))
    } else {
        None
    }
}