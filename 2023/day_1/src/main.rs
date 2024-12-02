use std::collections::HashMap;
use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

fn part_one(lines: &Vec<&str>) {
    let mut sum = 0;
    for line in lines {
        let mut l: u32 = 0;
        let mut r: u32 = 0;
        for c in line.chars() {
            if c.is_digit(10) {
                l = c.to_digit(10).unwrap();
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_digit(10) {
                r = c.to_digit(10).unwrap();
                break;
            }
        }

        sum += r + (l * 10);
    }
    println!("Total sum: {}", sum);
}

fn part_two(lines: &Vec<&str>) {
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut digit_map: HashMap<&str, usize> = HashMap::new();
    let mut len_map: HashMap<&str, usize> = HashMap::new();
    for (i, number) in numbers.iter().enumerate() {
        digit_map.insert(number, i + 1);
        let len = number.chars().count();
        len_map.insert(number, len);
    }

    let mut sum = 0;
    for line in lines {
        let mut l: u32 = 0;
        let mut r: u32 = 0;

        let line_len: usize = line.len();

        'left_search: for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                l = c.to_digit(10).unwrap();
                break 'left_search;
            }

            for number in &numbers {
                let number_len = len_map.get(number).unwrap();
                if number_len > &line_len || i + number_len >= line_len {
                    continue;
                }

                if line[i..i + *number_len] == **number {
                    l = *digit_map.get(number).unwrap() as u32;
                    break 'left_search;
                }
            }
        }

        'right_search: for (i, c) in line.chars().rev().enumerate() {
            let idx = line_len - i - 1;
            if c.is_digit(10) {
                r = c.to_digit(10).unwrap();
                break 'right_search;
            }

            for number in &numbers {
                let number_len = len_map.get(number).unwrap();
                if number_len > &line_len || idx < *number_len {
                    continue;
                }

                if line[idx - number_len + 1..idx + 1] == **number {
                    r = *digit_map.get(number).unwrap() as u32;
                    break 'right_search;
                }
            }
        }
        sum += r + (l * 10);
    }
    println!("Fixed total sum: {}", sum);
}

fn main() {
    let content = read_to_string(INPUT_FILE).unwrap();

    let lines: Vec<&str> = content.trim().split("\n").collect();

    part_one(&lines);
    part_two(&lines);
}
