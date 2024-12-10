use std::collections::VecDeque;
use advent_of_code::helpers;
use log::{debug, info};

#[derive(Debug, Copy, Clone)]
struct File {
    id: usize, 
    starting_position: usize,
    length: usize
}

fn main() {
    helpers::init();
    info!("Start day 9 challenge...");
    let (files, free_space) = parse();
    info!("Got {} files and {} free spaces", files.len(), free_space.len());

    info!("Checksum for PART ONE is {}", part_one(files.clone(), free_space.clone()));
    info!("Checksum for PART TWO is {}", part_two(files, free_space));
}

fn part_one(mut files: Vec<File>, mut free_space: VecDeque<File>) -> usize {
    let mut checksum = 0;
    let mut current_free_space: File = free_space.pop_front().unwrap();

    // Iterate over files from last to first
    for file in files.iter_mut().rev() {
        // Check if  there is no space for defragmentation anymore
        if current_free_space.starting_position > file.starting_position { 
            checksum += calc_checksum(file.starting_position, file.length, file.id);
            continue;
        }

        loop {
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
    checksum
}

fn part_two(mut files: Vec<File>, mut free_space: VecDeque<File>) -> usize {
    let mut checksum = 0;
    // Iterate over files from last to first
    'outer: for file in files.iter_mut().rev() {
        for (index, space) in free_space.iter().enumerate() {
            if space.starting_position > file.starting_position { 
                // File could not be moved due to lack of free space
                checksum += calc_checksum(file.starting_position, file.length, file.id);
                continue 'outer
            } else if space.length >= file.length {
                checksum += calc_checksum(space.starting_position, file.length, file.id);
                free_space[index].starting_position += file.length;
                free_space[index].length -= file.length;
                continue 'outer
            }
        }
    }
    checksum
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