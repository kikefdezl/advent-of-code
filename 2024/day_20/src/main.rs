use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

const INPUT_FILE: &str = "input.txt";

const THRESHOLD: usize = 100;

const DIRECTIONS: [Direction; 4] = [
    Direction::NORTH,
    Direction::SOUTH,
    Direction::EAST,
    Direction::WEST,
];

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
    fn move_(&self, direction: &Direction) -> Option<Position> {
        match direction {
            Direction::NORTH => match self.y {
                0 => None,
                _ => Some(Position {
                    x: self.x,
                    y: self.y - 1,
                }),
            },
            Direction::SOUTH => Some(Position {
                x: self.x,
                y: self.y + 1,
            }),
            Direction::WEST => match self.x {
                0 => None,
                _ => Some(Position {
                    x: self.x - 1,
                    y: self.y,
                }),
            },
            Direction::EAST => Some(Position {
                x: self.x + 1,
                y: self.y,
            }),
        }
    }

    fn manhattan_distance(&self, other: &Position) -> usize {
        (self.x as isize - other.x as isize).abs() as usize
            + (self.y as isize - other.y as isize).abs() as usize
    }
}

#[derive(Clone)]
struct Grid {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize,
    start: Position,
    end: Position,
}

impl Grid {
    fn from_text(text: &str) -> Grid {
        let mut start = Position { x: 0, y: 0 };
        let mut end = Position { x: 0, y: 0 };
        let mut data: Vec<Vec<char>> = Vec::new();

        for (y, line) in text.lines().enumerate() {
            data.push(line.chars().collect());

            if let Some(x) = line.find('S') {
                (start.x, start.y) = (x, y);
            }
            if let Some(x) = line.find('E') {
                (end.x, end.y) = (x, y);
            }
        }

        Grid {
            height: data.len(),
            width: data[0].len(),
            data,
            start,
            end,
        }
    }

    fn print_with_path(&self, path: &Path) {
        let positions: HashSet<Position> = HashSet::from_iter(path.iter().cloned());
        for y in 0..self.height {
            for x in 0..self.width {
                if positions.contains(&Position { x, y }) {
                    print!("\x1b[91mO\x1b[0m");
                } else {
                    print!("{}", self.data[y][x]);
                }
            }
            println!("");
        }
        println!("");
    }
}

fn find_path(grid: &Grid) -> Path {
    let start: Path = vec![grid.start.clone()];
    let mut paths: Vec<Path> = vec![start];
    let mut visited: HashSet<Position> = HashSet::new();

    loop {
        let current = paths.remove(0);
        let position = current.last().unwrap();

        for direction in DIRECTIONS {
            let next = match position.move_(&direction) {
                None => continue,
                Some(n) => n,
            };

            if grid.data[next.y][next.x] == '#' || visited.contains(&next) {
                continue;
            }

            let mut new_path = current.clone();
            if next.x == grid.end.x && next.y == grid.end.y {
                new_path.push(next);
                return new_path;
            }

            visited.insert(next.clone());
            new_path.push(next);
            paths.push(new_path);
        }
    }
}

fn find_jumpable(grid: &Grid, position: &Position, distance: usize) -> Vec<Position> {
    let x1 = position.x.saturating_sub(distance);
    let y1 = position.y.saturating_sub(distance);
    let x2 = std::cmp::min(position.x + distance + 1, grid.width - 1);
    let y2 = std::cmp::min(position.y + distance + 1, grid.height - 1);

    let mut jumpables = Vec::new();
    for y in y1..y2 {
        for x in x1..x2 {
            let jump = Position { x, y };
            if position.manhattan_distance(&jump) <= distance
                && ['.', 'E'].contains(&grid.data[y][x])
            {
                jumpables.push(jump);
            }
        }
    }
    jumpables
}

fn solve(grid: &mut Grid, path: &Path, allowed_jumps: usize) -> usize {
    let mut times = HashMap::new();
    for (i, position) in path.iter().enumerate() {
        times.insert(position, path.len() - i);
    }

    let mut cheats = 0;
    for position in path {
        let current_cost = times.get(&position).unwrap();

        let jumpables = find_jumpable(grid, &position, allowed_jumps);

        for jumpable in jumpables {
            let new_cost = times.get(&jumpable).unwrap();
            let cheat_cost = position.manhattan_distance(&jumpable);

            if new_cost + cheat_cost < *current_cost
                && current_cost - new_cost - cheat_cost >= THRESHOLD
            {
                cheats += 1;
            }
        }
    }
    cheats
}

fn part1(grid: &mut Grid, path: &Path) {
    let cheats = solve(grid, path, 2);
    println!("{} cheats of '2' save >={} picoseconds", cheats, THRESHOLD);
}

fn part2(grid: &mut Grid, path: &Path) {
    let cheats = solve(grid, path, 20);
    println!("{} cheats of '20' save >={} picoseconds", cheats, THRESHOLD);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let mut grid = Grid::from_text(&input);

    let path = find_path(&mut grid);
    grid.print_with_path(&path);
    println!("Picoseconds to run: {}", path.len());
    part1(&mut grid, &path);
    part2(&mut grid, &path);
}
