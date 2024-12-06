// basic selection sort solution

use std::collections::HashSet;
use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

fn parse_input(input: &str) -> (HashSet<(u8, u8)>, Vec<Vec<u8>>) {
    let lines: Vec<&str> = input.trim().split("\n").collect();

    let mut rules: HashSet<(u8, u8)> = HashSet::new();
    let mut i = 0;
    while lines[i].trim() != "" {
        let values: Vec<u8> = lines[i]
            .trim()
            .split("|")
            .map(|x| x.parse::<u8>().unwrap())
            .collect();
        rules.insert((values[0], values[1]));
        i += 1;
    }

    let mut updates: Vec<Vec<u8>> = Vec::new();

    i += 1; // skip the blank line
    while i < lines.len() {
        let values: Vec<u8> = lines[i]
            .split(",")
            .map(|x| x.parse::<u8>().unwrap())
            .collect();
        updates.push(values);
        i += 1;
    }

    (rules, updates)
}

fn is_correct_update(update: &Vec<u8>, rules: &HashSet<(u8, u8)>) -> bool {
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            if rules.contains(&(update[j], update[i])) {
                return false;
            }
        }
    }
    true
}

fn part_one(rules: &HashSet<(u8, u8)>, updates: &Vec<Vec<u8>>) {
    let mut sum: u32 = 0;
    for update in updates {
        if is_correct_update(&update, &rules) {
            let middle = update.len() / 2;
            sum += update[middle] as u32;
        }
    }
    println!("Total sum is {}", sum);
}

fn part_two(rules: &HashSet<(u8, u8)>, updates: &Vec<Vec<u8>>) {
    let mut sum: u32 = 0;
    for update in updates {
        if is_correct_update(&update, &rules) {
            continue;
        }

        let mut fixed = update.clone();
        for i in 0..fixed.len() {
            for j in i + 1..fixed.len() {
                if rules.contains(&(fixed[j], fixed[i])) {
                    let tmp = fixed[j];
                    fixed[j] = fixed[i];
                    fixed[i] = tmp;
                }
            }
        }
        assert!(is_correct_update(&fixed, rules));
        let middle = fixed.len() / 2;
        sum += fixed[middle] as u32;
    }
    println!("Total sum of fixed updates is {}", sum);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();

    let (rules, updates) = parse_input(&input);

    part_one(&rules, &updates);
    part_two(&rules, &updates);
}
