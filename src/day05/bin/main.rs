use std::collections::{HashMap, HashSet};

use log::{debug, info};
use advent_of_code::helpers;


fn main() {
    helpers::init();
    info!("Start day 5 challenge...");

    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut updates: Vec<Vec<usize>> = Vec::new();
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

    debug!("Hashmap: {:?}", rules);
    debug!("Updates: {:?}", updates);
    let mut result: usize = 0;
    let mut unsafe_reports: Vec<Vec<usize>> = Vec::new();

    //  1. Check if value is in not_allowed_pages: Here are the pages included which we should not see in the update implied by the given rules
    //  2. Get current page from hashmap
    //  3. Store all pages we should not see in the rest of the update in the not_allowed_pages set
    //  4. Get the middle element, if update is safe
    'outer: for update in updates {
        let mut not_allowed_pages: HashSet<usize> = HashSet::new();

        for page in update.clone() {
            if not_allowed_pages.contains(&page) {
                debug!("Update is unsafe: {:?}", update);
                unsafe_reports.push(update);
                continue 'outer;
            }
            not_allowed_pages.extend(rules.get(&page).cloned().unwrap_or_default());
        }

        result += update.get(update.len().div_ceil(2)).unwrap_or(&0);
    }
    info!("The result is: {}", result);

    // Part 2 looks like topological sort, but just bruting it
    info!("Amount of unsafe updates: {}", unsafe_reports.len());
    result = 0;

    for mut update in unsafe_reports {
        loop {
            if let Some(value) = check_update(&mut update, &rules) {
                result += value;
                break;
            }
        }
    }
    info!("Corrected unsafe reports result: {}", result);
}


fn check_update(update: &mut Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> Option<usize> {
    let mut not_allowed_pages: HashSet<usize> = HashSet::new();

    for (index, page) in update.iter().enumerate() {
        // If hitting unsafe report, swap the two entries in the vector and continue
        if not_allowed_pages.contains(&page) {
            info!("Update is unsafe on page {page}: {:?}", update);
            // search vector for clashing element and swap with current index
            let index_of_clashing_element = update[0..index].iter().position(|&e| rules.get(&e).cloned().unwrap_or_default().contains(&page)).unwrap();
            info!("Swap clashing element {} with {}", update[index_of_clashing_element], update[index]);
            update.swap(index, index_of_clashing_element);
            return None;
        } 
        not_allowed_pages.extend(rules.get(page).cloned().unwrap_or_default());
    }

    update.get(update.len() / 2).copied()
}