use std::collections::HashMap;
use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";
const START: [char; 3] = ['A', 'A', 'A'];
const END: [char; 3] = ['Z', 'Z', 'Z'];

type Node = [char; 3];

fn parse_input(input: &str) -> (Vec<char>, HashMap<Node, (Node, Node)>) {
    let lines: Vec<&str> = input.lines().collect();
    let instructions: Vec<char> = lines[0].chars().collect();

    let mut graph: HashMap<Node, (Node, Node)> = HashMap::new();
    for line in &lines[2..] {
        let chars: Vec<char> = line.chars().collect();
        let start = [chars[0], chars[1], chars[2]];
        let left = [chars[7], chars[8], chars[9]];
        let right = [chars[12], chars[13], chars[14]];
        graph.insert(start, (left, right));
    }

    (instructions, graph)
}

fn part_one(instructions: &Vec<char>, graph: &HashMap<Node, (Node, Node)>) {
    let total_instructions = instructions.len();
    let mut instruction_idx = 0;
    let mut steps = 0;
    let mut current = START;
    while current != END {
        let instruction = instructions[instruction_idx];
        current = match instruction {
            'L' => graph.get(&current).unwrap().0,
            'R' => graph.get(&current).unwrap().1,
            _ => unreachable!(),
        };
        steps += 1;

        instruction_idx += 1;
        if instruction_idx >= total_instructions {
            instruction_idx = 0;
        }
    }

    println!("Need {} steps to exit", steps);
}

fn part_two(instructions: &Vec<char>, graph: &HashMap<Node, (Node, Node)>) {
    let starting_nodes: Vec<Node> = graph.keys().cloned().filter(|x| x[2] == 'A').collect();

    let total_instructions = instructions.len();

    let mut min_steps_per_node = Vec::new();
    for node in &starting_nodes {
        let mut instruction_idx = 0;
        let mut steps = 0;
        let mut current = node.clone();
        while current[2] != 'Z' {
            let instruction = instructions[instruction_idx];
            current = match instruction {
                'L' => graph.get(&current).unwrap().0,
                'R' => graph.get(&current).unwrap().1,
                _ => unreachable!(),
            };
            steps += 1;

            instruction_idx += 1;
            if instruction_idx >= total_instructions {
                instruction_idx = 0;
            }
        }
        min_steps_per_node.push(steps);
    }
    let total_steps = lcm(&min_steps_per_node);
    println!("Need {} steps to exit as a ghost", total_steps);
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let (instructions, graph) = parse_input(&input);

    part_one(&instructions, &graph);
    part_two(&instructions, &graph);
}
