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

    let mut result: HashSet<(i32, i32)> = HashSet::new();

    for (_, positions) in input.iter() {
        for (&first, &second) in positions.iter().cartesian_product(positions) {
            if first == second { continue; }
            result.insert(antinode(first, second));
            result.insert(antinode(second, first));
        }
    }

    // Filter results to be in bound of the 
    let result: Vec<&(i32, i32)> = result.iter().filter(|(x, y)| { x >= &0 && x < &(row_length as i32) && y >= &0 && y < &(col_length as i32) }).collect();

    info!("Total amount of new positions: {}", result.len());
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

fn antinode(p1: (i32,i32), p2: (i32,i32)) -> (i32, i32) {
    let diff = (p2.0 - p1.0, p2.1 - p1.1);
    (p2.0 + diff.0, p2.1 + diff.1)
}