use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::{SystemTime, UNIX_EPOCH};

const INPUT_FILE: &str = "input.txt";

type Component = u16;

#[derive(Clone, Debug)]
struct Wire {
    a: Component,
    b: Component,
}

fn get_wire_from_name(map: &mut HashMap<String, Component>, name: &str) -> Component {
    match map.get(name) {
        Some(w) => *w,
        None => {
            let next = map.len() as Component;
            map.insert(name.to_string(), next);
            next
        }
    }
}

fn parse_connections(input: &str) -> Vec<Wire> {
    let mut map = HashMap::new();
    let mut connections = Vec::new();
    for line in input.lines() {
        let semicolon = line.find(":").unwrap();
        let first_name = line[..semicolon].trim().to_string();
        let first = get_wire_from_name(&mut map, &first_name);

        let parts: Vec<String> = line[semicolon + 1..]
            .split_whitespace()
            .map(|p| p.to_string())
            .collect();

        for part in parts {
            let wire = get_wire_from_name(&mut map, &part);
            connections.push(Wire { a: first, b: wire });
        }
    }
    connections
}

fn rand_idx(max: usize) -> usize {
    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    t.as_nanos() as usize % max
}

#[derive(Clone)]
struct Graph {
    wires: Vec<Wire>,
    nodes: usize,
    counts: HashMap<Component, usize>,
}

impl Graph {
    fn from_wires(wires: &[Wire]) -> Graph {
        let mut counts = HashMap::new();
        for wire in wires {
            counts.insert(wire.a, 1);
            counts.insert(wire.b, 1);
        }
        Graph {
            wires: wires.to_vec(),
            nodes: counts.len(),
            counts,
        }
    }

    fn kargers_once(&mut self) {
        while self.nodes > 2 {
            let idx = rand_idx(self.wires.len());
            let wire = self.wires.remove(idx);

            let keep = wire.a;
            let remove = wire.b;

            let removed_count = self.counts.remove(&remove).unwrap();
            *self.counts.get_mut(&keep).unwrap() += removed_count;

            for wire in self.wires.iter_mut() {
                if wire.a == remove {
                    wire.a = keep;
                }
                if wire.b == remove {
                    wire.b = keep;
                }
            }

            self.wires.retain(|w| w.a != w.b);
            self.nodes -= 1;
        }
    }
}

fn part1(connections: Vec<Wire>) {
    let graph = Graph::from_wires(&connections);

    loop {
        let mut cloned = graph.clone();
        cloned.kargers_once();

        if cloned.wires.len() == 3 {
            let a = cloned.counts.get(&cloned.wires[0].a).unwrap();
            let b = cloned.counts.get(&cloned.wires[0].b).unwrap();
            println!("Remaining group sizes: {} & {}", a, b);
            println!("Multiplied: {}", a * b);
            break;
        }
    }
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let connections = parse_connections(&input);
    part1(connections);
}
