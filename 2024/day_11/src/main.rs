use std::collections::HashMap;
use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

const MULTIPLIER: u64 = 2024;
const BLINKS_25: u8 = 25;
const BLINKS_75: u8 = 75;

fn blink(number: u64) -> Vec<u64> {
    if number == 0 {
        return vec![1];
    }
    if number.to_string().len() % 2 == 0 {
        let mut tmp = number;
        let mut digits = 0;
        while tmp > 0 {
            tmp /= 10;
            digits += 1;
        }

        let divisor = 10_u64.pow(digits / 2);
        let left = number / divisor;
        let right = number % divisor;
        return vec![left, right];
    }
    vec![number * MULTIPLIER]
}

fn get_resulting_stones(number: u64, blinks: u8, memo: &mut HashMap<(u64, u8), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }
    if let Some(&cached) = memo.get(&(number, blinks)) {
        return cached;
    }

    let mut count = 0;
    for stone in blink(number) {
        count += get_resulting_stones(stone, blinks - 1, memo);
    }

    memo.insert((number, blinks), count);
    count
}

fn part_one(numbers: &Vec<u64>) {
    let mut stones = 0;
    let mut memo: HashMap<(u64, u8), u64> = HashMap::new();
    for number in numbers {
        stones += get_resulting_stones(*number, BLINKS_25, &mut memo);
    }
    println!("Stones after {} blinks: {}", BLINKS_25, stones);
}

fn part_two(numbers: &Vec<u64>) {
    let mut stones = 0;
    let mut memo: HashMap<(u64, u8), u64> = HashMap::new();
    for number in numbers {
        stones += get_resulting_stones(*number, BLINKS_75, &mut memo);
    }
    println!("Stones after {} blinks: {}", BLINKS_75, stones);
}

fn main() {
    let contents = read_to_string(INPUT_FILE).unwrap();
    let numbers: Vec<u64> = contents
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    part_one(&numbers);
    part_two(&numbers);
}
