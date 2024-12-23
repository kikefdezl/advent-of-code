use std::{cmp::Ordering, collections::HashSet, fs::read_to_string};

const INPUT_FILE: &str = "input.txt";
const GRID_SIZE: usize = 71;
const TAKE: usize = 1024;

const START: (usize, usize) = (0, 0);
const END: (usize, usize) = (GRID_SIZE - 1, GRID_SIZE - 1);

type Path = Vec<Position>;

#[derive(Clone)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn from_text(text: &str) -> Position {
        let comma = text.find(',').unwrap();
        Position {
            x: text[..comma].parse().unwrap(),
            y: text[comma + 1..].parse().unwrap(),
        }
    }

    fn move_(&self, direction: &Direction) -> Option<Position> {
        match direction {
            Direction::NORTH => match self.y {
                0 => None,
                _ => Some(Position {
                    x: self.x,
                    y: self.y - 1,
                }),
            },
            Direction::SOUTH => match self.y.cmp(&(GRID_SIZE - 1)) {
                Ordering::Less => Some(Position {
                    x: self.x,
                    y: self.y + 1,
                }),
                _ => None,
            },
            Direction::WEST => match self.x {
                0 => None,
                _ => Some(Position {
                    x: self.x - 1,
                    y: self.y,
                }),
            },
            Direction::EAST => match self.x.cmp(&(GRID_SIZE - 1)) {
                Ordering::Less => Some(Position {
                    x: self.x + 1,
                    y: self.y,
                }),
                _ => None,
            },
        }
    }
}

#[derive(Clone)]
struct Grid {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            data: vec![vec!['.'; GRID_SIZE]; GRID_SIZE],
            width: GRID_SIZE,
            height: GRID_SIZE,
        }
    }

    fn corrupt_with(&mut self, bytes: &[Position]) {
        for byte in bytes {
            self.data[byte.y][byte.x] = '#';
        }
    }

    fn print_with_path(&self, path: &Path) {
        let positions: HashSet<Position> = HashSet::from_iter(path.iter().cloned());
        for y in 0..self.height {
            for x in 0..self.width {
                if positions.contains(&Position { x, y }) {
                    print!("O");
                } else {
                    print!("{}", self.data[y][x]);
                }
            }
            println!("");
        }
        println!("");
    }
}

fn solve(grid: &Grid) -> Option<Path> {
    let start: Path = vec![Position {
        x: START.0,
        y: START.1,
    }];
    let mut paths: Vec<Path> = vec![start];
    let mut visited: HashSet<Position> = HashSet::new();

    loop {
        if paths.is_empty() {
            return None;
        }
        let current = paths.remove(0);
        let position = current.last().unwrap();

        for direction in [
            Direction::NORTH,
            Direction::EAST,
            Direction::WEST,
            Direction::SOUTH,
        ] {
            let next = match position.move_(&direction) {
                None => continue,
                Some(n) => n,
            };

            if grid.data[next.y][next.x] == '#' || visited.contains(&next) {
                continue;
            }

            let mut new_path = current.clone();
            if next.x == END.0 && next.y == END.1 {
                return Some(new_path);
            }

            visited.insert(next.clone());
            new_path.push(next);
            paths.push(new_path);
        }
    }
}

fn part1(grid: &mut Grid, bytes: &Vec<Position>) {
    grid.corrupt_with(&bytes[..TAKE]);
    let path = solve(grid).unwrap();
    grid.print_with_path(&path);
    println!("Minimum steps: {}", path.len());
}

fn part2(grid: &Grid, bytes: &Vec<Position>) {
    for t in TAKE..bytes.len() - 1 {
        let mut new_grid = grid.clone();
        new_grid.corrupt_with(&bytes[..t]);
        if solve(&new_grid).is_none() {
            println!(
                "No more paths after Byte at: {},{}",
                bytes[t - 1].x,
                bytes[t - 1].y
            );
            break;
        }
    }
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let mut bytes: Vec<Position> = Vec::new();
    for line in input.trim().lines() {
        bytes.push(Position::from_text(line));
    }

    let mut grid = Grid::new();
    part1(&mut grid, &bytes);
    part2(&grid, &bytes);
}
