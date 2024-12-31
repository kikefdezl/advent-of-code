use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

type Key = Vec<u8>;
type Lock = Vec<u8>;

fn keylock_from_text(text: &str) -> Vec<u8> {
    let mut heights = vec![0; 5];
    for line in text.trim().lines() {
        for (i, pin) in line.chars().enumerate() {
            if pin == '#' {
                heights[i] += 1;
            }
        }
    }
    heights
}

fn key_fits_lock(key: &Key, lock: &Lock) -> bool {
    for (k, l) in key.iter().zip(lock.iter()) {
        if k + l > 7 {
            return false;
        }
    }
    true
}

fn parse_input(input: &str) -> (Vec<Key>, Vec<Lock>) {
    let mut keys: Vec<Key> = Vec::new();
    let mut locks: Vec<Lock> = Vec::new();

    let lines: Vec<&str> = input.lines().collect();

    for lines in lines.chunks(8) {
        let text = lines.join("\n");
        match text.trim().chars().nth(0).unwrap() {
            '.' => keys.push(keylock_from_text(&text)),
            '#' => locks.push(keylock_from_text(&text)),
            _ => panic!(),
        }
    }
    (keys, locks)
}

fn part1(keys: &Vec<Key>, locks: &Vec<Lock>) {
    let mut sum = 0;
    for key in keys {
        for lock in locks {
            if key_fits_lock(&key, &lock) {
                sum += 1;
            }
        }
    }
    println!("Sum of fitting keys: {}", sum);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let (keys, locks) = parse_input(&input);
    part1(&keys, &locks);
}
