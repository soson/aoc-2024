extern crate aocf;

use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2024))
        .day(Some(2))
        .cookie_file("./examples/cookie")
        .init()
        .unwrap();

    let input = if let Ok(i) = aoc.get_input(false) {
        i
    } else {
        "you probably need to add a valid cookie".to_string()
    };

    let reports = parse_data(&input);
    let safe_reports = get_safe_reports(&reports).len();
    let single_bad_level_reports = get_single_bad_level_reports(&reports).len();
    println!("day 2 - part 1: {:?}", safe_reports);
    println!(
        "day 2 - part 2: {:?}",
        safe_reports + single_bad_level_reports
    );
}

fn parse_data(input: &str) -> Vec<Vec<u32>> {
    let data: Vec<Vec<_>> = input
        .lines()
        .map(|l| {
            return l
                .split_whitespace()
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
        })
        .collect();

    data
}

fn get_safe_reports(input: &Vec<Vec<u32>>) -> Vec<&Vec<u32>> {
    let safe_reports = input
        .iter()
        .filter(|&report| is_report_ok(&report))
        .collect::<Vec<_>>();

    safe_reports
}

fn is_report_ok(report: &Vec<u32>) -> bool {
    let pairs: Vec<(u32, u32)> = report
        .clone()
        .into_iter()
        .zip(report.clone().into_iter().skip(1))
        .collect();

    let (left, right) = pairs.first().unwrap();
    let is_ascending = left < right;

    pairs.iter().all(|(a, b)| {
        if is_ascending != (a < b) {
            return false;
        }
        match a.abs_diff(*b) {
            1..=3 => return true,
            _ => return false,
        }
    })
}

fn get_single_bad_level_reports(input: &Vec<Vec<u32>>) -> Vec<&Vec<u32>> {
    let safe_reports = input
        .iter()
        .filter(|&report| !is_report_ok(&report))
        .filter(|&report| {
            for i in 0..report.len() {
                let mut subreport = report.clone();
                subreport.remove(i);
                if is_report_ok(&subreport) {
                    return true;
                }
            }
            false
        })
        .collect::<Vec<_>>();

    safe_reports
}

#[cfg(test)]
mod tests {
    use crate::{get_safe_reports, get_single_bad_level_reports};

    #[test]
    fn test_safe_reports() {
        let reports = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        let safe_reports = get_safe_reports(&reports);
        let single_bad_level_reports = get_single_bad_level_reports(&reports);
        assert_eq!(safe_reports.len(), 2);
        assert_eq!(single_bad_level_reports.len(), 2);
    }
}
