use std::{collections::HashMap, fs::read_to_string};

const INPUT_FILE: &str = "input.txt";

#[derive(Clone, Debug)]
struct Record {
    springs: Vec<char>,
    groups: Vec<u8>,
}

impl Record {
    fn from_text(text: &str) -> Vec<Record> {
        let mut springs: Vec<Record> = Vec::new();
        for line in text.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let data: Vec<char> = parts[0].chars().collect();
            let groups: Vec<u8> = parts[1]
                .split(',')
                .map(|x| x.parse::<u8>().unwrap())
                .collect();

            let field = Record {
                springs: data,
                groups,
            };
            springs.push(field);
        }
        springs
    }

    fn unfold(&self, by: usize) -> Record {
        let mut spring_clones: Vec<Vec<char>> = Vec::new();
        let mut groups: Vec<u8> = Vec::new();
        for _ in 0..by {
            spring_clones.push(self.springs.clone());
            groups.extend(self.groups.clone());
        }
        Record {
            springs: spring_clones.join(&'?'),
            groups,
        }
    }

    fn arrangements(&self) -> usize {
        Record::_arrangements(
            self.springs.clone(),
            self.groups.clone(),
            &mut HashMap::new(),
        )
    }

    fn _arrangements(
        springs: Vec<char>,
        groups: Vec<u8>,
        memo: &mut HashMap<(Vec<char>, Vec<u8>), usize>,
    ) -> usize {
        let result = if groups.is_empty() {
            if springs.is_empty() || springs.iter().all(|&x| x != '#') {
                return 1;
            }
            return 0;
        } else if springs.is_empty() {
            return 0;
        } else if let Some(&cached) = memo.get(&(springs.clone(), groups.clone())) {
            return cached;
        } else {
            match springs[0] {
                '.' => Record::_arrangements(springs[1..].to_vec(), groups.clone(), memo),
                '#' => {
                    let needed = groups[0] as usize;

                    if springs.len() < needed || springs[..needed].contains(&'.') {
                        return 0;
                    }
                    if springs.len() == needed {
                        if groups.len() == 1 {
                            return 1;
                        }
                        return 0;
                    }
                    if springs[needed] == '#' {
                        return 0;
                    }
                    Record::_arrangements(
                        springs[needed + 1..].to_vec(),
                        groups[1..].to_vec(),
                        memo,
                    )
                }
                '?' => {
                    let mut with_dmg = springs.clone();
                    with_dmg[0] = '#';
                    let mut with_dot = springs.clone();
                    with_dot[0] = '.';
                    Self::_arrangements(with_dmg, groups.clone(), memo)
                        + Self::_arrangements(with_dot, groups.clone(), memo)
                }
                _ => unreachable!(),
            }
        };
        memo.insert((springs, groups), result);
        result
    }
}

fn part1(records: Vec<Record>) {
    let mut sum = 0;
    for record in records {
        let arrangements = record.arrangements();
        sum += arrangements;
    }
    println!("Total of {} possible arrangements", sum);
}

fn part2(records: Vec<Record>) {
    let mut sum = 0;
    for record in records {
        let record = record.unfold(5);
        let arrangements = record.arrangements();
        sum += arrangements;
    }
    println!("Total of {} possible arrangements after unfolding", sum);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let records = Record::from_text(&input);
    part1(records.clone());
    part2(records);
}
