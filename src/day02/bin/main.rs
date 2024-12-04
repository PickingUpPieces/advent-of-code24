use log::{debug, info, warn};
use advent_of_code::helpers;

// monotonicity invariants
// - decreasing/increasing by max 3 at a time
// - No equal numbers

fn main() {
    helpers::init();
    info!("Start day 2 challenge...");

    let mut reports = Vec::with_capacity(2000);

    // Parse the input
    for line in include_bytes!("../input.txt").split(|&byte| byte == b'\n') {
        let mut report: Vec<usize> = Vec::new();
        for level in line.split(|&byte| byte == b' ') {
            report.push(atoi::atoi::<usize>(level).unwrap());
        }
        reports.push(report);
    }

    info!("Total amount of reports: {}", reports.len());

    // Check monotonicity
    let mut amount_safe_reports = 0;

    for report in reports.clone() {
        amount_safe_reports += check_sequence_validity(report, false);
    }

    info!("Amount of safe reports: {amount_safe_reports}");


    info!("Calculate problem 2...");
    amount_safe_reports = 0;

    for report in reports {
        amount_safe_reports += check_sequence_validity(report, true);
    }

    info!("Amount of safe reports: {amount_safe_reports}");
}


fn check_sequence_validity(report: Vec<usize>, tolerate_one_bad_window: bool) -> usize {
    let mut increasing: Option<bool> = None;


    for (index, window) in report.windows(2).enumerate() {
        let diff = window[1] as i64 - window[0] as i64;

        if diff == 0 || diff.abs() > 3 {
            if tolerate_one_bad_window {
                warn!("Tolerate bad number {:?} with index {index} in report {:?}", window, report);
                return brute_all_windows(report);
            } else {
                debug!("Unsafe sequence: {:?}", report); 
                return 0;
            }
        }

        match increasing {
            None => increasing = Some(diff > 0),
            Some(is_increasing) => 
                if is_increasing && diff < 0 || !is_increasing && diff > 0 { 
                    if tolerate_one_bad_window {
                        warn!("Tolerate bad number {:?} with index {index} in report {:?}", window, report);
                        return brute_all_windows(report);
                    } else {
                        debug!("Unsafe sequence: {:?}", report); 
                        return 0;
                    }
            },
        };
    }
    return 1;
}

fn brute_all_windows(report: Vec<usize>) -> usize {
    // Remove all elements once and try if there is a safe version of the report
    for (index, _) in report.iter().enumerate() {
        let mut new_report = report.clone();
        new_report.remove(index);
        if check_sequence_validity(new_report, false) == 1 { return 1 }
    }
    return 0
}