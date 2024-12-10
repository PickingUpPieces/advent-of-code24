use std::collections::{HashMap, HashSet};
use advent_of_code::helpers;
use log::{debug, info};
use itertools::Itertools;

// An antinode occurs at any point that is perfectly in line with two antennas of the same frequency - but only when one of the antennas is twice as far away as the other.

fn main() {
    helpers::init();
    info!("Start day 8 challenge...");

    let (row_length, col_length, input) = parse();
    debug!("Parsed input with row length {row_length} and col length {col_length}: {:?}", input);

    info!("Total amount of positions PART ONE: {}", part_one(row_length, col_length, input.clone()));
    info!("Total amount of positions PART TWO: {}", part_two(row_length, col_length, input));
}

fn part_one(row_length: usize, col_length: usize, input: HashMap<char, Vec<(i32, i32)>>) -> usize {
    let mut result: HashSet<(i32, i32)> = HashSet::new();

    for (_, positions) in input.iter() {
        for (&first, &second) in positions.iter().cartesian_product(positions) {
            if first == second { continue; }
            result.insert(antinode(first, second));
            result.insert(antinode(second, first));
        }
    }

    // Filter results to be in bound of the 
    result.retain(|&pos| in_bounds(pos, row_length, col_length));
    result.len()
}

fn part_two(row_length: usize, col_length: usize, input: HashMap<char, Vec<(i32, i32)>>) -> usize {
    let mut result: HashSet<(i32, i32)> = HashSet::new();

    for (_, positions) in input.iter() {
        for (&first, &second) in positions.iter().cartesian_product(positions) {
            if first == second { continue; }
            let mut candidate = first;
            let delta = (second.0 - first.0, second.1 - first.1);

            loop {
                candidate = (candidate.0 + delta.0, candidate.1 + delta.1);
                if !in_bounds(candidate, row_length, col_length) { break; }
                result.insert(candidate);
            }
            candidate = second;
            loop {
                candidate = (candidate.0 - delta.0, candidate.1 - delta.1);
                if !in_bounds(candidate, row_length, col_length) { break; }
                result.insert(candidate);
            }
        }
        // Add positions of antennas as antinodes
        for &pos in positions {
            result.insert(pos);
        }
    }

    // Filter results to be in bound of the 
    result.retain(|&pos| in_bounds(pos, row_length, col_length));
    result.len()
}

fn parse() -> (usize, usize, HashMap<char, Vec<(i32, i32)>>) {
    let mut result: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut row_length: usize = 0;
    let mut col_length: usize = 0;

    for (y, line) in include_bytes!("../input.txt").split(|&b| b == b'\n').enumerate() {
        row_length = line.len();
        col_length += 1;
        for (x, &symbol) in line.iter().enumerate() {
            if symbol != b'.' {
                result.entry(symbol as char).or_default().push((x as i32, y as i32));
            }
        }
    }
    (row_length, col_length, result)
}

fn in_bounds(p: (i32, i32), row_length: usize, col_length: usize) -> bool {
    p.0 >= 0 && p.0 < row_length as i32 && p.1 >= 0 && p.1 < col_length as i32
}

fn antinode(p1: (i32,i32), p2: (i32,i32)) -> (i32, i32) {
    let diff = (p2.0 - p1.0, p2.1 - p1.1);
    (p2.0 + diff.0, p2.1 + diff.1)
}