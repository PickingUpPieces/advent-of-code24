use std::collections::HashSet;
use log::{debug, info};
use advent_of_code::helpers;

const DIRECTIONS: [(i32, i32); 4] = [(-1,0), (0,1), (1,0), (0, -1)];

fn main() {
    helpers::init();
    info!("Start day 6 challenge...");

    // Parse input as matrix
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in include_bytes!("../input.txt").split(|&byte| byte == b'\n') {
        let newline: Vec<char> = Vec::from(line).iter().map(|&byte| byte as char ).collect();
        matrix.push(newline);
    }
    debug!("Parsed Matrix {:?}", matrix);

    // Find starting position at byte '^'
    let mut starting_position= (0,0);
    'outer: for (i, row) in matrix.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == '^' {
                starting_position = (i, j);
                break 'outer;
            }
        }
    }
    info!("Found '^' at position: {:?}", starting_position);

    let mut direction_index: usize = 0;
    let mut direction: (i32, i32) = DIRECTIONS[direction_index];
    let mut position = starting_position;
    let mut visited_fields: HashSet<(usize, usize)> = HashSet::new();
    // Add current guard position
    visited_fields.insert(starting_position);

    loop {
        if let Some((x, y)) = get_new_position(&matrix, position, direction) {
            position = match matrix[x][y] {
                '.' => { 
                    visited_fields.insert((x,y)); 
                    (x,y) 
                }, 
                '#' => { 
                    direction_index = (direction_index + 1) % 4; 
                    direction = DIRECTIONS[direction_index]; 
                    position 
                },
                _ => panic!("Unknown char found in matrix")
            };
        } else {
            break;
        }
    }

    info!("The amount of visisted fields are: {}", visited_fields.len());
    info!("Start problem 2...");

    // visited_fields includes the path of the guard
    // Brute force: Set an obstacle into every field and check if the guard makes a loop
    visited_fields.remove(&starting_position);
    let mut amount_loops: HashSet<(usize, usize)> = HashSet::new();
    matrix[starting_position.0][starting_position.1] = '.';

    for field in visited_fields.iter() {
        // Set field to wall char '#'
        matrix[field.0][field.1] = '#';

        // Reset other parameters
        position = starting_position;
        direction_index = 0;
        direction = DIRECTIONS[direction_index]; 

        // seen_fields are (direction, x, y) to uniqly identify which fields already have been visited
        let mut seen_fields: HashSet<(usize, usize, usize)> = HashSet::new();
        seen_fields.insert((direction_index, starting_position.0, starting_position.1)); 

        loop {
            if let Some((x, y)) = get_new_position(&matrix, position, direction) {
                position = match matrix[x][y] {
                    '.' => (x,y),
                    '#' => { 
                        direction_index = (direction_index + 1) % 4; 
                        direction = DIRECTIONS[direction_index]; 
                        position
                    },
                    x => panic!("Unknown char {x} found in matrix")
                };

                // Check if we already visited this field
                if seen_fields.get(&(direction_index, position.0, position.1)).is_some() {
                    debug!("Found loop with obstruction in field {:?}", field);
                    amount_loops.insert(position);
                    break;
                }
                seen_fields.insert((direction_index, position.0, position.1)); 

            } else {
                break; // About to leave the area
            }
        }

        // Unset obstacle field 
        matrix[field.0][field.1] = '.';
    }

    // 536 too low
    info!("The amount of possible loops are: {}", amount_loops.len());
}

fn get_new_position(matrix: &Vec<Vec<char>>, position: (usize, usize), direction: (i32, i32)) -> Option<(usize, usize)> {
    let new_row = (position.0 as i32 + direction.0) as usize;
    let new_col = (position.1 as i32 + direction.1) as usize;

    if new_row < matrix.len() && new_col < matrix[new_row].len() {
        Some((new_row, new_col))
    } else {
        None
    }
}
