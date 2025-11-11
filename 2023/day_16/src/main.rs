use std::collections::HashSet;
use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

#[derive(Clone)]
enum Device {
    MirrorForward,
    MirrorBackward,
    SplitterVertical,
    SplitterHorizontal,
    Empty,
}

impl Device {
    fn to_char(&self) -> char {
        match self {
            Device::MirrorForward => '/',
            Device::MirrorBackward => '\\',
            Device::SplitterVertical => '|',
            Device::SplitterHorizontal => '-',
            Device::Empty => '.',
        }
    }
}

#[derive(Clone)]
struct Tile {
    device: Device,
    energized: bool,
}

impl Tile {
    fn init_from_char(c: char) -> Tile {
        let device = match c {
            '/' => Device::MirrorForward,
            '\\' => Device::MirrorBackward,
            '|' => Device::SplitterVertical,
            '-' => Device::SplitterHorizontal,
            '.' => Device::Empty,
            _ => unreachable!(),
        };
        Tile {
            device,
            energized: false,
        }
    }
}

#[derive(Clone)]
struct Grid {
    data: Vec<Vec<Tile>>,
}

impl Grid {
    fn init_from_str(s: &str) -> Grid {
        let mut rows = Vec::new();
        for line in s.trim().lines() {
            let tiles: Vec<Tile> = line.chars().map(Tile::init_from_char).collect();
            rows.push(tiles);
        }
        Grid { data: rows }
    }

    fn print(&self) {
        for row in &self.data {
            for tile in row {
                print!("{}", tile.device.to_char());
            }
            println!();
        }
    }

    fn print_energy(&self) {
        for row in &self.data {
            for tile in row {
                match tile.energized {
                    true => print!("#"),
                    false => print!(" "),
                }
            }
            println!();
        }
    }

    fn count_energized(&self) -> usize {
        let mut sum = 0;
        for row in &self.data {
            for tile in row {
                if tile.energized {
                    sum += 1;
                }
            }
        }
        sum
    }

    fn get(&self, coord: &Coord) -> &Tile {
        &self.data[coord.y][coord.x]
    }

    fn mark_energized(&mut self, coord: &Coord) {
        self.data[coord.y][coord.x].energized = true;
    }

