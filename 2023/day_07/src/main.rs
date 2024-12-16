use std::{collections::HashMap, fs::read_to_string, iter::zip};

const INPUT_FILE: &str = "input.txt";

#[derive(Clone, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Clone, Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u32,
    type_: HandType,
    jokers: bool,
}

impl Hand {
    fn from_str(str: &str) -> Hand {
        let parts: Vec<&str> = str.trim().split_whitespace().collect();
        let cards = parts[0].trim().chars().collect();
        let bid = parts[1].parse().unwrap();

        let mut map: HashMap<char, u8> = HashMap::new();
        for &card in &cards {
            let mut count = match map.get(&card) {
                Some(n) => *n,
                None => 0,
            };
            count += 1;
            map.insert(card, count);
        }
        let mut counts: Vec<u8> = map.values().cloned().collect();
        counts.sort_by(|a, b| b.cmp(a));

        let type_ = match counts[0] {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => match counts[1] {
                2 => HandType::FullHouse,
                _ => HandType::ThreeOfAKind,
            },
            2 => match counts[1] {
                2 => HandType::TwoPair,
                _ => HandType::OnePair,
            },
            _ => HandType::HighCard,
        };

        Hand {
            cards,
            bid,
            type_,
            jokers: false,
        }
    }

    fn use_jokers(&mut self) {
        let joker_count = self.cards.iter().filter(|&x| *x == 'J').count();
        if joker_count == 0 || self.jokers {
            return;
        }
        match self.type_ {
            HandType::FourOfAKind => self.type_ = HandType::FiveOfAKind,
            HandType::FullHouse => self.type_ = HandType::FiveOfAKind,
            HandType::ThreeOfAKind => self.type_ = HandType::FourOfAKind,
            HandType::TwoPair => {
                match joker_count {
                    1 => self.type_ = HandType::FullHouse,
                    2 => self.type_ = HandType::FourOfAKind,
                    _ => unreachable!(),
                };
            }
            HandType::OnePair => self.type_ = HandType::ThreeOfAKind,
            HandType::HighCard => self.type_ = HandType::OnePair,
            _ => {}
        };
        self.jokers = true;
    }

    fn get_rank(&self) -> u8 {
        match self.type_ {
            HandType::FiveOfAKind => 6,
            HandType::FourOfAKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfAKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0,
        }
    }

    fn is_larger_than(&self, other: &Hand) -> bool {
        if self.get_rank() > other.get_rank() {
            return true;
        } else if other.get_rank() > self.get_rank() {
            return false;
        }
        for (a, b) in zip(&self.cards, &other.cards) {
            let rank_a = get_card_rank(*a, self.jokers);
            let rank_b = get_card_rank(*b, self.jokers);
            if rank_a > rank_b {
                return true;
            } else if rank_b > rank_a {
                return false;
            }
        }
        false
    }
}

fn get_card_rank(card: char, jokers: bool) -> u8 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => match jokers {
            true => 1,
            false => 11,
        },
        'T' => 10,
        _ => card.to_digit(10).unwrap() as u8,
    }
}

fn sort_hands(hands: &mut Vec<Hand>) {
    for i in 0..hands.len() {
        for j in (i + 1)..hands.len() {
            if hands[i].is_larger_than(&hands[j]) {
                let tmp = hands[i].clone();
                hands[i] = hands[j].clone();
                hands[j] = tmp;
            }
        }
    }
}

fn solve(hands: &mut Vec<Hand>) -> u32 {
    sort_hands(hands);

    let mut winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        winnings += hand.bid * (i as u32 + 1);
    }
    winnings
}

fn part1(hands: &mut Vec<Hand>) {
    let winnings = solve(hands);
    println!("Total winnings: {}", winnings);
}

fn part2(hands: &mut Vec<Hand>) {
    for hand in &mut *hands {
        hand.use_jokers();
    }
    let winnings = solve(hands);
    println!("Total winnings with jokers: {}", winnings);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let mut hands: Vec<Hand> = Vec::new();
    for line in input.trim().lines() {
        hands.push(Hand::from_str(line));
    }

    part1(&mut hands);
    part2(&mut hands);
}
