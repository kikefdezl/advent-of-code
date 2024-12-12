use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

const OPEN: char = '(';
const CLOSE: char = ')';

fn part_one(input: &str) {
    let mut floor = 0;
    for char in input.trim().chars() {
        match char {
            OPEN => floor += 1,
            CLOSE => floor -= 1,
            _ => {}
        }
    }
    println!("Resulting floor is {}", floor);
}

fn part_two(input: &str) {
    let mut floor = 0;
    let mut basement = 0;
    for (i, char) in input.trim().chars().enumerate() {
        match char {
            OPEN => floor += 1,
            CLOSE => floor -= 1,
            _ => {}
        }
        if floor == -1 {
            basement = i + 1;
            break;
        }
    }
    println!("Santa first enters the basement at position {}", basement);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    part_one(&input);
    part_two(&input);
}
