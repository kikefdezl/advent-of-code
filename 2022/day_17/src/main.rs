use std::collections::hash_map::Entry;
use std::{collections::HashMap, fs::read_to_string};

const INPUT_FILE: &str = "input.txt";
const CHAMBER_WIDTH: usize = 7;
const SPAWN_X: usize = 2;
const SPAWN_Y_DELTA: usize = 3;
const TOWER_HEIGHT_BUFFER: usize = 7;

#[derive(Clone, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn offset(&self, other: &Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Rock {
    pos: Coord,
    blocks: Vec<Coord>,
}

const HBAR: [Coord; 4] = [
    Coord { x: 0, y: 0 },
    Coord { x: 1, y: 0 },
    Coord { x: 2, y: 0 },
    Coord { x: 3, y: 0 },
];

const PLUS: [Coord; 5] = [
    Coord { x: 1, y: 0 },
    Coord { x: 0, y: 1 },
    Coord { x: 1, y: 1 },
    Coord { x: 2, y: 1 },
    Coord { x: 1, y: 2 },
];
const LBAR: [Coord; 5] = [
    Coord { x: 0, y: 0 },
    Coord { x: 1, y: 0 },
    Coord { x: 2, y: 0 },
    Coord { x: 2, y: 1 },
    Coord { x: 2, y: 2 },
];
const VBAR: [Coord; 4] = [
    Coord { x: 0, y: 0 },
    Coord { x: 0, y: 1 },
    Coord { x: 0, y: 2 },
    Coord { x: 0, y: 3 },
];
const BOX: [Coord; 4] = [
    Coord { x: 0, y: 0 },
    Coord { x: 1, y: 0 },
    Coord { x: 0, y: 1 },
    Coord { x: 1, y: 1 },
];

impl Rock {
    fn spawn(y: usize, rock_idx: usize) -> Rock {
        let blocks = if rock_idx == 0 {
            HBAR.to_vec()
        } else if rock_idx == 1 {
            PLUS.to_vec()
        } else if rock_idx == 2 {
            LBAR.to_vec()
        } else if rock_idx == 3 {
            VBAR.to_vec()
        } else {
            BOX.to_vec()
        };
        Rock {
            pos: Coord { x: SPAWN_X, y },
            blocks,
        }
    }

    fn can_move_down(&self, chamber: &Chamber) -> bool {
        for block in &self.blocks {
            let coord = self.pos.offset(block);
            if coord.y == 0 {
                return false;
            }
            if chamber.rows[coord.y - 1].blocks[coord.x] == Block::Rock {
                return false;
            }
        }
        true
    }

    fn move_down(&mut self) {
        self.pos.y -= 1;
    }

    fn can_move_left(&self, chamber: &Chamber) -> bool {
        for block in &self.blocks {
            let coord = self.pos.offset(block);
            if coord.x == 0 {
                return false;
            }
            if chamber.rows[coord.y].blocks[coord.x - 1] == Block::Rock {
                return false;
            }
        }
        true
    }

    fn move_left(&mut self) {
        self.pos.x -= 1;
    }

    fn can_move_right(&self, chamber: &Chamber) -> bool {
        for block in &self.blocks {
            let coord = self.pos.offset(block);
            if coord.x >= CHAMBER_WIDTH - 1 {
                return false;
            }
            if chamber.rows[coord.y].blocks[coord.x + 1] == Block::Rock {
                return false;
            }
        }
        true
    }

    fn move_right(&mut self) {
        self.pos.x += 1;
    }
}

enum Stream {
    Left,
    Right,
}

fn parse_streams(s: &str) -> Vec<Stream> {
    let mut streams = Vec::new();
    for c in s.chars() {
        if c == '<' {
            streams.push(Stream::Left)
        } else if c == '>' {
            streams.push(Stream::Right)
        }
    }
    streams
}

#[derive(Debug)]
struct Chamber {
    rows: Vec<Row>,
    height: usize,
}

impl Chamber {
    fn new() -> Chamber {
        Chamber {
            rows: vec![],
            height: 0,
        }
    }

    fn add_row(&mut self) {
        self.rows.push(Row::new());
    }

    fn empty_top_rows(&self) -> usize {
        let mut count = 0;
        for i in ((self.height().saturating_sub(TOWER_HEIGHT_BUFFER))..self.height()).rev() {
            if !self.rows[i].has_rock() {
                count += 1;
            }
        }
        count
    }

