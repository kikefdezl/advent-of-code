use std::{collections::HashMap, fs::read_to_string};

const INPUT_FILE: &str = "input.txt";

const EMPTY: char = '.';
const START: char = 'S';
const END: char = 'E';

#[derive(Debug, Clone)]
enum Action {
    FWD,
    CW,
    CCW,
}

impl Action {
    fn is_rotation(&self) -> bool {
        match self {
            Action::CW => true,
            Action::CCW => true,
            Action::FWD => false,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
}

impl Position {
    fn apply_action(&self, action: &Action) -> Position {
        match action {
            Action::FWD => self.move_fwd(),
            Action::CW => self.rotate_cw(),
            Action::CCW => self.rotate_ccw(),
        }
    }

    fn move_fwd(&self) -> Position {
        Position {
            x: (self.x as isize + self.dx) as usize,
            y: (self.y as isize + self.dy) as usize,
            dx: self.dx,
            dy: self.dy,
        }
    }

    fn rotate_cw(&self) -> Position {
        let (dx, dy) = match (self.dx, self.dy) {
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            (0, -1) => (1, 0),
            _ => unreachable!(),
        };
        Position {
            x: self.x,
            y: self.y,
            dx,
            dy,
        }
    }

    fn rotate_ccw(&self) -> Position {
        let (dx, dy) = match (self.dx, self.dy) {
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            (-1, 0) => (0, 1),
            (0, 1) => (1, 0),
            _ => unreachable!(),
        };
        Position {
            x: self.x,
            y: self.y,
            dx,
            dy,
        }
    }
}

#[derive(Debug, Clone)]
struct Path {
    actions: Vec<Action>,
    cost: u32,
}

impl Path {
    fn simulate(&self, start: &Position) -> Position {
        let mut position = start.clone();
        for action in &self.actions {
            position = position.apply_action(action);
        }
        position
    }

    fn last_rotated(&self) -> bool {
        let len = self.actions.len();
        if len < 1 {
            return false;
        }
        self.actions[len - 1].is_rotation()
    }
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize,
    start: Position,
}

impl Grid {
    fn from_text(text: &str) -> Grid {
        let mut start = Position {
            x: 0,
            y: 0,
            dx: 1,
            dy: 0,
        };
        let mut data: Vec<Vec<char>> = Vec::new();

        for (y, line) in text.lines().enumerate() {
            data.push(line.chars().collect());

            if let Some(x) = line.find(START) {
                (start.x, start.y) = (x, y);
            }
        }

        Grid {
            height: data.len(),
            width: data[0].len(),
            data,
            start,
        }
    }

    fn print_with_path(&self, path: &Path) {
        let mut position = self.start.clone();
        let mut positions: HashMap<(usize, usize), char> = HashMap::new();
        for action in &path.actions {
            position = position.apply_action(&action);
            let char = match (position.dx, position.dy) {
                (1, 0) => '>',
                (-1, 0) => '<',
                (0, 1) => 'v',
                (0, -1) => '^',
                _ => unreachable!(),
            };
            positions.insert((position.x, position.y), char);
        }
        for y in 0..self.height {
            for x in 0..self.width {
                match positions.get(&(x, y)) {
                    Some(c) => print!("{}", c),
                    None => print!("{}", self.data[y][x]),
                }
            }
            println!("");
        }
        println!("");
    }
}

fn insert_sorted(paths: &mut Vec<Path>, new_path: Path) {
    if paths.len() == 0 {
        paths.push(new_path);
        return;
    }
    let mut idx = 0;
    while idx < paths.len() && paths[idx].cost < new_path.cost {
        idx += 1;
    }
    paths.insert(idx, new_path);
}

fn solve(grid: &Grid) -> Vec<Path> {
    let start = Path {
        actions: Vec::new(),
        cost: 0,
    };
    let mut paths: Vec<Path> = vec![start];
    let mut visited: HashMap<Position, u32> = HashMap::new();
    let mut best_paths: Vec<Path> = Vec::new();
    while !paths.is_empty() {
        let current = paths.remove(0);
        let position = current.simulate(&grid.start);

        let next = position.move_fwd();

        // if visited already and cost was less, skip
        if let Some(cost) = visited.get(&next) {
            if cost < &(current.cost + 1) {
                continue;
            }
        }

        if grid.data[next.y][next.x] == EMPTY {
            let mut new_path = current.clone();
            new_path.actions.push(Action::FWD);
            new_path.cost += 1;
            visited.insert(next, new_path.cost);
            insert_sorted(&mut paths, new_path);
        } else if grid.data[next.y][next.x] == END {
            let mut best = current.clone();
            best.actions.push(Action::FWD);
            best.cost += 1;
            let len = best_paths.len();
            if len == 0 || best_paths[0].cost == best_paths[len - 1].cost {
                best_paths.push(best);
            } else {
                return best_paths;
            }
        }

        if !current.last_rotated() {
            let next_cw = position.rotate_cw().move_fwd();
            if grid.data[next_cw.y][next_cw.x] == EMPTY {
                let mut new_path = current.clone();
                new_path.actions.push(Action::CW);
                new_path.cost += 1000;

                insert_sorted(&mut paths, new_path);
            }
            let next_ccw = position.rotate_ccw().move_fwd();
            if grid.data[next_ccw.y][next_ccw.x] == EMPTY {
                let mut new_path = current.clone();
                new_path.actions.push(Action::CCW);
                new_path.cost += 1000;
                insert_sorted(&mut paths, new_path);
            }
        }
    }
    best_paths
}

fn count_common_tiles(grid: &Grid, paths: &Vec<Path>) -> u16 {
    let mut visited: Vec<Vec<u16>> = vec![vec![0; grid.width]; grid.height];
    for path in paths {
        let mut position = grid.start.clone();
        for action in &path.actions {
            position = position.apply_action(action);
            visited[position.y][position.x] += 1;
        }
    }
    visited.iter().flatten().filter(|&&value| value > 1).count() as u16
}

fn parts_1_and_2(grid: &Grid) {
    let best_paths = solve(grid);
    grid.print_with_path(&best_paths[0]);
    println!("Minimum cost: {}", best_paths[0].cost);
    println!("Paths with minimum cost: {}", best_paths.len());
    let count = count_common_tiles(&grid, &best_paths);
    println!("{} tiles are visited by more than one path", count);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let grid = Grid::from_text(&input);
    parts_1_and_2(&grid);
}
