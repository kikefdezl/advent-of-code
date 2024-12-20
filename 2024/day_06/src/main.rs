// I implemented part 2 in two different ways:
// - Using a HashSet to check if a (x, y, dx, dy) ever repeats.
// - Counting steps and exiting if we surpass the max
//   possible steps (the area of the map).
// Counting steps ended up being much faster.

use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

const OBSTACLE: char = '#';
const VISITED: char = 'X';
const UNVISITED: char = '.';
const START: char = '^';
const START_DX: isize = 0;
const START_DY: isize = -1;

#[derive(Clone)]
struct Map {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize,
    start: (isize, isize),
}

impl Map {
    fn new(data: Vec<Vec<char>>) -> Map {
        let height = data.len();
        let width = data[0].len();
        let start = Map::find_start(&data);
        Map {
            data,
            height,
            width,
            start,
        }
    }

    fn find_start(map: &Vec<Vec<char>>) -> (isize, isize) {
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == START {
                    return (x as isize, y as isize);
                }
            }
        }
        panic!();
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", &self.data[y][x]);
            }
            println!("");
        }
    }
}

fn rotate(dx: isize, dy: isize) -> (isize, isize) {
    match (dx, dy) {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => panic!(),
    }
}

fn part_one(mut map: Map) {
    let mut count = 1;

    let (mut x, mut y) = (map.start.0, map.start.1);
    let mut dx = START_DX;
    let mut dy = START_DY;

    loop {
        let mut nx = x + dx;
        let mut ny = y + dy;

        if nx < 0 || nx >= map.width as isize || ny < 0 || ny >= map.height as isize {
            break;
        }

        while map.data[ny as usize][nx as usize] == OBSTACLE {
            nx -= dx;
            ny -= dy;
            (dx, dy) = rotate(dx, dy);
            nx += dx;
            ny += dy;
        }

        x = nx;
        y = ny;
        if map.data[y as usize][x as usize] == UNVISITED {
            map.data[y as usize][x as usize] = VISITED;
            count += 1;
        }
    }
    println!("Total visited places: {}", count);
}

fn part_two(map: Map) {
    let mut count = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if map.data[y as usize][x as usize] == OBSTACLE
                || map.data[y as usize][x as usize] == START
            {
                continue;
            }

            let mut obstacle_map = map.clone();
            obstacle_map.data[y as usize][x as usize] = OBSTACLE;

            if is_map_endless_loop(obstacle_map) {
                count += 1;
            }
        }
    }
    println!("Total infinite loop positions: {}", count);
}

fn is_map_endless_loop(mut map: Map) -> bool {
    // finds endless loop by exiting if we surpass the max amount of possible
    // steps (area of the map).
    let (mut x, mut y) = map.start;
    let mut dx = START_DX;
    let mut dy = START_DY;

    let max_steps = map.width * map.height - 1;

    let mut steps = 1;
    loop {
        let mut nx = x + dx;
        let mut ny = y + dy;

        if nx < 0 || nx >= map.width as isize || ny < 0 || ny >= map.height as isize {
            return false;
        }

        while map.data[ny as usize][nx as usize] == OBSTACLE {
            nx -= dx;
            ny -= dy;
            (dx, dy) = rotate(dx, dy);
            nx += dx;
            ny += dy;
        }

        x = nx;
        y = ny;
        if map.data[y as usize][x as usize] == UNVISITED {
            map.data[y as usize][x as usize] = VISITED;
        }
        steps += 1;
        if steps > max_steps {
            return true;
        }
    }
}

fn main() {
    let contents = read_to_string(INPUT_FILE).unwrap();
    let data: Vec<Vec<char>> = contents.lines().map(|x| x.chars().collect()).collect();
    let map = Map::new(data);

    part_one(map.clone());
    part_two(map.clone());
}