    fn buffer(&mut self) {
        if self.height() < TOWER_HEIGHT_BUFFER {
            for _ in 0..(TOWER_HEIGHT_BUFFER - self.height()) {
                self.add_row();
            }
        }
        for _ in 0..(TOWER_HEIGHT_BUFFER - self.empty_top_rows()) {
            self.add_row()
        }
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn settle(&mut self, rock: &Rock) {
        for block in &rock.blocks {
            let coord = rock.pos.offset(block);
            self.rows[coord.y].blocks[coord.x] = Block::Rock;

            if coord.y + 1 > self.height {
                self.height = coord.y + 1;
            }
        }
    }

    fn simulate(
        &mut self,
        streams: &[Stream],
        iters: usize,
        start_stream_idx: usize,
        start_rock_idx: usize,
    ) {
        let mut stream_idx = start_stream_idx;

        for i in 0..iters {
            self.buffer();

            let rock_idx = (start_rock_idx + i) % 5;
            let mut rock = Rock::spawn(self.height + SPAWN_Y_DELTA, rock_idx);
            loop {
                // stream of air
                let stream = &streams[stream_idx];
                match stream {
                    Stream::Left => {
                        if rock.can_move_left(self) {
                            rock.move_left();
                        };
                    }
                    Stream::Right => {
                        if rock.can_move_right(self) {
                            rock.move_right();
                        };
                    }
                }
                stream_idx += 1;
                if stream_idx >= streams.len() {
                    stream_idx = 0;
                }

                // fall
                if !rock.can_move_down(self) {
                    break;
                }
                rock.move_down();
            }
            self.settle(&rock);
        }
    }

    fn surface_profile(&self) -> [usize; CHAMBER_WIDTH] {
        let mut profile = [0usize; CHAMBER_WIDTH];
        for (col, height) in profile.iter_mut().enumerate() {
            for row in (0..self.height).rev() {
                if self.rows[row].blocks[col] == Block::Rock {
                    *height = self.height - row;
                    break;
                }
            }
        }
        profile
    }
}

#[derive(Clone, Debug)]
struct Row {
    blocks: Vec<Block>,
}

impl Row {
    fn new() -> Row {
        Row {
            blocks: vec![Block::Air; CHAMBER_WIDTH],
        }
    }

    fn has_rock(&self) -> bool {
        for b in &self.blocks {
            if *b == Block::Rock {
                return true;
            }
        }
        false
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Block {
    Rock,
    Air,
}

struct LoopParams {
    // How many iterations a loop has
    iters: usize,
    // The height of every loop
    height: usize,
    // The rock idx at which the loop starts
    rock_idx: usize,
    // The stream idx at which the loop starts
    stream_idx: usize,
}

fn find_loop_params(streams: &[Stream]) -> LoopParams {
    let mut chamber = Chamber::new();
    let mut stream_idx = 0;

    let mut visited = HashMap::new();

    let mut i = 0;
    loop {
        chamber.buffer();

        let rock_idx = i % 5;
        let mut rock = Rock::spawn(chamber.height + SPAWN_Y_DELTA, rock_idx);
        loop {
            // stream of air
            let stream = &streams[stream_idx];
            match stream {
                Stream::Left => {
                    if rock.can_move_left(&chamber) {
                        rock.move_left();
                    };
                }
                Stream::Right => {
                    if rock.can_move_right(&chamber) {
                        rock.move_right();
                    };
                }
            }
            stream_idx += 1;
            if stream_idx >= streams.len() {
                stream_idx = 0;
            }

            // fall
            if !rock.can_move_down(&chamber) {
                break;
            }
            rock.move_down();
        }

        chamber.settle(&rock);
        i += 1;

        let profile = chamber.surface_profile();
        match visited.entry((stream_idx, rock_idx % 5, profile)) {
            Entry::Vacant(e) => {
                e.insert((i, stream_idx, chamber.height));
            }
            Entry::Occupied(o) => {
                let entry = o.get();
                return LoopParams {
                    iters: i - entry.0,
                    height: chamber.height - entry.2,
                    rock_idx: entry.0,
                    stream_idx: entry.1,
                };
            }
        }
    }
}

fn part1(streams: &[Stream]) {
    let rocks = 2022;
    let mut chamber = Chamber::new();
    chamber.simulate(streams, rocks, 0, 0);
    println!("Height after {} rocks: {}", rocks, chamber.height);
}

fn part2(streams: &[Stream]) {
    let rocks = 1_000_000_000_000;
    let mut chamber = Chamber::new();

    let loop_ = find_loop_params(streams);

    chamber.simulate(streams, loop_.rock_idx, 0, 0);
    let rocks_after_prefix = rocks - loop_.rock_idx;
    let total_loops = rocks_after_prefix / loop_.iters;
    let remaining = rocks_after_prefix % total_loops;
    chamber.simulate(streams, remaining, loop_.stream_idx, loop_.rock_idx);

    let total_height = chamber.height + (total_loops * loop_.height);

    println!("Height after {} rocks: {}", rocks, total_height);
}

fn main() {
    let streams_input = read_to_string(INPUT_FILE).unwrap();
    let streams = parse_streams(&streams_input);

    part1(&streams);
    part2(&streams);
}
