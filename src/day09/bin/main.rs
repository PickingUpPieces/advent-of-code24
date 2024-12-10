use std::collections::VecDeque;
use advent_of_code::helpers;
use log::{debug, info};

#[derive(Debug)]
struct File {
    id: usize, 
    starting_position: usize,
    length: usize
}

fn main() {
    helpers::init();
    info!("Start day 9 challenge...");
    let mut checksum = 0;
    let (mut files, mut free_space) = parse();
    let mut current_free_space: File = free_space.pop_front().unwrap();
    info!("Got {} files and {} free spaces", files.len(), free_space.len());

    // Iterate over files from last to first
    for file in files.iter_mut().rev() {
        // Check if  there is no space for defragmentation anymore
        if current_free_space.starting_position > file.starting_position { 
            checksum += calc_checksum(file.starting_position, file.length, file.id);
            continue;
        }

        loop {
            //debug!("File {} with starting position {} and length {}", file.id, file.starting_position, file.length);
            if current_free_space.length <= file.length {
                debug!("Moving {} units of file {} to free space starting at {}", current_free_space.length, file.id, current_free_space.starting_position);
                file.length -= current_free_space.length;
                // Calculating checksum is factiorial the added length 
                checksum += calc_checksum(current_free_space.starting_position, current_free_space.length, file.id);
                current_free_space = free_space.pop_front().unwrap();
                if file.length == 0 { break; }
            } else if current_free_space.length > file.length {
                debug!("Moving {} units of file {} to free space starting at {}", file.length, file.id, current_free_space.starting_position);
                current_free_space.length -= file.length;
                checksum += calc_checksum(current_free_space.starting_position, file.length, file.id);
                current_free_space.starting_position += file.length;
                break;
            }
        }
    }
    info!("Checksum is {checksum}");
}

fn parse() -> (Vec<File>, VecDeque<File>) {
    let mut files: Vec<File> = Vec::new();
    let mut free_space: VecDeque<File> = VecDeque::new();
    let mut current_position = 0;
    let mut id = 0;

    for (index, element) in include_str!("../input.txt").chars().enumerate() {
        let length = element.to_digit(10).unwrap() as usize;
        if length == 0 { continue }

        if index % 2 == 0 {
            files.push(File{id, starting_position: current_position, length});
            id += 1;
        } else {
            free_space.push_back(File{id: 0, starting_position: current_position, length});
        }
        current_position += length; 
    }
    (files, free_space)
}

fn calc_checksum(start: usize, len: usize, file_id: usize) -> usize {
    (start..start + len)
        .fold(0, |acc, x| acc + (x * file_id))
}