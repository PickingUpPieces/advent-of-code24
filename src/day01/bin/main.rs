// Calculate total distance between both lists
use log::{debug, info};
use advent_of_code::helpers;

fn main() {
    helpers::init();
    info!("Start day 1 challenge...");

    // Read Input 
    let file_path = "src/day01/input.txt";
    let file_content = helpers::read_file(&file_path).unwrap();
    let (mut list1, mut list2) = parse_lists(file_content).unwrap();

    assert_eq!(list1.len(), list2.len());
    debug!("{:?}", list1);
    debug!("{:?}", list2);

    // Sort both lists
    quick_sort(&mut list1);
    quick_sort(&mut list2);

    let mut distance = 0;

    for i in 0..list1.len() {
        distance += list1[i].abs_diff(list2[i]);
    }

    info!("The distance between both lists is {}", distance);
    // TODO: Calculate total distance between entries
    // TODO: Add up distance
}


fn parse_lists(input: Vec::<String>) -> Result<(Vec<u64>, Vec<u64>), &'static str> {
    let mut list1: Vec<u64> = Vec::new();
    let mut list2: Vec<u64> = Vec::new();

    for line in input {
        let splitted: Vec<&str> = line.split("   ").collect();

        if splitted.len() >= 2 {
            list1.push(splitted.get(0).unwrap().parse().expect("String splitting did not work"));
            list2.push(splitted.get(1).unwrap().parse().expect("String splitting did not work"));
        }
    }

    return Ok((list1, list2))
}


fn quick_sort(list: &mut [u64]) {
    debug!("Start quicksort function with list length {}", list.len());
    if list.len() <= 1 { return }

    let pivot = list[0];

    let mut first = 1;
    let mut last = list.len() - 1;

    while first != last {
        if list[first] <= pivot {
            // If smaller, just advance first pointer, since element is already in its correct place
            first += 1;
        } else {
            // If first is larger, swap with last
            if list[last] < pivot {
                list.swap(first, last);
            } else {
                last -= 1;
            }
        }
    }

    // Swap pivot into middle 
    let (head, tail) = if list[first] <= pivot {
        list.swap(0, first);
        list.split_at_mut(first)
    } else {
        list.swap(0, first - 1);
        list.split_at_mut(first - 1)
    };

    // Split into two slices from 0 to first and from first + 1 to list.len()
    quick_sort(head); 
    quick_sort(&mut tail[1..]);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort_reverse_sorted() {
        let mut list = vec![9,8,7,6,5,4,3,2,1];
        quick_sort(&mut list);

        assert_eq!(list, vec![1,2,3,4,5,6,7,8,9])
    }

    #[test]
    fn test_quick_sort_optimal_pivot() {
        let mut list = vec![5,1,6,2,7,3,8,4,9];
        quick_sort(&mut list);

        assert_eq!(list, vec![1,2,3,4,5,6,7,8,9])
    }
}
