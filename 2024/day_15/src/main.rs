use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

const LEFT: char = '<';
const RIGHT: char = '>';
const UP: char = '^';
const DOWN: char = 'v';

const WALL: char = '#';
const BOX: char = 'O';
const ROBOT: char = '@';
const EMPTY: char = '.';

const BOX_L: char = '[';
const BOX_R: char = ']';

#[derive(Clone)]
struct Map {
    data: Vec<Vec<char>>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Map {
    fn from_text(input: &str) -> Map {
        let mut data: Vec<Vec<char>> = input
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect();

        let height = data.len();
        let width = data[0].len();

        for y in 0..height {
            for x in 0..width {
                if data[y][x] == ROBOT {
                    data[y][x] = EMPTY;
                    return Map {
                        data,
                        width,
                        height,
                        x,
                        y,
                    };
                }
            }
        }
        panic!("Couldn't parse map");
    }

    fn extend(&self) -> Map {
        let new_width = self.width * 2;
        let mut data = Vec::with_capacity(self.height);
        for row in &self.data {
            let mut new_row = Vec::with_capacity(new_width);
            for &cell in row {
                let (ca, cb) = match cell {
                    EMPTY => (EMPTY, EMPTY),
                    WALL => (WALL, WALL),
                    BOX => (BOX_L, BOX_R),
                    _ => unreachable!(),
                };
                new_row.push(ca);
                new_row.push(cb);
            }
            data.push(new_row);
        }
        Map {
            data,
            width: new_width,
            height: self.height,
            x: self.x * 2,
            y: self.y,
        }
    }

    fn move_robot(&mut self, dir: char) {
        let (dx, dy) = match dir {
            LEFT => (-1, 0),
            RIGHT => (1, 0),
            UP => (0, -1),
            DOWN => (0, 1),
            _ => unreachable!(),
        };

        let mut boxes = HashSet::new();
        self.get_touching_boxes(self.x, self.y, dx, dy, &mut boxes);

        let (nx, ny) = Map::next(self.x, self.y, dx, dy);

        if boxes.is_empty() {
            if self.data[ny][nx] == WALL {
                return;
            }
        } else {
            if !self.boxes_can_move(&boxes, dx, dy) {
                return;
            }
            self.move_boxes(&boxes, dx, dy);
        }

        self.x = nx;
        self.y = ny;
    }

    fn get_touching_boxes(
        &self,
        x: usize,
        y: usize,
        dx: isize,
        dy: isize,
        block: &mut HashSet<(usize, usize)>,
    ) {
        let (ax, ay) = Map::next(x, y, dx, dy);

        let adjacents = match self.data[ay][ax] {
            BOX => vec![(ax, ay)],
            BOX_R => vec![(ax, ay), (ax - 1, ay)],
            BOX_L => vec![(ax, ay), (ax + 1, ay)],
            _ => Vec::new(),
        };
        for adjacent in adjacents {
            if !block.contains(&adjacent) {
                block.insert(adjacent);
                self.get_touching_boxes(adjacent.0, adjacent.1, dx, dy, block);
            }
        }
    }

    fn boxes_can_move(&self, block: &HashSet<(usize, usize)>, dx: isize, dy: isize) -> bool {
        for b in block {
            let (nx, ny) = Map::next(b.0, b.1, dx, dy);
            if self.data[ny][nx] == WALL {
                return false;
            }
        }
        true
    }

    fn move_boxes(&mut self, block: &HashSet<(usize, usize)>, dx: isize, dy: isize) -> bool {
        let mut current = HashMap::new();
        for b in block {
            current.insert((b.0, b.1), self.data[b.1][b.0].clone());
        }
        for b in block {
            let (nx, ny) = Map::next(b.0, b.1, dx, dy);
            self.data[ny][nx] = *current.get(&(b.0, b.1)).unwrap();

            let (px, py) = Map::next(b.0, b.1, -dx, -dy);
            if current.contains_key(&(px, py)) {
                self.data[b.1][b.0] = *current.get(&(px, py)).unwrap();
            } else {
                self.data[b.1][b.0] = EMPTY;
            }
        }
        true
    }

    fn next(x: usize, y: usize, dx: isize, dy: isize) -> (usize, usize) {
        let nx = (x as isize + dx) as usize;
        let ny = (y as isize + dy) as usize;
        (nx, ny)
    }

    fn sum_gps(&self) -> usize {
        let mut sum: usize = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if [BOX, BOX_L].contains(&self.data[y][x]) {
                    sum += (100 * y) + x;
                }
            }
        }
        sum
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.x != x || self.y != y {
                    print!("{}", self.data[y][x])
                } else {
                    print!("{}", ROBOT)
                }
            }
            println!("");
        }
    }
}

fn part_one(mut map: Map, movements: &Vec<char>) {
    for movement in movements {
        map.move_robot(*movement);
    }

    map.print();
    println!("Final GPS sum: {}", map.sum_gps());
}

fn part_two(map: Map, movements: &Vec<char>) {
    let mut map = map.extend();
    for movement in movements {
        map.move_robot(*movement);
    }

    map.print();
    println!("Final GPS sum for wide map: {}", map.sum_gps());
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let blank_line = input.trim().find("\n\n").unwrap();
    let map = Map::from_text(&input[..blank_line]);
    let movements = &input[blank_line..]
        .trim()
        .chars()
        .filter(|c| *c != '\n')
        .collect();

    part_one(map.clone(), &movements);
    part_two(map, &movements);
}
