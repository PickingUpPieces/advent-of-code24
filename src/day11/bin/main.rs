use std::collections::HashMap;
use advent_of_code::helpers;
use log::info;

// part_two is the more efficient solution, but kept bot

fn main() {
    helpers::init();
    info!("Start day 11 challenge...");

    info!("Solution for PART ONE is {}", part_one());
    info!("Solution for PART TWO is {}", part_two());
}

fn part_one() -> usize {
    let mut stones = parse();
    let mut round = 0;

    for _ in 0..25 {
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

    stones.len()
}

fn part_two() -> usize {
    // Idea: Use a hashset to store the amount of each value instead of all values in a list. Iterate over hashset in every round
    let mut stones = parse_two();
    let mut round = 0;

    for _ in 0..75 {
        info!("Round: {} with length {}", round, stones.len());
        let mut stones_next_round = HashMap::new();
        for (&number, &occ) in stones.iter() {
            // Rules
            if number == 0 {
                *stones_next_round.entry(1).or_insert(0) += occ;
            } else if number.to_string().len() % 2 == 0 {
                let num_str = number.to_string();
                let mid = num_str.len() / 2;
                let left = num_str[..mid].parse::<usize>().unwrap();
                let right = num_str[mid..].parse::<usize>().unwrap();
                *stones_next_round.entry(left).or_insert(0) += occ;
                *stones_next_round.entry(right).or_insert(0) += occ;
            } else {
                *stones_next_round.entry(number * 2024).or_insert(0) += occ;
            }
        }
        stones = stones_next_round;
        round += 1;
    }

    stones.values().sum()
}

fn parse() -> Vec<usize> {
    let mut stones = Vec::new();
    for number in include_str!("../input.txt").split(|b| b == ' ') {
        stones.push(number.parse::<usize>().unwrap());
    }
    stones
}

fn parse_two() -> HashMap<usize, usize> {
    let mut stones = HashMap::new();
    for number in include_str!("../input.txt").split(|b| b == ' ') {
        stones.entry(number.parse::<usize>().unwrap()).and_modify(|occ| *occ += 1).or_insert(1);
    }
    stones
}