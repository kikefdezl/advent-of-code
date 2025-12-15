use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn step(&self, dir: &Direction, count: usize) -> Coord {
        match dir {
            Direction::Up => Coord {
                x: self.x,
                y: self.y - count as isize,
            },
            Direction::Right => Coord {
                x: self.x + count as isize,
                y: self.y,
            },
            Direction::Down => Coord {
                x: self.x,
                y: self.y + count as isize,
            },
            Direction::Left => Coord {
                x: self.x - count as isize,
                y: self.y,
            },
        }
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Instruction {
    direction: Direction,
    steps: usize,
    color: String,
}

impl Instruction {
    fn from_string(s: &str) -> Vec<Instruction> {
        let mut instructions: Vec<Instruction> = Vec::new();

        for line in s.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let direction = match parts[0].trim().chars().collect::<Vec<char>>()[0] {
                'U' => Direction::Up,
                'R' => Direction::Right,
                'D' => Direction::Down,
                'L' => Direction::Left,
                _ => unreachable!(),
            };
            let steps: usize = parts[1].trim().parse().expect("Should parse to a number");
            let color: String = parts[2].trim_matches(['(', ')']).to_string();
            instructions.push(Instruction {
                direction,
                steps,
                color,
            });
        }
        instructions
    }

    fn convert(&self) -> Instruction {
        let steps = usize::from_str_radix(&self.color[1..6], 16).unwrap();
        let direction = match self.color.chars().last().unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => unreachable!(),
        };

        Instruction {
            direction,
            steps,
            color: self.color.clone(),
        }
    }
}

// Shoelace formula
fn area(coords: &[Coord]) -> usize {
    let mut sum = 0;
    let mut perimeter = 0;
    for i in 0..coords.len() - 1 {
        sum += coords[i].x * coords[i + 1].y;
        sum -= coords[i].y * coords[i + 1].x;
        perimeter += coords[i].x.abs_diff(coords[i + 1].x);
        perimeter += coords[i].y.abs_diff(coords[i + 1].y);
    }

    let area = (sum / 2) + (perimeter.div_ceil(2) as isize) + 1;
    area.unsigned_abs()
}

fn compute_coords(instructions: &[Instruction]) -> Vec<Coord> {
    let mut coords = vec![Coord { x: 0, y: 0 }];

    for instruction in instructions {
        let next = coords
            .last()
            .unwrap()
            .step(&instruction.direction, instruction.steps);
        coords.push(next);
    }
    coords
}

fn part1(instructions: &[Instruction]) {
    let coords = compute_coords(instructions);
    let area = area(&coords);

    println!("Area of the polygon: {}", area);
}

fn part2(instructions: &[Instruction]) {
    let converted: Vec<Instruction> = instructions.iter().map(|i| i.convert()).collect();
    let coords = compute_coords(&converted);
    let area = area(&coords);

    println!("With converted instructions: {}", area);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let instructions = Instruction::from_string(&input);
    part1(&instructions);
    part2(&instructions);
}
