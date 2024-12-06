use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

struct Card {
    winning_numbers: Vec<u8>,
    numbers: Vec<u8>,
}

fn part_one(cards: &Vec<Card>) {
    let mut points = 0;
    for card in cards {
        let mut matches = 0;
        for number in &card.numbers {
            if card.winning_numbers.contains(number) {
                matches += 1;
            }
        }
        if matches == 0 {
            continue;
        }
        points += 2_u32.pow(matches - 1);
    }
    println!("Total points: {}", points);
}

fn part_two(cards: Vec<Card>) {
    let mut scratchcards = vec![1; cards.len()];
    for (i, card) in cards.into_iter().enumerate() {
        let mut matches = 0;
        for number in &card.numbers {
            if card.winning_numbers.contains(number) {
                matches += 1;
            }
        }

        for j in i..i + matches {
            scratchcards[j + 1] += scratchcards[i]
        }
    }
    println!("Total scratchcards: {}", scratchcards.iter().sum::<u32>());
}

fn main() {
    let contents = read_to_string(INPUT_FILE).unwrap();
    let mut cards: Vec<Card> = Vec::new();
    for line in contents.trim().lines() {
        let start = line.find(":").unwrap() + 1;
        let separator = line.find("|").unwrap();

        let winning_numbers: Vec<u8> = line[start..separator]
            .split_whitespace()
            .map(|x| x.parse::<u8>().unwrap())
            .collect();
        let numbers: Vec<u8> = line[separator + 1..]
            .split_whitespace()
            .map(|x| x.parse::<u8>().unwrap())
            .collect();
        cards.push(Card {
            winning_numbers,
            numbers,
        });
    }

    part_one(&cards);
    part_two(cards);
}
