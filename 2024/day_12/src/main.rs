use std::fs::read_to_string;
use std::time::Instant;

const INPUT_FILE: &str = "input.txt";

const EAST: (isize, isize) = (1, 0);
const WEST: (isize, isize) = (-1, 0);
const NORTH: (isize, isize) = (0, -1);
const SOUTH: (isize, isize) = (0, 1);
const NORTH_EAST: (isize, isize) = (1, -1);
const NORTH_WEST: (isize, isize) = (-1, -1);
const SOUTH_EAST: (isize, isize) = (1, 1);
const SOUTH_WEST: (isize, isize) = (-1, 1);

#[derive(Clone)]
struct Map {
    data: Vec<Vec<char>>,
    height: usize,
    width: usize,
    visited: Vec<Vec<bool>>,
}

impl Map {
    fn from_text(text: &str) -> Map {
        let data: Vec<Vec<char>> = text
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let height = data.len();
        let width = data[0].len();
        let visited = vec![vec![false; width]; height];
        Map {
            data,
            height,
            width,
            visited,
        }
    }

    fn is_within_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height
    }
}

struct Region {
    coordinates: Vec<(usize, usize)>,
}

impl Region {
    fn calculate_perimeter(&self) -> usize {
        let mut perimeter = 0;
        let map = self.to_padded_bool_map();
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] {
                    perimeter += 4 - Self::count_adjacent(&map, (x, y));
                }
            }
        }
        perimeter
    }

    fn count_adjacent(map: &Vec<Vec<bool>>, pos: (usize, usize)) -> usize {
        let mut adjacent = 0;
        for dir in [NORTH, EAST, SOUTH, WEST] {
            if Self::get_adjacent_value(&map, pos, dir) {
                adjacent += 1;
            }
        }
        adjacent
    }

    fn calculate_sides(&self) -> usize {
        // sides == corners, so we count total corners
        let map = self.to_padded_bool_map();

        let mut corners = 0;
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if !map[y][x] {
                    continue;
                }

                corners += Self::is_corner(&map, (x, y), NORTH_WEST) as usize;
                corners += Self::is_corner(&map, (x, y), NORTH_EAST) as usize;
                corners += Self::is_corner(&map, (x, y), SOUTH_WEST) as usize;
                corners += Self::is_corner(&map, (x, y), SOUTH_EAST) as usize;
            }
        }
        corners
    }

    fn is_corner(map: &Vec<Vec<bool>>, pos: (usize, usize), corner: (isize, isize)) -> bool {
        let adjacents = match corner {
            NORTH_WEST => (NORTH, WEST),
            SOUTH_WEST => (SOUTH, WEST),
            NORTH_EAST => (NORTH, EAST),
            SOUTH_EAST => (SOUTH, EAST),
            _ => panic!("Not a corner!"),
        };
        let c = Self::get_adjacent_value(&map, pos, corner);
        let a0 = Self::get_adjacent_value(&map, pos, adjacents.0);
        let a1 = Self::get_adjacent_value(&map, pos, adjacents.1);
        (!c && (a0 == a1)) || (c && !a0 && !a1)
    }

    fn to_padded_bool_map(&self) -> Vec<Vec<bool>> {
        let (xmin, ymin, xmax, ymax) = self.get_range();
        let width = xmax - xmin;
        let height = ymax - ymin;
        let mut map: Vec<Vec<bool>> = vec![vec![false; width + 3]; height + 3];
        for coord in &self.coordinates {
            map[coord.1 - ymin + 1][coord.0 - xmin + 1] = true;
        }
        map
    }

    fn get_range(&self) -> (usize, usize, usize, usize) {
        let (mut xmin, mut ymin) = (usize::MAX, usize::MAX);
        let (mut xmax, mut ymax) = (usize::MIN, usize::MIN);
        for coord in &self.coordinates {
            if coord.0 < xmin {
                xmin = coord.0;
            }
            if coord.0 > xmax {
                xmax = coord.0;
            }
            if coord.1 < ymin {
                ymin = coord.1;
            }
            if coord.1 > ymax {
                ymax = coord.1;
            }
        }
        (xmin, ymin, xmax, ymax)
    }

    fn get_adjacent_value(map: &Vec<Vec<bool>>, pos: (usize, usize), dir: (isize, isize)) -> bool {
        let nx = pos.0 as isize + dir.0;
        let ny = pos.1 as isize + dir.1;
        map[ny as usize][nx as usize]
    }
}

fn get_crop_coordinates(map: &mut Map, x: usize, y: usize) -> Vec<(usize, usize)> {
    map.visited[y][x] = true;
    let mut coordinates: Vec<(usize, usize)> = vec![(x, y)];

    let crop_type = map.data[y][x];
    for dir in [EAST, NORTH, WEST, SOUTH] {
        let (nx, ny) = (x as isize + dir.0, y as isize + dir.1);

        if map.is_within_bounds(nx, ny)
            && !map.visited[ny as usize][nx as usize]
            && map.data[ny as usize][nx as usize] == crop_type
        {
            coordinates.extend(get_crop_coordinates(map, nx as usize, ny as usize));
        }
    }
    coordinates
}

fn part_one(mut map: Map) {
    let mut cost = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if map.visited[y][x] {
                continue;
            }
            let region = Region {
                coordinates: get_crop_coordinates(&mut map, x, y),
            };
            cost += region.coordinates.len() * region.calculate_perimeter();
        }
    }
    println!("Total cost with perimeter: {}", cost);
}

fn part_two(mut map: Map) {
    let mut cost = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if map.visited[y][x] {
                continue;
            }
            let region = Region {
                coordinates: get_crop_coordinates(&mut map, x, y),
            };
            cost += region.coordinates.len() * region.calculate_sides();
        }
    }
    println!("Total cost with sides: {}", cost);
}

fn main() {
    let text = read_to_string(INPUT_FILE).unwrap();
    let map = Map::from_text(&text);
    let start = Instant::now();
    part_one(map.clone());
    println!("{:?}", Instant::now() - start);
    let start = Instant::now();
    part_two(map);
    println!("{:?}", Instant::now() - start);
}
