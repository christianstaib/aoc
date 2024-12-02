use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Read},
};

use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let number_of_safe_reports = get_number_of_safe_reports(file);

    println!("number of safe reports is {}", number_of_safe_reports);
}

fn get_number_of_safe_reports<R: Read>(reader: R) -> u32 {
    // build list of reports
    let reports = BufReader::new(reader)
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let report = line
                .split_whitespace()
                .flat_map(&str::parse::<u32>)
                .collect::<Vec<_>>();

            report
        })
        .collect::<Vec<_>>();

    // count safe reports. Use abs_diff to avoid overflow
    let number_of_safe_reports = reports
        .iter()
        .filter(|&report| {
            // first check if reports without modification are legal
            if check_report(report) {
                return true;
            }

            // brute force the check if the report can be made valid by removing one element
            (0..report.len()).any(|i| {
                let mut report = report.clone();
                report.remove(i);
                check_report(&report)
            })
        })
        .count() as u32;

    number_of_safe_reports
}

fn check_report(report: &[u32]) -> bool {
    let increasing_report = report
        .iter()
        .tuple_windows()
        .all(|(&num0, &num1)| num0 < num1 && num0.abs_diff(num1) <= 3);

    let decreasing_report = report
        .iter()
        .tuple_windows()
        .all(|(&num0, &num1)| num0 > num1 && num0.abs_diff(num1) <= 3);

    increasing_report || decreasing_report
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::get_number_of_safe_reports;

    #[test]
    fn test_get_number_of_safe_reports() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        let result = get_number_of_safe_reports(Cursor::new(input));
        assert_eq!(result, 4);
    }
}
