use log::{debug, info};
use advent_of_code::helpers;

// Words allowed to be horizontal, vertical, diagonal, written backwards, or even overlapping other words. 
// Find all occurences of the word "XMAS"

fn main() {
    helpers::init();
    info!("Start day 4 challenge...");

    // Parse input as matrix
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in include_bytes!("../input.txt").split(|&byte| byte == b'\n') {
        let newline: Vec<char> = Vec::from(line).iter().map(|&byte| byte as char ).collect();
        matrix.push(newline);
    }
    debug!("Parsed Matrix {:?}", matrix);

    info!("Absolut amount of XMAS is: {}", match_xmas(matrix.clone()));
    info!("Absolut amount of MAS-crosses is: {}", match_mas(matrix));
}

fn match_xmas(matrix: Vec<Vec<char>>) -> usize {
    let mut xmas_counter = 0;
    let xmas = ['X', 'M', 'A', 'S'];

    // Remark: Alternative approach would be to just make a substring search through all rows, columns (transposed rows), and diagonals

    // Assumption: Every XMAS starts with an X. Finding every X and check all directions
    // Remark: If we store the coordinates of every X in the parse run, we could reduce the runtime
    for (line_index, line) in matrix.iter().enumerate() {
        for (element_index, element) in line.iter().enumerate() {
            if *element == 'X' {
                // Forward
                if element_index + 3 < line.len() {
                    if &line[element_index..element_index + 4] == xmas {
                        xmas_counter += 1;
                    }
                }
                // Backward
                if element_index >= 3 {
                    if &line[element_index - 3..=element_index] == ['S', 'A', 'M', 'X'] {
                        xmas_counter += 1;
                    }
                }
                if line_index + 3 < matrix.len() {
                    let downward: Vec<char> = (0..4).map(|i| matrix[line_index + i][element_index]).collect();
                    if downward == xmas {
                        xmas_counter += 1;
                    }
                }
                if line_index >= 3 {
                    let upward: Vec<char> = (0..4).map(|i| matrix[line_index - i][element_index]).collect();
                    if upward == xmas {
                        xmas_counter += 1;
                    }
                }
                if line_index + 3 < matrix.len() && element_index + 3 < line.len() {
                    let cross_right_down: Vec<char> = (0..4).map(|i| matrix[line_index + i][element_index + i]).collect();
                    if cross_right_down == xmas {
                        xmas_counter += 1;
                    }
                }
                if line_index >= 3 && element_index + 3 < line.len() {
                    let cross_right_up: Vec<char> = (0..4).map(|i| matrix[line_index - i][element_index + i]).collect();
                    if cross_right_up == xmas {
                        xmas_counter += 1;
                    }
                }
                if line_index + 3 < matrix.len() && element_index >= 3 {
                    let cross_left_down: Vec<char> = (0..4).map(|i| matrix[line_index + i][element_index - i]).collect();
                    if cross_left_down == xmas {
                        xmas_counter += 1;
                    }
                }
                if line_index >= 3 && element_index >= 3 {
                    let cross_left_up: Vec<char> = (0..4).map(|i| matrix[line_index - i][element_index - i]).collect();
                    if cross_left_up == xmas {
                        xmas_counter += 1;
                    }
                }
            }
        }
    } 

    xmas_counter
}

fn match_mas(matrix: Vec<Vec<char>>) -> usize {
    let mut counter: usize = 0;

    for (line_index, line) in matrix.iter().enumerate() {
        for (element_index, element) in line.iter().enumerate() {
            if *element == 'A' {
                // Search for MAS or SAM in a cross with A in the middle, so cross_left_up combined with cross_left_down and cross_right_up with cross_right_down
                if line_index >= 1 && element_index >= 1 && line_index + 1 < matrix.len() && element_index + 1 < line.len() {
                    let cross_left_up: Vec<char> = (0..2).map(|i| matrix[line_index - i][element_index - i]).collect();
                    let cross_left_down: Vec<char> = (0..2).map(|i| matrix[line_index + i][element_index - i]).collect();
                    let cross_right_up: Vec<char> = (0..2).map(|i| matrix[line_index - i][element_index + i]).collect();
                    let cross_right_down: Vec<char> = (0..2).map(|i| matrix[line_index + i][element_index + i]).collect();
                    debug!("cross left up {:?}:{:?}", cross_left_up, cross_right_down);
                    debug!("cross right up {:?}:{:?}", cross_right_up, cross_left_down);

                    if ( cross_left_up == ['A', 'M'] && cross_right_down == ['A', 'S'] || cross_left_up == ['A', 'S'] && cross_right_down == ['A', 'M'] ) && ( cross_right_up == ['A', 'S'] && cross_left_down == ['A', 'M'] || cross_right_up == ['A', 'M'] && cross_left_down == ['A', 'S'] ) {
                        counter += 1;
                    }
                }
            }
        }
    }
    counter
}
