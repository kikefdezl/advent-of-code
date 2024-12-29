use rayon::prelude::*;
use std::{
    fs::read_to_string,
    sync::atomic::{AtomicU32, Ordering},
};

const INPUT_FILE: &str = "input.txt";

const COUNT: u16 = 2000;

fn mix(value: u64, secret_number: u64) -> u64 {
    value ^ secret_number
}

fn prune(secret_number: u64) -> u64 {
    secret_number % 16777216
}

fn next_secret_number(secret_number: u64) -> u64 {
    let secret_number = prune(mix(secret_number * 64, secret_number));
    let secret_number = prune(mix(secret_number / 32, secret_number));
    prune(mix(secret_number * 2048, secret_number))
}

fn part1(secret_numbers: &Vec<u64>) {
    let mut sum = 0;
    for &secret_number in secret_numbers {
        let mut number = secret_number;
        for _ in 0..COUNT {
            number = next_secret_number(number);
        }
        sum += number;
    }
    println!("Sum: {}", sum);
}

fn get_price_changes(secret_numbers: &Vec<u64>) -> Vec<Vec<(u8, i8)>> {
    let mut sequences = Vec::new();

    for &secret_number in secret_numbers {
        let mut number = secret_number;
        let mut reduced = (secret_number % 10) as u8;

        let mut sequence: Vec<(u8, i8)> = vec![(reduced, 0)];

        for _ in 0..COUNT {
            let new_number = next_secret_number(number);
            let new_reduced = (new_number % 10) as u8;
            sequence.push((new_reduced, new_reduced as i8 - reduced as i8));
            number = new_number;
            reduced = new_reduced;
        }
        sequences.push(sequence);
    }
    sequences
}

fn get_possible_buy_instructions() -> Vec<Vec<i8>> {
    let mut instructions = Vec::new();
    for a in -9..10 {
        for b in -9..10 {
            for c in -9..10 {
                for d in -9..10 {
                    let sum = a + b + c + d;
                    if -9 <= sum && sum <= 9 {
                        instructions.push(vec![a, b, c, d]);
                    }
                }
            }
        }
    }
    instructions
}

fn get_purchased_bananas(instruction: &Vec<i8>, buyer_sequence: &Vec<(u8, i8)>) -> u8 {
    for i in 4..buyer_sequence.len() {
        if instruction[0] == buyer_sequence[i - 3].1
            && instruction[1] == buyer_sequence[i - 2].1
            && instruction[2] == buyer_sequence[i - 1].1
            && instruction[3] == buyer_sequence[i].1
        {
            return buyer_sequence[i].0;
        }
    }
    0
}

// all hail the brute force approach (parallelized with Rayon)
fn part2(secret_numbers: &Vec<u64>) {
    let buyer_sequences: Vec<Vec<(u8, i8)>> = get_price_changes(secret_numbers);
    let instructions = get_possible_buy_instructions();

    let most_bananas = AtomicU32::new(0);
    instructions.par_iter().for_each(|instruction| {
        let bananas: u32 = buyer_sequences
            .par_iter()
            .map(|buyer_sequence| get_purchased_bananas(instruction, buyer_sequence) as u32)
            .sum();

        most_bananas.fetch_max(bananas, Ordering::Relaxed);
    });

    let most_bananas = most_bananas.load(Ordering::Relaxed);

    println!("Most bananas you can get: {}", most_bananas);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let secret_numbers: Vec<u64> = input.lines().map(|x| x.parse().unwrap()).collect();

    part1(&secret_numbers);
    let start = std::time::Instant::now();
    part2(&secret_numbers);
    println!("Elapsed: {:?}", std::time::Instant::now() - start);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        assert_eq!(mix(15, 42), 37);
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }
}
