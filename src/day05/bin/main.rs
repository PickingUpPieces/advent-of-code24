use std::collections::{HashMap, HashSet};

use log::{debug, info};
use advent_of_code::helpers;


fn main() {
    helpers::init();
    info!("Start day 5 challenge...");

    let (unsafe_reports, result) = part_one();
    info!("Result of PART ONE: {result}");

    info!("Amount of unsafe updates: {}", unsafe_reports.len());
    info!("PART TWO: Corrected unsafe reports result: {}", part_two(unsafe_reports));
}

fn part_one() -> (Vec<Vec<usize>>, usize) {
    let (updates, rules) = parse();
    let mut result: usize = 0;
    let mut unsafe_reports: Vec<Vec<usize>> = Vec::new();

    'outer: for update in updates {
        let mut not_allowed_pages: HashSet<usize> = HashSet::new();

        for page in &update {
            if not_allowed_pages.contains(page) {
                debug!("Update is unsafe: {:?}", update);
                unsafe_reports.push(update.clone());
                continue 'outer;
            }
            not_allowed_pages.extend(rules.get(page).cloned().unwrap_or_default());
        }

        // Correct the calculation of the middle element
        let middle_index = update.len() / 2;
        result += update[middle_index];
    }
    (unsafe_reports, result)
}

fn part_two(unsafe_reports: Vec<Vec<usize>>) -> usize {
    let (_, rules) = parse();
    let mut result: usize = 0;

    // Part 2 looks like topological sort, but just bruting it
    for mut update in unsafe_reports {
        loop {
            if let Some(value) = check_update(&mut update, &rules) {
                result += value;
                break;
            }
        }
    }
    result
}


fn check_update(update: &mut Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> Option<usize> {
    let mut not_allowed_pages: HashSet<usize> = HashSet::new();

    for (index, page) in update.iter().enumerate() {
        // If hitting unsafe report, swap the two entries in the vector and continue
        if not_allowed_pages.contains(&page) {
            debug!("Update is unsafe on page {page}: {:?}", update);
            // search vector for clashing element and swap with current index
            let index_of_clashing_element = update[0..index].iter().position(|&e| rules.get(&e).cloned().unwrap_or_default().contains(&page)).unwrap();
            debug!("Swap clashing element {} with {}", update[index_of_clashing_element], update[index]);
            update.swap(index, index_of_clashing_element);
            return None;
        } 
        not_allowed_pages.extend(rules.get(page).cloned().unwrap_or_default());
    }

    update.get(update.len() / 2).copied()
}

fn parse() -> (Vec<Vec<usize>>, HashMap<usize, Vec<usize>>) {
    let mut updates: Vec<Vec<usize>> = Vec::new();
    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut is_updates = false;

    for line in include_bytes!("../input.txt").split(|&byte| byte == b'\n') {
        if line.is_empty() {
            is_updates = true;
            continue;
        }

        if is_updates {
            updates.push(line.split(|&b| b == b',').map(|level| atoi::atoi::<usize>(level).unwrap()).collect::<Vec<usize>>());
        } else {
            // Idea: Add the rules in reversed order, since for rule X|Y we need to check when we see Y that we do not see page X in the following.
            let value = atoi::atoi::<usize>(&line[0..2]).unwrap();
            rules.entry(atoi::atoi::<usize>(&line[3..5]).unwrap()).or_default().push(value);
        }
    }
    (updates, rules)
}