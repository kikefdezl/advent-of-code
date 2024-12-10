use std::collections::HashSet;
use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

const DIRECTIONS: [(isize, isize); 4] = [
    (1, 0),  // right
    (-1, 0), // left
    (0, 1),  // down
    (0, -1), //up
];

struct Map {
    data: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Map {
    fn from_data(data: Vec<Vec<u8>>) -> Map {
        let height: usize = data.len();
        let width: usize = data[0].len();
        Map {
            data,
            height,
            width,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn get_adjacent(&self, direction: (isize, isize)) -> Option<Coord> {
        let nx = self.x as isize + direction.0;
        let ny = self.y as isize + direction.1;

        if nx < 0 || ny < 0 {
            return None;
        }
        Some(Coord {
            x: nx as usize,
            y: ny as usize,
        })
    }
}

struct Trail {
    data: Vec<Coord>,
}

fn find_trails(map: &Map, start: &Coord) -> Vec<Trail> {
    let current = map.data[start.y][start.x];
    if current == 9 {
        return vec![Trail {
            data: vec![start.clone()],
        }];
    }

    let mut trails: Vec<Trail> = Vec::new();
    for direction in DIRECTIONS {
        let adjacent: Coord = match start.get_adjacent(direction) {
            Some(coord) => coord,
            None => continue,
        };

        if adjacent.x >= map.width || adjacent.y >= map.height {
            continue;
        }

        let adjacent_value: u8 = map.data[adjacent.y][adjacent.x];
        if adjacent_value == current + 1 {
            let new_trails = find_trails(map, &adjacent);
            for mut new_trail in new_trails {
                new_trail.data.insert(0, start.clone());
                trails.push(new_trail);
            }
        }
    }
    trails
}

fn filter_unique_trails(trails: Vec<Trail>) -> Vec<Trail> {
    // keep only trails that start and end at different positions
    let mut unique = Vec::new();
    let mut seen: HashSet<(Coord, Coord)> = HashSet::new();

    for trail in trails {
        let first = trail.data[0].clone();
        let last = trail.data[trail.data.len() - 1].clone();

        if !seen.contains(&(first.clone(), last.clone())) {
            seen.insert((first, last));
            unique.push(trail);
        }
    }
    unique
}

fn parts_one_and_two(map: &Map) {
    let mut trails: Vec<Trail> = Vec::new();
    for y in 0..map.height {
        for x in 0..map.width {
            if map.data[y][x] != 0 {
                continue;
            }
            let coord = Coord { x, y };
            trails.extend(find_trails(&map, &coord));
        }
    }
    println!("Found {} total trails", trails.len());

    let trails = filter_unique_trails(trails);
    println!("Found {} unique trails", trails.len());
}

fn main() {
    let contents = read_to_string(INPUT_FILE).unwrap();
    let data: Vec<Vec<u8>> = contents
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|y| y.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();
    let map = Map::from_data(data);
    parts_one_and_two(&map);
}
