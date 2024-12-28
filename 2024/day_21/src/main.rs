use std::collections::VecDeque;
use std::{collections::HashMap, fs::read_to_string};

const INPUT_FILE: &str = "input.txt";

const UP: char = '^';
const DOWN: char = 'v';
const RIGHT: char = '>';
const LEFT: char = '<';
const ACCEPT: char = 'A';

type Action = char;

struct Pad {
    graph: HashMap<char, HashMap<Action, char>>,
}

impl Pad {
    fn new_numpad() -> Pad {
        let mut graph: HashMap<char, HashMap<Action, char>> = HashMap::new();

        graph.insert('A', HashMap::from([(UP, '3'), (LEFT, '0')]));
        graph.insert('0', HashMap::from([(UP, '2'), (RIGHT, 'A')]));
        graph.insert('1', HashMap::from([(UP, '4'), (RIGHT, '2')]));
        graph.insert(
            '2',
            HashMap::from([(UP, '5'), (RIGHT, '3'), (LEFT, '1'), (DOWN, '0')]),
        );
        graph.insert('3', HashMap::from([(UP, '6'), (LEFT, '2'), (DOWN, 'A')]));
        graph.insert('4', HashMap::from([(UP, '7'), (RIGHT, '5'), (DOWN, '1')]));
        graph.insert(
            '5',
            HashMap::from([(UP, '8'), (RIGHT, '6'), (LEFT, '4'), (DOWN, '2')]),
        );
        graph.insert('6', HashMap::from([(UP, '9'), (LEFT, '5'), (DOWN, '3')]));
        graph.insert('7', HashMap::from([(RIGHT, '8'), (DOWN, '4')]));
        graph.insert('8', HashMap::from([(RIGHT, '9'), (LEFT, '7'), (DOWN, '5')]));
        graph.insert('9', HashMap::from([(LEFT, '8'), (DOWN, '6')]));
        Pad { graph }
    }

    fn new_arrowpad() -> Pad {
        let mut graph: HashMap<char, HashMap<Action, char>> = HashMap::new();

        graph.insert('A', HashMap::from([(DOWN, '>'), (LEFT, '^')]));
        graph.insert('>', HashMap::from([(LEFT, 'v'), (UP, 'A')]));
        graph.insert('^', HashMap::from([(DOWN, 'v'), (RIGHT, 'A')]));
        graph.insert('<', HashMap::from([(RIGHT, 'v')]));
        graph.insert('v', HashMap::from([(UP, '^'), (RIGHT, '>'), (LEFT, '<')]));
        Pad { graph }
    }

    fn find_sequences(&self, from: char, to: char) -> Vec<Vec<Action>> {
        if from == to {
            return vec![vec![ACCEPT]];
        }

        let mut queue = VecDeque::new();
        let mut sequences: Vec<Vec<Action>> = Vec::new();
        let mut visited: HashMap<char, usize> = HashMap::from([(from, 0)]);

        queue.push_back(vec![(' ', from)]);

        while let Some(path) = queue.pop_front() {
            let current_node = path.last().unwrap().1;
            let current_len = path.len();

            for (action, neighbor_node) in self.graph[&current_node].iter() {
                if visited.contains_key(&neighbor_node) && visited[&neighbor_node] < current_len + 1
                {
                    continue;
                }
                visited.insert(*neighbor_node, current_len + 1);

                let mut new_path = path.clone();
                new_path.push((action.clone(), *neighbor_node));

                if *neighbor_node == to {
                    let mut sequence: Vec<Action> =
                        new_path.iter().skip(1).map(|(x, _)| x.clone()).collect();
                    sequence.push(ACCEPT);
                    sequences.push(sequence);
                } else {
                    queue.push_back(new_path);
                }
            }
        }
        sequences
    }
}

fn get_numpad_sequences(numpad: &Pad, code: &Vec<char>) -> Vec<Vec<Action>> {
    let mut from = ACCEPT;
    let mut full_sequences = vec![Vec::new()];

    for target in code.iter() {
        let sequences = numpad.find_sequences(from, *target);

        let mut new_full_sequences = Vec::new();
        for full_sequence in full_sequences {
            for sequence in &sequences {
                let mut new_full_sequence = full_sequence.clone();
                new_full_sequence.extend(sequence);
                new_full_sequences.push(new_full_sequence);
            }
        }

        full_sequences = new_full_sequences;
        from = *target;
    }
    full_sequences
}

fn get_sequence_length(
    arrowpad: &Pad,
    sequence: &Vec<Action>,
    steps: u8,
    memo: &mut HashMap<(Vec<Action>, u8), usize>,
) -> usize {
    if let Some(&s) = memo.get(&(sequence.clone(), steps)) {
        return s;
    }

    if steps == 0 {
        return sequence.len();
    }

    let mut start = ACCEPT;
    let mut length = 0;
    for &end in sequence {
        let subsequences = arrowpad.find_sequences(start, end);

        let mut shortest: usize = usize::MAX;
        for subsequence in subsequences {
            let next_len = get_sequence_length(&arrowpad, &subsequence, steps - 1, memo);
            if next_len < shortest {
                shortest = next_len;
            }
        }
        length += shortest;
        start = end;
    }
    memo.insert((sequence.clone(), steps), length);
    length
}

fn run(codes: &[&str], robots: u8) {
    let numpad = Pad::new_numpad();
    let arrowpad = Pad::new_arrowpad();

    let mut memo = HashMap::new();
    let mut sum = 0;
    for code in codes {
        let numbers: Vec<char> = code.chars().collect();
        let sequences = get_numpad_sequences(&numpad, &numbers);

        let mut shortest = usize::MAX;
        for sequence in sequences {
            let len = get_sequence_length(&arrowpad, &sequence, robots, &mut memo);
            if len < shortest {
                shortest = len;
            }
        }

        let code_number: usize = code.trim()[..code.len() - 1].parse().unwrap();
        sum += code_number * shortest;
    }
    println!("Sum of complexities for {} robots: {}", robots, sum);
}

fn part1(codes: &[&str]) {
    run(&codes, 2);
}

fn part2(codes: &[&str]) {
    run(&codes, 25);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();

    let codes: Vec<&str> = input.lines().collect();
    part1(&codes);
    part2(&codes);
}
