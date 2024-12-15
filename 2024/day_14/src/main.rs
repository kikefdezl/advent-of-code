use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

const AREA_WIDTH: i16 = 101;
const AREA_HEIGHT: i16 = 103;

const SECONDS_100: u8 = 100;
const SECONDS_10K: u16 = 10000;

#[derive(Clone)]
struct Robot {
    x: i16,
    y: i16,
    vx: i16,
    vy: i16,
}

impl Robot {
    fn from_str(str: &str) -> Robot {
        let mut parts = str.split_whitespace();

        let p = parts.next().unwrap();
        let comma = p.find(",").unwrap();
        let x: i16 = p[2..comma].parse().unwrap();
        let y: i16 = p[comma + 1..].parse().unwrap();

        let v = parts.next().unwrap();
        let comma = v.find(",").unwrap();
        let vx: i16 = v[2..comma].parse().unwrap();
        let vy: i16 = v[comma + 1..].parse().unwrap();

        Robot { x, y, vx, vy }
    }

    fn move_(&mut self) {
        let mut nx = self.x + self.vx;
        while nx < 0 {
            nx += AREA_WIDTH;
        }
        while nx >= AREA_WIDTH {
            nx -= AREA_WIDTH;
        }
        let mut ny = self.y + self.vy;
        while ny < 0 {
            ny += AREA_HEIGHT;
        }
        while ny >= AREA_HEIGHT {
            ny -= AREA_HEIGHT;
        }
        self.x = nx;
        self.y = ny;
    }

    fn quadrant(&self) -> Option<u8> {
        let (hx, hy) = (AREA_WIDTH / 2, AREA_HEIGHT / 2);
        if self.x == hx || self.y == hy {
            return None;
        }
        let (qx, qy) = (self.x < hx, self.y < hy);
        match (qx, qy) {
            (true, true) => Some(1),
            (false, true) => Some(2),
            (true, false) => Some(3),
            (false, false) => Some(4),
        }
    }
}

fn part_one(mut robots: Vec<Robot>) {
    for _ in 0..SECONDS_100 {
        for robot in &mut robots {
            robot.move_();
        }
    }
    let mut quadrants = vec![0; 4];
    for robot in &robots {
        let quadrant = match robot.quadrant() {
            Some(q) => q as usize,
            None => continue,
        };
        quadrants[quadrant - 1] += 1;
    }
    println!(
        "Safety factor is: {}",
        quadrants.iter().fold(1, |acc, x| acc * x)
    );
}

fn part_two(mut robots: Vec<Robot>) {
    let mut frame = 0;
    let mut max_consecutives = 0;
    for s in 1..SECONDS_10K {
        let mut area = vec![vec![0; AREA_WIDTH as usize]; AREA_HEIGHT as usize];

        for robot in &mut robots {
            robot.move_();
            area[robot.y as usize][robot.x as usize] += 1;
        }

        for row in area {
            let mut consecutives = 0;
            for cell in row {
                if cell == 0 {
                    consecutives = 0;
                } else {
                    consecutives += 1;
                    if consecutives > max_consecutives {
                        max_consecutives = consecutives;
                        frame = s;
                    }
                }
            }
        }
    }
    println!("After {} seconds has most consecutives", frame);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let robots: Vec<Robot> = input.trim().lines().map(|s| Robot::from_str(s)).collect();
    part_one(robots.clone());
    part_two(robots.clone());
}
