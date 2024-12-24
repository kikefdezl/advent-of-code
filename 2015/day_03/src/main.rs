use std::{collections::HashSet, fs::read_to_string};

const INPUT_FILE: &str = "input.txt";

fn move_(pos: &(isize, isize), dir: &char) -> (isize, isize) {
    match dir {
        '^' => (pos.0, pos.1 - 1),
        'v' => (pos.0, pos.1 + 1),
        '>' => (pos.0 + 1, pos.1),
        '<' => (pos.0 - 1, pos.1),
        _ => unreachable!(),
    }
}

fn part1(moves: &Vec<char>) {
    let mut pos: (isize, isize) = (0, 0);

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    for dir in moves {
        pos = move_(&pos, &dir);
        visited.insert(pos);
    }

    println!("{} houses have at least one gift.", visited.len());
}

fn part2(moves: &Vec<char>) {
    let mut pos1: (isize, isize) = (0, 0);
    let mut pos2: (isize, isize) = (0, 0);

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    for (i, dir) in moves.iter().enumerate() {
        match i % 2 {
            0 => {
                pos1 = move_(&pos1, &dir);
                visited.insert(pos1);
            }
            1 => {
                pos2 = move_(&pos2, &dir);
                visited.insert(pos2);
            }
            _ => unreachable!(),
        };
    }

    println!("With robosanta, it's {}.", visited.len());
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let moves: Vec<char> = input.trim().chars().collect();
    part1(&moves);
    part2(&moves);
}
