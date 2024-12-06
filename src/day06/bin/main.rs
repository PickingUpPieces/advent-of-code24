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
    let mut position= (0,0);
    'outer: for (i, row) in matrix.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == '^' {
                position = (i, j);
                break 'outer;
            }
        }
    }
    info!("Found '^' at position: {:?}", position);

    let mut direction_index: usize = 0;
    let mut direction: (i32, i32) = DIRECTIONS[direction_index];
    let mut visited_fields: HashSet<(usize, usize)> = HashSet::new();
    visited_fields.insert(position);

    loop {
        if let Some((x, y)) = get_new_position(&matrix, position, direction) {
            position = match matrix[x][y] {
                '.' => { 
                    visited_fields.insert((x,y)); 
                    (x,y) 
                }, 
                '#' => { 
                    direction_index = ( direction_index + 1) % 4; 
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
