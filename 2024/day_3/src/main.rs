// doing it without the regex crate

use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";
const MAX_SIZE: usize = 12; // mul(123,123)

fn part_one(lines: &Vec<&str>) {
    let mut sum: u32 = 0;
    for line in lines {
        let line_len = line.len();
        let mut start = 0;

        while start < line_len - 4 {
            if &line[start..start + 4] != "mul(" {
                start += 1;
                continue;
            }
            start += 4;

            // find the closing parenthesis
            let slice_end = std::cmp::min(start + MAX_SIZE - 4, line_len);
            let end = match line[start..slice_end].find(')') {
                Some(idx) => start + idx,
                None => continue,
            };

            let contents = &line[start..end];

            // find the comma
            let comma_idx = match contents.find(',') {
                Some(idx) => idx,
                None => continue,
            };

            // parse numbers
            let first_number: u32 = match contents[..comma_idx].parse::<u32>() {
                Ok(num) => num,
                Err(_) => continue,
            };
            let second_number: u32 = match contents[comma_idx + 1..].parse::<u32>() {
                Ok(num) => num,
                Err(_) => continue,
            };

            sum += first_number * second_number;
            start += 5;
        }
    }
    println!("Total sum is {}", sum);
}

fn part_two(lines: &Vec<&str>) {
    let mut sum: u32 = 0;
    let mut is_enabled = true;

    for line in lines {
        let line_len = line.len();
        let mut start = 0;

        while start < line_len - 4 {
            if &line[start..start + 4] == "do()" {
                is_enabled = true;
                start += 4;
                continue;
            }

            if !is_enabled {
                start += 1;
                continue;
            }

            if &line[start..std::cmp::min(start + 7, line_len)] == "don't()" {
                is_enabled = false;
                start += 7;
                continue;
            }

            if &line[start..start + 4] != "mul(" {
                start += 1;
                continue;
            }
            start += 4;

            // find the closing parenthesis
            let slice_end = std::cmp::min(start + MAX_SIZE - 4, line_len);
            let end = match line[start..slice_end].find(')') {
                Some(idx) => start + idx,
                None => continue,
            };

            let contents = &line[start..end];

            // find the comma
            let comma_idx = match contents.find(',') {
                Some(idx) => idx,
                None => continue,
            };

            // parse numbers
            let first_number: u32 = match contents[..comma_idx].parse::<u32>() {
                Ok(num) => num,
                Err(_) => continue,
            };
            let second_number: u32 = match contents[comma_idx + 1..].parse::<u32>() {
                Ok(num) => num,
                Err(_) => continue,
            };

            sum += first_number * second_number;
            start += 5;
        }
    }
    println!("Total enabled sum {}", sum);
}

fn main() {
    let contents = read_to_string(INPUT_FILE).unwrap();

    let lines = contents.trim().split("\n").collect();
    part_one(&lines);
    part_two(&lines);
}
