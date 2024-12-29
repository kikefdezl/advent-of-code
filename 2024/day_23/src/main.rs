use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

const INPUT_FILE: &str = "input.txt";

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    let mut connections = Vec::new();
    for line in input.trim().lines() {
        connections.push((&line[..2], &line[3..]));
    }
    connections
}

fn build_graph(connections: &Vec<(&str, &str)>) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for &(from, to) in connections {
        graph
            .entry(from.to_string())
            .or_insert_with(Vec::new)
            .push(to.to_string());

        graph
            .entry(to.to_string())
            .or_insert_with(Vec::new)
            .push(from.to_string());
    }
    graph
}

fn part1(graph: &HashMap<String, Vec<String>>) {
    let mut triangles: HashSet<Vec<String>> = HashSet::new();
    for (current, neighbors) in graph.iter() {
        for neighbor in neighbors {
            let next_neighbors = graph.get(neighbor).unwrap();
            for next_neighbor in next_neighbors {
                if neighbors.contains(next_neighbor)
                    && (current.starts_with("t")
                        || neighbor.starts_with("t")
                        || next_neighbor.starts_with("t"))
                {
                    let mut triangle = vec![
                        current.to_string(),
                        neighbor.to_string(),
                        next_neighbor.to_string(),
                    ];
                    triangle.sort();
                    triangles.insert(triangle);
                }
            }
        }
    }
    println!(
        "There are {} sets of 3-connected computers with at least one 't'.",
        triangles.len()
    );
}

fn largest_fully_connected(graph: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut queue: VecDeque<Vec<&str>> = VecDeque::new();
    let nodes: Vec<Vec<&str>> = graph.keys().map(|x| vec![x.as_str()]).collect();
    queue.extend(nodes);

    let mut longest = Vec::new();

    let mut scanned: HashSet<Vec<&str>> = HashSet::new();
    while let Some(network) = queue.pop_front() {
        let last = network.last().unwrap();
        for neighbor in graph.get(*last).unwrap() {
            if !network
                .iter()
                .all(|x| graph.get(*x).unwrap().contains(neighbor))
            {
                continue;
            }

            let mut new_network = network.clone();
            new_network.push(neighbor);
            new_network.sort();
            if scanned.contains(&new_network) {
                continue;
            }
            if new_network.len() > longest.len() {
                longest = new_network.iter().map(|x| x.to_string()).collect();
            }
            queue.push_back(new_network.clone());
            scanned.insert(new_network);
        }
    }
    longest
}

fn part2(graph: &HashMap<String, Vec<String>>) {
    let mut network = largest_fully_connected(&graph);
    network.sort();
    println!("Password: {:?}", network.join(","));
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let connections = parse_input(&input);
    let graph = build_graph(&connections);

    part1(&graph);
    part2(&graph);
}
