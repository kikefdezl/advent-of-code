use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

impl Coord {
    fn from_str(s: &str) -> Coord {
        let mut parts = s.trim().split(",");
        Coord {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap(),
            z: parts.next().unwrap().parse().unwrap(),
        }
    }

    fn above(&self) -> Coord {
        Coord {
            x: self.x,
            y: self.y,
            z: self.z + 1,
        }
    }

    fn below(&self) -> Coord {
        Coord {
            x: self.x,
            y: self.y,
            z: self.z - 1,
        }
    }
}

#[derive(Debug, Clone)]
struct Brick {
    a: Coord,
    b: Coord,
}

impl Brick {
    fn from_str(s: &str) -> Brick {
        let mut parts = s.split("~");
        Brick {
            a: Coord::from_str(parts.next().unwrap()),
            b: Coord::from_str(parts.next().unwrap()),
        }
    }

    fn all_coords(&self) -> Vec<Coord> {
        let mut occupied = HashSet::new();
        occupied.insert(self.a.clone());
        occupied.insert(self.b.clone());
        for x in self.a.x..self.b.x {
            occupied.insert(Coord {
                x,
                y: self.a.y,
                z: self.a.z,
            });
        }
        for y in self.a.y..self.b.y {
            occupied.insert(Coord {
                x: self.a.x,
                y,
                z: self.a.z,
            });
        }
        for z in self.a.z..self.b.z {
            occupied.insert(Coord {
                x: self.a.x,
                y: self.a.y,
                z,
            });
        }
        occupied.into_iter().collect()
    }

    fn can_drop(&self, space: &Space) -> bool {
        let occupied = self.all_coords();
        if occupied.iter().any(|o| o.z == 1) {
            return false;
        }
        let mut below = occupied.iter().map(|o| space.get(&o.below()));
        if !below.all(|b| b.is_none()) {
            return false;
        }
        true
    }

    fn drop(&mut self) {
        self.a.z -= 1;
        self.b.z -= 1;
    }
}

type BrickId = usize;

struct Layer {
    cubes: Vec<Vec<Option<BrickId>>>,
}

impl Layer {
    fn new_empty(x_len: usize, y_len: usize) -> Layer {
        let mut rows = Vec::new();
        for _ in 0..y_len {
            let cubes = (0..x_len).map(|_| None).collect();
            rows.push(cubes);
        }
        Layer { cubes: rows }
    }
}

struct Space {
    layers: Vec<Layer>,
}

impl Space {
    fn new_empty(x_len: usize, y_len: usize, z_len: usize) -> Space {
        let layers = (0..z_len).map(|_| Layer::new_empty(x_len, y_len)).collect();
        Space { layers }
    }

    fn get(&self, coord: &Coord) -> Option<BrickId> {
        self.layers[coord.z].cubes[coord.y][coord.x]
    }

    fn insert(&mut self, brick_id: &BrickId, brick: &Brick) {
        let occupied = brick.all_coords();
        for coord in occupied {
            self.layers[coord.z].cubes[coord.y][coord.x] = Some(*brick_id);
        }
    }

    fn compute_supports_map(&self) -> HashMap<BrickId, Vec<BrickId>> {
        let mut map: HashMap<BrickId, Vec<BrickId>> = HashMap::new();
        for (z, layer) in self.layers[..self.layers.len() - 1].iter().enumerate() {
            for (y, row) in layer.cubes.iter().enumerate() {
                for (x, current) in row.iter().enumerate() {
                    let coord = Coord { x, y, z };
                    let above = self.get(&coord.above());

                    if let Some(c) = current {
                        if !map.contains_key(c) {
                            map.insert(*c, Vec::new());
                        };
                        if let Some(b) = above
                            && b != *c
                        {
                            map.get_mut(c).unwrap().push(b);
                        }
                    }
                }
            }
        }
        map
    }
}

fn get_bricks_bounds(bricks: &[Brick]) -> (usize, usize, usize) {
    let (mut max_x, mut max_y, mut max_z) = (0, 0, 0);
    for brick in bricks {
        let x = max(brick.a.x, brick.b.x);
        if x > max_x {
            max_x = x;
        }

        let y = max(brick.a.y, brick.b.y);
        if y > max_y {
            max_y = y;
        }

        let z = max(brick.a.z, brick.b.z);
        if z > max_z {
            max_z = z;
        }
    }
    (max_x, max_y, max_z)
}

fn invert_map(map: &HashMap<BrickId, Vec<BrickId>>) -> HashMap<BrickId, Vec<BrickId>> {
    let mut inverted: HashMap<BrickId, Vec<BrickId>> = HashMap::new();
    for (k, v) in map {
        for value in v {
            inverted.entry(*value).or_default().push(*k);
        }
    }
    inverted
}

type BrickMap = HashMap<BrickId, Brick>;

fn simulate_fall(bricks: &mut [Brick]) -> Space {
    bricks.sort_by(|a, b| a.a.z.cmp(&b.a.z));
    let mut brickmap: BrickMap = bricks
        .iter()
        .enumerate()
        .map(|(i, b)| (i, b.clone()))
        .collect();
    let (max_x, max_y, max_z) = get_bricks_bounds(bricks);
    let mut space = Space::new_empty(max_x + 1, max_y + 1, max_z + 1);

    for brick_id in 0..bricks.len() {
        let brick = brickmap.get_mut(&brick_id).unwrap();
        while brick.can_drop(&space) {
            brick.drop();
        }
        space.insert(&brick_id, brick);
    }
    space
}

fn part1(bricks: &mut [Brick]) {
    let space = simulate_fall(bricks);

    let supports_map = space.compute_supports_map();
    let supported_by_map = invert_map(&supports_map);

    let mut sum = 0;
    'outer: for (brick, bricks_above) in supports_map {
        for brick_above in bricks_above {
            let supports = supported_by_map.get(&brick_above).unwrap();
            if supports.iter().all(|b| *b == brick) {
                continue 'outer;
            }
        }
        sum += 1;
    }
    println!("{} bricks can be safely desintegrated", sum);
}

fn parse_bricks(s: &str) -> Vec<Brick> {
    let mut bricks = Vec::new();
    for line in s.lines() {
        bricks.push(Brick::from_str(line));
    }
    bricks
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let mut bricks = parse_bricks(&input);
    part1(&mut bricks);
}
