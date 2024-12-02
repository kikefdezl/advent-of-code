use std::fs::read_to_string;
const INPUT_FILE: &str = "input.txt";

fn part_one(reports: &Vec<Vec<i16>>) {
    let mut safe_reports: u16 = 0;
    for levels in reports {
        if is_safe_report(&levels) {
            safe_reports += 1;
        }
    }
    println!("Safe reports: {}", safe_reports)
}

fn part_two(reports: &Vec<Vec<i16>>) {
    let mut safe_reports: u16 = 0;
    for levels in reports {
        for l in 0..levels.len() {
            let mut dampened_levels = levels.clone();
            dampened_levels.remove(l);

            if is_safe_report(&dampened_levels) {
                safe_reports += 1;
                break;
            }
        }
    }
    println!("Safe reports with Problem Dampener: {}", safe_reports)
}

fn is_safe_report(report: &Vec<i16>) -> bool {
    let is_ascending: bool = report[1] - report[0] > 0;
    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];
        let ascends: bool = diff > 0;

        if (is_ascending != ascends) || diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
    }
    true
}

fn main() {
    let contents = read_to_string(INPUT_FILE).unwrap();

    let mut reports: Vec<Vec<i16>> = Vec::new();

    for report in contents.trim().split("\n") {
        let levels: Vec<i16> = report
            .trim()
            .split(" ")
            .filter_map(|s| s.parse::<i16>().ok())
            .collect();
        reports.push(levels);
    }

    part_one(&reports);
    part_two(&reports);
}
