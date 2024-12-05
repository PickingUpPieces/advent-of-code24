use log::{debug, info};
use advent_of_code::helpers;
use regex::Regex;

//  mul(X,Y), where X and Y are each 1-3 digit numbers

fn main() {
    helpers::init();
    info!("Start day 3 challenge...");

    // Parse input
    let input = include_str!("../input.txt");
    let match_mul = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let all_multiply_operations: Vec<&str> = match_mul.find_iter(&input).map(|occ| occ.as_str()).collect();

    debug!("Multiplication found: {:?}", all_multiply_operations);

    let match_numbers = Regex::new(r"(\d+),(\d+)").unwrap(); 

    let result = all_multiply_operations.iter().fold(0, |acc, mul| {
        acc + if let Some(matches) = match_numbers.captures(mul) {
            matches[1].parse::<usize>().unwrap() * matches[2].parse::<usize>().unwrap()
        } else {
            0
        }
    });

    info!("The result is: {}", result);
}