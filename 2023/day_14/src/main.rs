use std::collections::HashSet;
use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

#[derive(PartialEq, Clone, Hash, Eq)]
enum Tile {
    RoundedRock,
    SquareRock,
    Empty,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            'O' => Tile::RoundedRock,
            '#' => Tile::SquareRock,
            '.' => Tile::Empty,
            _ => panic!(),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Tile::RoundedRock => 'O',
            Tile::SquareRock => '#',
            Tile::Empty => '.',
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Platform {
    grid: Vec<Vec<Tile>>,
}

impl Platform {
    fn from_str(s: &str) -> Platform {
        let mut grid: Vec<Vec<Tile>> = Vec::new();
        for line in s.lines() {
            let row = line.trim().chars().map(Tile::from_char).collect();
            grid.push(row);
        }
        Platform { grid }
    }

    fn print(&self) {
        println!();
        for row in &self.grid {
            for tile in row {
                print!("{}", tile.to_char());
            }
            println!();
        }
        println!();
    }

    fn spin_cycle(&mut self) {
        self.roll_north();
        self.roll_west();
        self.roll_south();
        self.roll_east();
    }

    fn roll_north(&mut self) {
        let n_columns = self.grid[0].len();
        let n_rows = self.grid.len();

        for x in 0..n_columns {
            for y in 1..n_rows {
                if self.grid[y][x] != Tile::RoundedRock {
                    continue;
                }

                let mut curr = y;
                while curr > 0 && self.grid[curr - 1][x] == Tile::Empty {
                    self.grid[curr - 1][x] = Tile::RoundedRock;
                    self.grid[curr][x] = Tile::Empty;
                    curr -= 1;
                }
            }
        }
    }

    fn roll_west(&mut self) {
        let n_columns = self.grid[0].len();
        let n_rows = self.grid.len();

        for y in 0..n_rows {
            for x in 1..n_columns {
                if self.grid[y][x] != Tile::RoundedRock {
                    continue;
                }

                let mut curr = x;
                while curr > 0 && self.grid[y][curr - 1] == Tile::Empty {
                    self.grid[y][curr - 1] = Tile::RoundedRock;
                    self.grid[y][curr] = Tile::Empty;
                    curr -= 1;
                }
            }
        }
    }

    fn roll_south(&mut self) {
        let n_columns = self.grid[0].len();
        let n_rows = self.grid.len();

        for x in 0..n_columns {
            for y in (0..(n_rows - 1)).rev() {
                if self.grid[y][x] != Tile::RoundedRock {
                    continue;
                }

                let mut curr = y;
                while curr < (n_rows - 1) && self.grid[curr + 1][x] == Tile::Empty {
                    self.grid[curr + 1][x] = Tile::RoundedRock;
                    self.grid[curr][x] = Tile::Empty;
                    curr += 1;
                }
            }
        }
    }

    fn roll_east(&mut self) {
        let n_columns = self.grid[0].len();
        let n_rows = self.grid.len();

        for y in 0..n_rows {
            for x in (0..(n_columns - 1)).rev() {
                if self.grid[y][x] != Tile::RoundedRock {
                    continue;
                }

                let mut curr = x;
                while curr < (n_columns - 1) && self.grid[y][curr + 1] == Tile::Empty {
                    self.grid[y][curr + 1] = Tile::RoundedRock;
                    self.grid[y][curr] = Tile::Empty;
                    curr += 1;
                }
            }
        }
    }

    fn weigh(&self) -> usize {
        let n_rows = self.grid.len();
        let mut sum = 0;
        for (i, row) in self.grid.iter().enumerate() {
            let n_rounded = row.iter().filter(|&x| *x == Tile::RoundedRock).count();
            sum += n_rounded * (n_rows - i);
        }
        sum
    }
}

fn part1(mut platform: Platform) {
    platform.roll_north();

    println!();
    platform.print();

    let weight = platform.weigh();
    println!("Total weight after rolling north is: {}", weight);
}

fn part2(mut platform: Platform) {
    // The process of spinning stabilizes early into a cyclic pattern, so what we can do
    // is find the length of the cycles, and then just predict the state of the platform
    // at exactly 1B iterations.

    // run the cycle for 500 iterations, should be enough to make it stabilize into
    // a cyclic pattern
    let warmup = 500;
    for _ in 0..(warmup - 10) {
        platform.spin_cycle();
    }
    // This is a bit hacky, but to find the cyclic pattern, let's keep track of the last 10
    // weights and find the moment at which they start repeating.
    let mut last_ten: Vec<usize> = Vec::new();
    for _ in 0..10 {
        platform.spin_cycle();
        last_ten.push(platform.weigh());
    }

    let mut set: HashSet<Vec<usize>> = HashSet::new();
    let mut cycle_len = 0;
    while !set.contains(&last_ten) {
        set.insert(last_ten.clone());
        platform.spin_cycle();
        let w = platform.weigh();
        last_ten.remove(0);
        last_ten.push(w);
        cycle_len += 1;
    }

    // Finally, extrapolate the state at 1B iterations
    let remaining_cycles = (1_000_000_000 - warmup) % cycle_len;

    for _ in 0..remaining_cycles {
        platform.spin_cycle();
    }

    println!("Total weight after 1B spin cycles is: {}", platform.weigh());
}

fn main() {
    let s = read_to_string(INPUT_FILE).unwrap();
    let platform = Platform::from_str(&s);
    platform.print();
    part1(platform.clone());
    part2(platform);
}
