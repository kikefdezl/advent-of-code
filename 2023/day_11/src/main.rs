use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

#[derive(Clone, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Map {
    data: Vec<Vec<char>>,
    empty_rows: HashSet<usize>,
    empty_cols: HashSet<usize>,
}

impl Map {
    fn from_text(text: &str) -> Map {
        let mut data: Vec<Vec<char>> = Vec::new();
        let mut empty_rows: HashSet<usize> = HashSet::new();
        let mut empty_cols: HashSet<usize> = HashSet::new();
        for (y, line) in text.lines().enumerate() {
            let chars: Vec<char> = line.chars().collect();
            if chars.iter().all(|c| *c == '.') {
                empty_rows.insert(y);
            }
            data.push(chars);
        }

        for x in 0..data[0].len() {
            let col: Vec<char> = data.iter().map(|r| r[x]).collect();
            if col.iter().all(|c| *c == '.') {
                empty_cols.insert(x);
            }
        }
        Map {
            data,
            empty_rows,
            empty_cols,
        }
    }

    fn print(&self) {
        for row in &self.data {
            for ch in row {
                print!("{ch}");
            }
            println!();
        }
    }

    fn get_galaxy_coords(&self) -> Vec<Coord> {
        let mut coords: Vec<Coord> = Vec::new();
        for (y, row) in self.data.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if *val == '#' {
                    coords.push(Coord { x, y });
                }
            }
        }
        coords
    }

    fn get_galaxy_pairs(&self) -> Vec<(Coord, Coord)> {
        let coords = self.get_galaxy_coords();

        let n_galaxies = coords.len();
        let mut pairs: Vec<(Coord, Coord)> = Vec::new();
        for i in 0..(n_galaxies - 1) {
            for j in (i + 1)..n_galaxies {
                pairs.push((coords[i].clone(), coords[j].clone()))
            }
        }
        pairs
    }
}

fn solve(map: Map, expansion_factor: usize) {
    let pairs = map.get_galaxy_pairs();
    let mut total_distance = 0;
    for pair in pairs {
        let x_min = min(pair.0.x, pair.1.x);
        let x_max = max(pair.0.x, pair.1.x);
        let y_min = min(pair.0.y, pair.1.y);
        let y_max = max(pair.0.y, pair.1.y);

        let mut jumps = 0;
        for row in map.empty_rows.iter() {
            if y_min < *row && *row < y_max {
                jumps += 1;
            }
        }
        for col in map.empty_cols.iter() {
            if x_min < *col && *col < x_max {
                jumps += 1;
            }
        }
        total_distance += pair.0.x.abs_diff(pair.1.x);
        total_distance += pair.0.y.abs_diff(pair.1.y);
        total_distance += jumps * (expansion_factor - 1);
    }
    println!("Expansion Factor {}", expansion_factor);
    println!("Total sum of shortest distances: {}", total_distance);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let map = Map::from_text(&input);
    println!("Initial map:");
    println!("Empty rows: {:?}", map.empty_rows);
    println!("Empty cols: {:?}", map.empty_cols);
    map.print();
    solve(map.clone(), 2);
    solve(map, 1000000);
}
