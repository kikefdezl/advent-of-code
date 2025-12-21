use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

const INPUT_FILE: &str = "input.txt";

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn neighbor(&self, dir: &Direction) -> Position {
        match dir {
            Direction::North => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::East => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::South => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Position {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn all() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }
}

#[derive(PartialEq, Clone)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    start: Position,
    end: Position,
}

impl Map {
    fn from_str(s: &str) -> Map {
        let mut tiles = Vec::new();
        for line in s.lines() {
            let mut row = Vec::new();
            for ch in line.chars() {
                let tile = match ch {
                    '#' => Tile::Forest,
                    '.' => Tile::Path,
                    '^' => Tile::Slope(Direction::North),
                    '>' => Tile::Slope(Direction::East),
                    'v' => Tile::Slope(Direction::South),
                    '<' => Tile::Slope(Direction::West),
                    _ => unreachable!(),
                };
                row.push(tile);
            }
            tiles.push(row);
        }
        let start = Position {
            x: tiles[0].iter().position(|x| *x == Tile::Path).unwrap(),
            y: 0,
        };
        let end = Position {
            x: tiles
                .last()
                .unwrap()
                .iter()
                .position(|x| *x == Tile::Path)
                .unwrap(),
            y: tiles.len() - 1,
        };
        Map { tiles, start, end }
    }

    fn get(&self, pos: &Position) -> Tile {
        self.tiles[pos.y][pos.x].clone()
    }

    fn width(&self) -> usize {
        self.tiles[0].len()
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn can_move(&self, pos: &Position, dir: &Direction) -> bool {
        match dir {
            Direction::North => {
                if pos.y == 0 {
                    return false;
                }
            }
            Direction::East => {
                if pos.x == self.width() - 1 {
                    return false;
                }
            }
            Direction::South => {
                if pos.y == self.height() - 1 {
                    return false;
                }
            }
            Direction::West => {
                if pos.x == 0 {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(Clone, Debug)]
struct Path {
    pos: Position,
    visited: HashSet<Position>,
    len: usize,
}

fn part1(map: &Map) {
    let starting_path = Path {
        pos: map.start.clone(),
        visited: HashSet::from([map.start.clone()]),
        len: 0,
    };

    let mut paths: VecDeque<Path> = VecDeque::from(vec![starting_path.clone()]);
    let mut longest = starting_path;

    while let Some(path) = paths.pop_front() {
        if path.pos == map.end && path.len > longest.len {
            longest = path;
            continue;
        }

        let mut valid = Vec::new();
        if let Tile::Slope(dir) = map.get(&path.pos) {
            let neighbor = path.pos.neighbor(&dir);
            if !path.visited.contains(&neighbor) {
                valid.push(dir);
            }
        } else {
            for dir in Direction::all() {
                if !map.can_move(&path.pos, &dir) {
                    continue;
                }
                let neighbor = path.pos.neighbor(&dir);
                if map.get(&neighbor) != Tile::Forest && !path.visited.contains(&neighbor) {
                    valid.push(dir);
                }
            }
        }

        for v in valid {
            let neigh = path.pos.neighbor(&v);
            let mut new = path.clone();
            new.visited.insert(neigh.clone());
            new.pos = neigh;
            new.len += 1;
            paths.push_back(new);
        }
    }

    println!("Longest hike with slippery slopes: {}", longest.len);
}

fn longest_path(
    map: &Map,
    pos: &Position,
    visited: &mut HashSet<Position>,
    len: usize,
    best: &mut usize,
) {
    if *pos == map.end && len > *best {
        *best = len;
    }

    for dir in Direction::all() {
        if !map.can_move(pos, &dir) {
            continue;
        }

        let next = pos.neighbor(&dir);
        if map.get(&next) == Tile::Forest || visited.contains(&next) {
            continue;
        }

        visited.insert(next.clone());
        longest_path(map, &next, visited, len + 1, best);
        visited.remove(&next);
    }
}

fn part2(map: &Map) {
    let start = map.start.clone();
    let mut visited = HashSet::from([start.clone()]);
    let mut best = 0;
    longest_path(map, &start, &mut visited, 0, &mut best);
    println!("Without them: {}", best);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let map = Map::from_str(&input);
    part1(&map);
    part2(&map);
}
