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
            rules.entry(atoi::atoi::<usize>(&line[3..5]).unwrap()).and_modify(|values| values.push(value)).or_insert(vec![value]);
        }
    }

    debug!("Hashmap: {:?}", rules);
    debug!("Updates: {:?}", updates);
    let mut result: usize = 0;

    //  1. Check if value is in not_allowed_pages: Here are the pages included which we should not see in the update implied by the given rules
    //  2. Get current page from hashmap
    //  3. Store all pages we should not see in the rest of the update in the not_allowed_pages set
    //  4. Get middle element if update is safe
    'outer: for update in updates {
        let mut not_allowed_pages: HashSet<usize> = HashSet::new();

        for page in update.clone() {
            if not_allowed_pages.contains(&page) {
                info!("Update is unsafe: {:?}", update);
                continue 'outer;
            }
            if let Some(vec) = rules.get(&page) {
                not_allowed_pages.extend(vec);
            }
        }

        result += update.get(update.len()/2).unwrap_or(&0);
    }
    info!("The result is: {}", result);
}