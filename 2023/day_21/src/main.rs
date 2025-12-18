use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";
const STEPS_P1: u8 = 64;
const STEPS_P2: usize = 26501365;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn move_(&self, dir: &Direction) -> Position {
        match dir {
            Direction::North => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::South => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::East => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::West => Position {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Plot,
    Rock,
}

impl Tile {
    fn to_char(self) -> char {
        match self {
            Tile::Plot => '.',
            Tile::Rock => '#',
        }
    }
}

struct Garden {
    tiles: Vec<Vec<Tile>>,
    start: Position,
}

impl Garden {
    fn from_str(s: &str) -> Garden {
        let mut start = Position { x: 0, y: 0 };
        let mut tiles = Vec::new();
        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => row.push(Tile::Plot),
                    '#' => row.push(Tile::Rock),
                    'S' => {
                        row.push(Tile::Plot);
                        start = Position { x, y };
                    }
                    _ => unreachable!(),
                }
            }
            tiles.push(row);
        }
        Garden { tiles, start }
    }

    fn print(&self) {
        for row in &self.tiles {
            for tile in row {
                print!("{}", tile.to_char());
            }
            println!();
        }
        println!("Start: x{} y{}", self.start.x, self.start.y);
    }

    fn get(&self, x: usize, y: usize) -> Tile {
        self.tiles[y][x]
    }

    fn width(&self) -> usize {
        self.tiles[0].len()
    }
    fn height(&self) -> usize {
        self.tiles.len()
    }
}

fn part1(garden: &Garden) {
    let mut positions = HashSet::new();
    positions.insert(garden.start.clone());
    let (w, h) = (garden.width(), garden.height());
    for _ in 0..STEPS_P1 {
        let mut new_positions = HashSet::new();
        for pos in positions {
            if pos.y > 0 && garden.get(pos.x, pos.y - 1) == Tile::Plot {
                new_positions.insert(pos.move_(&Direction::North));
            }
            if pos.x > 0 && garden.get(pos.x - 1, pos.y) == Tile::Plot {
                new_positions.insert(pos.move_(&Direction::West));
            }
            if pos.y < h - 1 && garden.get(pos.x, pos.y + 1) == Tile::Plot {
                new_positions.insert(pos.move_(&Direction::South));
            }
            if pos.x < w - 1 && garden.get(pos.x + 1, pos.y) == Tile::Plot {
                new_positions.insert(pos.move_(&Direction::East));
            }
        }
        positions = new_positions;
    }
    println!("{} positions after {} steps", positions.len(), STEPS_P1);
}

#[derive(Hash, Clone, Eq, PartialEq)]
struct GlobalPosition {
    rel: Position,
    abs_x: isize,
    abs_y: isize,
}

impl GlobalPosition {
    fn move_(&self, dir: &Direction, w: usize, h: usize) -> GlobalPosition {
        let mut new = self.clone();
        match dir {
            Direction::North => {
                if new.rel.y == 0 {
                    new.rel.y = h - 1;
                    new.abs_y -= 1;
                } else {
                    new.rel.y -= 1;
                }
            }
            Direction::West => {
                if new.rel.x == 0 {
                    new.rel.x = w - 1;
                    new.abs_x -= 1;
                } else {
                    new.rel.x -= 1;
                }
            }
            Direction::South => {
                if new.rel.y >= h - 1 {
                    new.rel.y = 0;
                    new.abs_y += 1;
                } else {
                    new.rel.y += 1;
                }
            }
            Direction::East => {
                if new.rel.x >= w - 1 {
                    new.rel.x = 0;
                    new.abs_x += 1;
                } else {
                    new.rel.x += 1;
                }
            }
        };
        new
    }

    fn neighbor(&self, dir: &Direction, w: usize, h: usize) -> GlobalPosition {
        self.move_(dir, w, h)
    }
}

fn solve_tri_equation(values: Vec<(usize, usize)>) -> (f64, f64, f64) {
    let (x0, y0) = (values[0].0 as f64, values[0].1 as f64);
    let (x1, y1) = (values[1].0 as f64, values[1].1 as f64);
    let (x2, y2) = (values[2].0 as f64, values[2].1 as f64);

    let numerator = (y2 - y1) * (x1 - x0) - (y1 - y0) * (x2 - x1);
    let denominator = (x2 * x2 - x1 * x1) * (x1 - x0) - (x1 * x1 - x0 * x0) * (x2 - x1);
    let a = numerator / denominator;
    let b = (y1 - y0 - a * (x1 * x1 - x0 * x0)) / (x1 - x0);
    let c = y0 - a * x0 * x0 - b * x0;

    (a, b, c)
}

fn part2(garden: &Garden) {
    let (w, h) = (garden.width(), garden.height());
    let half = w / 2;
    let sample_at = [half, half + w, half + 2 * w];

    let max_steps = *sample_at.last().unwrap();
    let mut distances: HashMap<GlobalPosition, usize> = HashMap::new();
    let mut queue = VecDeque::new();

    let start = GlobalPosition {
        rel: garden.start.clone(),
        abs_x: 0,
        abs_y: 0,
    };

    queue.push_back((start.clone(), 0));
    distances.insert(start, 0);

    while let Some((pos, dist)) = queue.pop_front()
        && dist < max_steps
    {
        for dir in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            let neighbor = pos.neighbor(&dir, w, h);
            if garden.get(neighbor.rel.x, neighbor.rel.y) == Tile::Plot
                && !distances.contains_key(&neighbor)
            {
                distances.insert(neighbor.clone(), dist + 1);
                queue.push_back((neighbor, dist + 1));
            }
        }
    }

    let mut samples = Vec::new();
    for &target in &sample_at {
        let count = distances
            .values()
            .filter(|&&d| d <= target && d % 2 == target % 2)
            .count();
        samples.push((target, count));
    }

    let (a, b, c) = solve_tri_equation(samples);
    let steps = STEPS_P2 as f64;
    let result = (a * steps * steps + b * steps + c) as usize;
    println!("{} possible plots after {} steps", result, STEPS_P2);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let garden = Garden::from_str(&input);
    garden.print();
    part1(&garden);
    part2(&garden);
}
