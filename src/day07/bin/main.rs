use std::collections::VecDeque;
use log::{debug, info};
use advent_of_code::helpers;

// Operators are always evaluated left-to-right, not according to precedence rules. 
// numbers in the equations cannot be rearranged.
// Operators: add (+) and multiply (*)

#[derive(Debug, Clone)]
struct Operation {
    result: usize,
    numbers: VecDeque<usize>
}

fn main() {
    helpers::init();
    info!("Start day 7 challenge...");

    // Problem space: operators^(numbers - 1) 
    // Input: longest equation has 11 numbers => 2^10 can be bruted
    // 8249306: 7 69 19 51 8 7 7 6 534 2
    // Idea: Depth First Search
    
    let operations = parse();
    debug!("Amount of operations: {}", operations.len());

    let mut result = 0;
    let mut amount_true_equations = 0;

    // Iterate over operations
    'outer: for mut operation in operations {
        let mut graph = vec![(operation.numbers.pop_front().unwrap(), operation)];

        while let Some((acc, mut operation)) = graph.pop() {
            if operation.result == acc {
                result += acc;
                amount_true_equations += 1;
                continue 'outer;
            } else if operation.result < acc || operation.numbers.is_empty() { 
                continue;
            }

            let next_number = operation.numbers.pop_front().unwrap();
            graph.push((acc * next_number, operation.clone()));
            graph.push((acc + next_number, operation.clone()));
        }
    }

    info!("Amount of true equation: {amount_true_equations}");
    info!("Sum of true equation results: {result}");
}

fn parse() -> Vec<Operation> {
    let mut operations = Vec::new();
    for line in include_bytes!("../input.txt").split(|&b| b == b'\n') {
        let equation: Vec<&[u8]> = line.split(|&b| b == b':').collect();

        // Get numbers
        let mut numbers: VecDeque<usize> = VecDeque::new();
        for number in equation[1].split(|&byte| byte == b' ') {
            if number.is_empty() { continue; }
            numbers.push_back(atoi::atoi::<usize>(number).unwrap());
        }

        let operation = Operation{
            result: atoi::atoi::<usize>(equation[0]).unwrap(),
            numbers
        };
        operations.push(operation);
    }
    operations
}