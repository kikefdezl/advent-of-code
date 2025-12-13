use std::{
    collections::{BinaryHeap, HashSet},
    fs,
};

const INPUT_FILE: &str = "input.txt";

#[derive(PartialEq, Hash, Eq, Clone)]
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

#[derive(PartialEq, Clone, Eq, Hash, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

struct Grid {
    tiles: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn get(&self, pos: &Position) -> usize {
        self.tiles[pos.y][pos.x]
    }

    fn from_string(s: &str) -> Grid {
        let mut tiles: Vec<Vec<usize>> = Vec::new();
        for l in s.lines() {
            let as_ints: Vec<usize> = l
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect();
            tiles.push(as_ints);
        }
        let height = tiles.len();
        let width = tiles[0].len();
        Grid {
            tiles,
            width,
            height,
        }
    }
}

#[derive(Eq, PartialEq)]
struct State {
    pos: Position,
    heat_loss: usize,
    last_dir: Option<Direction>,
    consecutive: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // NOTE: this is reversed since BinaryHeap is a max-heap and we want a min-heap!
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn start() -> State {
        State {
            pos: Position { x: 0, y: 0 },
            heat_loss: 0,
            last_dir: None,
            consecutive: 0,
        }
    }

    fn cache_key(&self) -> (Position, Option<Direction>, usize) {
        (self.pos.clone(), self.last_dir, self.consecutive)
    }
}

fn solve(grid: &Grid, min_consec: usize, max_consec: usize) -> usize {
    let start = State::start();
    let end_pos = Position {
        x: grid.width - 1,
        y: grid.height - 1,
    };

    let mut visited: HashSet<(Position, Option<Direction>, usize)> = HashSet::new();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(start);

    while let Some(curr) = heap.pop() {
        if curr.pos == end_pos {
            return curr.heat_loss;
        }

        let cache = curr.cache_key();
        if !visited.insert(cache.clone()) {
            continue;
        }

        // gather all possible next directions
        let valid_dirs: Vec<Direction> = Direction::all()
            .into_iter()
            .filter(|d| {
                if let Some(last) = curr.last_dir {
                    if *d == last.opposite() {
                        return false;
                    }
                    if curr.consecutive < min_consec && *d != last {
                        return false;
                    }
                    if curr.consecutive >= max_consec && *d == last {
                        return false;
                    }
                }
                match d {
                    Direction::North => curr.pos.y > 0,
                    Direction::West => curr.pos.x > 0,
                    Direction::South => curr.pos.y < grid.height - 1,
                    Direction::East => curr.pos.x < grid.width - 1,
                }
            })
            .collect();

        for dir in valid_dirs {
            let pos = curr.pos.move_(&dir);
            let heat_loss = curr.heat_loss + grid.get(&pos);
            let consecutive = if curr.last_dir == Some(dir) {
                curr.consecutive + 1
            } else {
                1
            };
            heap.push(State {
                pos,
                heat_loss,
                last_dir: Some(dir),
                consecutive,
            });
        }
    }
    0
}

fn part1(grid: &Grid) {
    let heat_loss = solve(grid, 0, 3);
    println!("Smallest heat loss: {heat_loss}");
}

fn part2(grid: &Grid) {
    let heat_loss = solve(grid, 4, 10);
    println!("With ultra crucibles: {heat_loss}");
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    let grid = Grid::from_string(&input);
    part1(&grid);
    part2(&grid);
}
