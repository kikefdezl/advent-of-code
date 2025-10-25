use std::fs::read_to_string;
const INPUT_FILE: &str = "input.txt";

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn step(&self, direction: &Direction) -> Position {
        match direction {
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

#[derive(Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn iter() -> std::slice::Iter<'static, Direction> {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .iter()
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
    data: Vec<Vec<char>>,
    start: Position,
}

impl Grid {
    fn from_text(text: &str) -> Grid {
        let lines = text.lines();
        let mut data: Vec<Vec<char>> = Vec::new();
        let mut start = Position { x: 0, y: 0 };

        for (r, line) in lines.enumerate() {
            let chars: Vec<char> = line.trim().chars().collect();
            if chars.contains(&'S') {
                start = Position {
                    x: chars.iter().position(|x| *x == 'S').unwrap(),
                    y: r,
                }
            }
            data.push(chars);
        }
        Grid { data, start }
    }

    fn get(&self, position: &Position) -> char {
        self.data[position.y][position.x]
    }

    fn next(&self, pos: &Position, from: &Direction) -> Option<Direction> {
        let tile = self.get(pos);

        match tile {
            '.' => None,
            '|' => match from {
                Direction::North => Some(Direction::South),
                Direction::South => Some(Direction::North),
                _ => None,
            },
            '-' => match from {
                Direction::East => Some(Direction::West),
                Direction::West => Some(Direction::East),
                _ => None,
            },
            'J' => match from {
                Direction::North => Some(Direction::West),
                Direction::West => Some(Direction::North),
                _ => None,
            },
            'L' => match from {
                Direction::North => Some(Direction::East),
                Direction::East => Some(Direction::North),
                _ => None,
            },
            'F' => match from {
                Direction::South => Some(Direction::East),
                Direction::East => Some(Direction::South),
                _ => None,
            },
            '7' => match from {
                Direction::South => Some(Direction::West),
                Direction::West => Some(Direction::South),
                _ => None,
            },
            'S' => None,
            _ => panic!("Unrecognized tile"),
        }
    }

    fn print(&self) {
        for row in &self.data {
            for c in row {
                print!("{c}");
            }
            println!();
        }
        println!("Start: {:?}", self.start);
    }

    fn find_first_direction(&self) -> Direction {
        for dir in Direction::iter() {
            let next = self.start.step(dir);
            if self.next(&next, &dir.opposite()).is_some() {
                return dir.clone();
            }
        }
        unreachable!();
    }
}

/// shoelace formula
fn polygon_area(polygon: Vec<Position>) -> usize {
    let mut lsum = 0;
    let mut rsum = 0;
    for i in 0..(polygon.len() - 1) {
        lsum += polygon[i].x * polygon[i + 1].y;
        rsum += polygon[i].y * polygon[i + 1].x;
    }
    (lsum - rsum) / 2
}

fn main() {
    let text = read_to_string(INPUT_FILE).unwrap();
    let grid = Grid::from_text(&text);

    grid.print();

    let to = grid.find_first_direction();
    let mut position = grid.start.step(&to);
    let mut from = to.opposite();

    let mut polygon: Vec<Position> = vec![grid.start.clone(), position.clone()];

    let mut steps = 1;

    while grid.get(&position) != 'S' {
        let to = grid.next(&position, &from).unwrap();
        position = position.step(&to);
        polygon.push(position.clone());
        from = to.opposite();
        steps += 1;
    }

    println!("Total loop length: {}", steps);
    println!("Furthest point: {}", steps / 2);

    let area = polygon_area(polygon);
    println!("Polygon area: {:?}", area);
    // I don't know why this formula works, but I inferred it from testing with a few different
    // inputs. I assumed the perimeter would have to be subtracted from the area but I don't
    // know why only half and then + 1.
    // Some quick Googling points to Pick's theorem `A = I + B/2 - 1`
    println!("Points in loop: {:?}", area + 1 - (steps / 2));  
}
