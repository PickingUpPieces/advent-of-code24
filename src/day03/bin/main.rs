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

    // Iterate over multiplications
    let result = all_multiply_operations.iter().fold(0, |acc, mul| {
        acc + exec_multiply(mul)
    });

    info!("The result is: {}", result);

    info!("Start day 3b challenge...");

    // Parse input
    let input = include_str!("../input.txt");
    let match_mul = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let all_multiply_operations: Vec<&str> = match_mul.find_iter(&input).map(|occ| occ.as_str()).collect();
    debug!("Overall commands found: {:?}", all_multiply_operations);

    let mut is_enabled: bool = true;
    let mut result = 0;

    for instruction in all_multiply_operations {
        result += match instruction.split('(').next().unwrap() {
            "mul" if is_enabled => exec_multiply(instruction),
            "mul" => 0,
            "do" => { is_enabled = true; 0 },
            "don't" => { is_enabled = false; 0 },
            _ => panic!("Received unkown instruction")
        };
    }

    info!("The result is: {}", result);
}


fn exec_multiply(mul: &str) -> usize {
    let match_numbers = Regex::new(r"(\d+),(\d+)").unwrap(); 

    if let Some(matches) = match_numbers.captures(mul) {
        matches[1].parse::<usize>().unwrap() * matches[2].parse::<usize>().unwrap()
    } else {
        0
    }
}