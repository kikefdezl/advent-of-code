use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

// const AREA_MIN: f64 = 7.0;
// const AREA_MAX: f64 = 27.0;
const AREA_MIN: f64 = 200000000000000.0;
const AREA_MAX: f64 = 400000000000000.0;

const N: usize = 6;
const EPS: f64 = 1e-8;

#[derive(Debug)]
struct Position {
    x: f64,
    y: f64,
    z: f64,
}

impl Position {
    fn is_in_area(&self) -> bool {
        self.x > AREA_MIN && self.x < AREA_MAX && self.y > AREA_MIN && self.y < AREA_MAX
    }
}

#[derive(Debug)]
struct Velocity {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
struct Hailstone {
    position: Position,
    velocity: Velocity,
}

impl Hailstone {
    fn xy_intersection_with(&self, other: &Hailstone) -> Option<Position> {
        let m_a = self.velocity.y / self.velocity.x;
        let m_b = other.velocity.y / other.velocity.x;

        //parallel
        if (m_a - m_b).abs() < EPS {
            return None;
        }

        let b_a = -self.position.x / self.velocity.x * self.velocity.y + self.position.y;
        let b_b = -other.position.x / other.velocity.x * other.velocity.y + other.position.y;

        let x = (b_b - b_a) / (m_a - m_b);
        let y = m_a * x + b_a;

        Some(Position { x, y, z: 0.0 })
    }

    fn is_in_future(&self, pos: &Position) -> bool {
        if self.velocity.x < 0.0 && self.position.x < pos.x {
            return false;
        }
        if self.velocity.x > 0.0 && self.position.x > pos.x {
            return false;
        }
        if self.velocity.y < 0.0 && self.position.y < pos.y {
            return false;
        }
        if self.velocity.y > 0.0 && self.position.y > pos.y {
            return false;
        }
        true
    }

    fn equations_with(&self, other: &Hailstone) -> [Equation; 3] {
        let xy = Equation {
            a: [
                self.velocity.y - other.velocity.y,
                other.velocity.x - self.velocity.x,
                0.0,
                self.position.y - other.position.y,
                other.position.x - self.position.x,
                0.0,
            ],
            b: (self.position.x * self.velocity.y)
                - (self.position.y * self.velocity.x)
                - (other.position.x * other.velocity.y)
                + (other.position.y * other.velocity.x),
        };
        let yz = Equation {
            a: [
                0.0,
                self.velocity.z - other.velocity.z,
                other.velocity.y - self.velocity.y,
                0.0,
                self.position.z - other.position.z,
                other.position.y - self.position.y,
            ],
            b: (self.position.y * self.velocity.z)
                - (self.position.z * self.velocity.y)
                - (other.position.y * other.velocity.z)
                + (other.position.z * other.velocity.y),
        };
        let xz = Equation {
            a: [
                self.velocity.z - other.velocity.z,
                0.0,
                other.velocity.x - self.velocity.x,
                self.position.z - other.position.z,
                0.0,
                other.position.x - self.position.x,
            ],
            b: (self.position.x * self.velocity.z)
                - (self.position.z * self.velocity.x)
                - (other.position.x * other.velocity.z)
                + (other.position.z * other.velocity.x),
        };
        [xy, yz, xz]
    }
}

fn parse_hailstones(s: &str) -> Vec<Hailstone> {
    let mut hailstones = Vec::new();
    for line in s.lines() {
        let parts: Vec<&str> = line.split("@").collect();
        let positions: Vec<&str> = parts[0].split(",").collect();
        let velocities: Vec<&str> = parts[1].split(",").collect();
        let hailstone = Hailstone {
            position: Position {
                x: positions[0].trim().parse().unwrap(),
                y: positions[1].trim().parse().unwrap(),
                z: positions[2].trim().parse().unwrap(),
            },
            velocity: Velocity {
                x: velocities[0].trim().parse().unwrap(),
                y: velocities[1].trim().parse().unwrap(),
                z: velocities[2].trim().parse().unwrap(),
            },
        };
        hailstones.push(hailstone);
    }
    hailstones
}

fn part1(hailstones: &[Hailstone]) {
    let mut sum = 0;
    for (i, a) in hailstones[..hailstones.len() - 1].iter().enumerate() {
        for b in &hailstones[i + 1..] {
            if let Some(i) = a.xy_intersection_with(b)
                && i.is_in_area()
                && a.is_in_future(&i)
                && b.is_in_future(&i)
            {
                sum += 1;
            }
        }
    }
    println!("Intersections: {}", sum);
}

#[derive(Clone)]
struct Equation {
    // x, y, z, vx, vy, vz
    a: [f64; N],
    b: f64,
}

// Least Squares
// Solving: A * x = b, where A is M×6, x is 6×1, b is M×1
// (A^T × A) × x = A^T × b
// M is number of equations
fn solve_system(equations: Vec<Equation>) -> Option<[f64; N]> {
    // A^T * A (6x6)
    let mut ata = [[0.0; N]; N];
    for (i, row) in ata.iter_mut().enumerate() {
        for (j, val) in row.iter_mut().enumerate() {
            let mut sum = 0.0;
            for eq in &equations {
                sum += eq.a[i] * eq.a[j];
            }
            *val = sum;
        }
    }

    // A^T * b (6x1)
    let mut atb = [0.0; N];
    for (i, val) in atb.iter_mut().enumerate() {
        let mut sum = 0.0;
        for eq in &equations {
            sum += eq.a[i] * eq.b;
        }
        *val = sum;
    }

    // (A^T * A) | x  (6x7)
    let mut matrix = [[0.0; N + 1]; N];
    for (i, row) in ata.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            matrix[i][j] = *val;
        }
        matrix[i][N] = atb[i];
    }

    // forward elimination with partial pivoting
    for col in 0..N {
        // find pivot
        let mut max_row = col;
        for row in (col + 1)..N {
            if matrix[row][col].abs() > matrix[max_row][col].abs() {
                max_row = row;
            }
        }

        matrix.swap(col, max_row);

        if matrix[col][col].abs() < 1e-10 {
            return None;
        }

        // eliminate
        for row in (col + 1)..N {
            let factor = matrix[row][col] / matrix[col][col];
            #[allow(clippy::needless_range_loop)]
            for c in col..=N {
                matrix[row][c] -= factor * matrix[col][c];
            }
        }
    }

    // back substitution
    let mut solution = [0.0; 6];
    for i in (0..N).rev() {
        let mut sum = matrix[i][N];
        for j in (i + 1)..N {
            sum -= matrix[i][j] * solution[j];
        }
        solution[i] = sum / matrix[i][i];
    }

    Some(solution)
}

fn part2(hailstones: &[Hailstone]) {
    let mut equations = Vec::new();

    equations.extend(hailstones[0].equations_with(&hailstones[1]));
    equations.extend(hailstones[0].equations_with(&hailstones[2]));
    equations.extend(hailstones[0].equations_with(&hailstones[3]));

    if let Some(solution) = solve_system(equations) {
        let [px, py, pz, vx, vy, vz] = solution;
        println!("Rock position: ({}, {}, {})", px, py, pz);
        println!("Rock velocity: ({}, {}, {})", vx, vy, vz);
        println!("Answer: {}", px + py + pz);
    } else {
        println!("Failed to solve system");
    }
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let hailstones = parse_hailstones(&input);
    part1(&hailstones);
    part2(&hailstones);
}