    fn energize_from(&mut self, initial_beam: Beam) {
        let grid_height = self.data.len();
        let grid_width = self.data[0].len();

        let mut beams = vec![initial_beam];
        let mut visited: HashSet<Beam> = HashSet::new();

        while !beams.is_empty() {
            let beam = beams.remove(0);
            self.mark_energized(&beam.coord);
            let reflecting_beams = beam.reflect_against(&self.get(&beam.coord).device);

            for beam in reflecting_beams {
                let next: Coord = match beam.direction {
                    Direction::North => {
                        if beam.coord.y == 0 {
                            continue;
                        }
                        Coord {
                            x: beam.coord.x,
                            y: beam.coord.y - 1,
                        }
                    }

                    Direction::East => {
                        if beam.coord.x == grid_width - 1 {
                            continue;
                        }
                        Coord {
                            x: beam.coord.x + 1,
                            y: beam.coord.y,
                        }
                    }

                    Direction::South => {
                        if beam.coord.y == grid_height - 1 {
                            continue;
                        }
                        Coord {
                            x: beam.coord.x,
                            y: beam.coord.y + 1,
                        }
                    }

                    Direction::West => {
                        if beam.coord.x == 0 {
                            continue;
                        }
                        Coord {
                            x: beam.coord.x - 1,
                            y: beam.coord.y,
                        }
                    }
                };

                let moved = Beam {
                    coord: next,
                    direction: beam.direction.clone(),
                };

                if !visited.contains(&moved) {
                    visited.insert(moved.clone());
                    beams.push(moved);
                }
            }
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Beam {
    coord: Coord,
    direction: Direction,
}

impl Beam {
    fn reflect_against(&self, device: &Device) -> Vec<Beam> {
        match self.direction {
            Direction::North => self.reflect_north(device),
            Direction::East => self.reflect_east(device),
            Direction::South => self.reflect_south(device),
            Direction::West => self.reflect_west(device),
        }
    }

    fn reflect_north(&self, device: &Device) -> Vec<Beam> {
        let mut beams = Vec::new();
        match device {
            Device::MirrorForward => beams.push(Beam {
                coord: self.coord.clone(),
                direction: Direction::East,
            }),
            Device::MirrorBackward => beams.push(Beam {
                coord: self.coord.clone(),
                direction: Direction::West,
            }),
            Device::SplitterHorizontal => {
                beams.push(Beam {
                    coord: self.coord.clone(),
                    direction: Direction::East,
                });
                beams.push(Beam {
                    coord: self.coord.clone(),
                    direction: Direction::West,
                });
            }
            _ => beams.push(Beam {
                coord: self.coord.clone(),
                direction: Direction::North,
            }),
        }
        beams
    }

    fn reflect_east(&self, device: &Device) -> Vec<Beam> {
        let mut beams = Vec::new();
        match device {
            Device::MirrorForward => beams.push(Beam {
                coord: self.coord.clone(),
                direction: Direction::North,
            }),
            Device::MirrorBackward => beams.push(Beam {
                coord: self.coord.clone(),
                direction: Direction::South,
            }),
            Device::SplitterVertical => {
                beams.push(Beam {
                    coord: self.coord.clone(),
                    direction: Direction::North,
                });
                beams.push(Beam {
                    coord: self.coord.clone(),
                    direction: Direction::South,
                });
            }
            _ => beams.push(Beam {
                coord: self.coord.clone(),
                direction: Direction::East,
            }),
        }
        beams
    }

    fn reflect_south(&self, device: &Device) -> Vec<Beam> {
        let mut beams = Vec::new();
        match device {
            Device::MirrorForward => beams.push(Beam {
                coord: self.coord.clone(),
                direction: Direction::West,
            }),
            Device::MirrorBackward => beams.push(Beam {
                coord: self.coord.clone(),
                direction: Direction::East,
            }),
            Device::SplitterHorizontal => {
                beams.push(Beam {
                    coord: self.coord.clone(),
                    direction: Direction::West,
                });
                beams.push(Beam {
                    coord: self.coord.clone(),
                    direction: Direction::East,
                });
            }
            _ => beams.push(Beam {
                coord: self.coord.clone(),
                direction: Direction::South,
            }),
        }
        beams
    }

    fn reflect_west(&self, device: &Device) -> Vec<Beam> {
        let mut beams = Vec::new();
        match device {
            Device::MirrorForward => beams.push(Beam {
                coord: self.coord.clone(),
                direction: Direction::South,
            }),
            Device::MirrorBackward => beams.push(Beam {
                coord: self.coord.clone(),
                direction: Direction::North,
            }),
            Device::SplitterVertical => {
                beams.push(Beam {
                    coord: self.coord.clone(),
                    direction: Direction::South,
                });
                beams.push(Beam {
                    coord: self.coord.clone(),
                    direction: Direction::North,
                });
            }
            _ => beams.push(Beam {
                coord: self.coord.clone(),
                direction: Direction::West,
            }),
        }
        beams
    }
}

fn part1(mut grid: Grid) {
    let initial_beam = Beam {
        coord: Coord { x: 0, y: 0 },
        direction: Direction::East,
    };

    grid.energize_from(initial_beam);
    grid.print_energy();
    println!("There are {} energized tiles", grid.count_energized());
}

fn part2(grid: Grid) {
    let grid_height = grid.data.len();
    let grid_width = grid.data[0].len();

    let mut initial_beams = Vec::new();
    for y in 0..grid_height {
        initial_beams.push(Beam {
            coord: Coord { x: 0, y },
            direction: Direction::East,
        });
        initial_beams.push(Beam {
            coord: Coord {
                x: grid_width - 1,
                y,
            },
            direction: Direction::West,
        });
    }
    for x in 0..grid_width {
        initial_beams.push(Beam {
            coord: Coord { x, y: 0 },
            direction: Direction::South,
        });
        initial_beams.push(Beam {
            coord: Coord {
                x,
                y: grid_height - 1,
            },
            direction: Direction::North,
        });
    }

    let mut max_energized = 0;
    for initial_beam in initial_beams {
        let mut cloned = grid.clone();
        cloned.energize_from(initial_beam);
        let energized_tiles = cloned.count_energized();
        if energized_tiles > max_energized {
            max_energized = energized_tiles;
        }
    }
    println!("Best configuration has {} energized tiles", max_energized);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let grid = Grid::init_from_str(&input);

    grid.print();

    part1(grid.clone());
    part2(grid);
}
