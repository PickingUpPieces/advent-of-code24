use advent_of_code::helpers;
use log::info;

fn main() {
    helpers::init();
    info!("Start day 11 challenge...");

    info!("Solution for PART ONE is {}", part_one());
    info!("Solution for PART TWO is {}", part_two());
}

fn part_one() -> usize {
    let mut stones = parse();
    let mut round = 0;

    for _ in 0..75 {
        info!("Round: {} with length {}", round, stones.len());
        let mut index = 0;
        while index < stones.len() {
            // Rules
            if stones[index] == 0 {
                stones[index] = 1;
            } else if stones[index].to_string().len() % 2 == 0 {
                let num_str = stones[index].to_string();
                let mid = num_str.len() / 2;
                let left = num_str[..mid].parse::<usize>().unwrap();
                let right = num_str[mid..].parse::<usize>().unwrap();
                stones[index] = left;
                stones.insert(index + 1, right);
                index += 1; // Skip the newly inserted stone
            } else {
                stones[index] *= 2024;
            }
            index += 1;
        }
        round += 1;
    }

    // result after 25 times
    stones.len()
}

fn part_two() -> usize {
    0
}

fn parse() -> Vec<usize> {
    let mut stones = Vec::new();
    for number in include_str!("../input.txt").split(|b| b == ' ') {
        stones.push(number.parse::<usize>().unwrap());
    }
    stones
}