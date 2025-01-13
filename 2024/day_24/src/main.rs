// Some of the ugliest code I've written in my life, but
// it gets the job done and I don't want to revisit this
// MADNESS.

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum GateOp {
    AND,
    OR,
    XOR,
}

impl GateOp {
    fn from_str(str: &str) -> GateOp {
        match str {
            "AND" => GateOp::AND,
            "OR" => GateOp::OR,
            "XOR" => GateOp::XOR,
            _ => panic!(),
        }
    }

    fn apply(&self, a: bool, b: bool) -> bool {
        match &self {
            GateOp::AND => a && b,
            GateOp::OR => a || b,
            GateOp::XOR => a ^ b,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Gate {
    inputs: (String, String),
    output: String,
    op: GateOp,
}

impl Gate {
    fn print(&self) {
        print!(
            "{} {:?} {} -> {}",
            self.inputs.0, self.op, self.inputs.1, self.output
        );
    }
}

fn bool_from_char(c: char) -> bool {
    match c {
        '0' => false,
        '1' => true,
        _ => panic!(),
    }
}

fn parse_input(input: &str) -> (HashMap<String, Option<bool>>, Vec<Gate>) {
    let lines: Vec<&str> = input.lines().collect();

    let mut wires: HashMap<String, Option<bool>> = HashMap::new();
    let mut gates = Vec::new();

    let mut i = 0;
    while lines[i].trim() != "" {
        wires.insert(
            lines[i][..3].to_string(),
            Some(bool_from_char(lines[i].chars().nth(5).unwrap())),
        );
        i += 1;
    }

    for line in lines.iter().skip(i + 1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let input_a = parts[0].to_string();
        let op = GateOp::from_str(&parts[1]);
        let input_b = parts[2].to_string();
        let output = parts[4].to_string();

        gates.push(Gate {
            inputs: (input_a.clone(), input_b.clone()),
            op,
            output: output.clone(),
        });

        for wire in [input_a, input_b, output] {
            if !wires.contains_key(&wire) {
                wires.insert(wire.to_string(), None);
            }
        }
    }

    (wires, gates)
}

fn wires_to_decimal(wires: &HashMap<String, Option<bool>>, prefix: &str) -> u64 {
    let mut decimal = 0;
    let num_prefixed_wires = wires.keys().filter(|x| x.starts_with(prefix)).count();
    for i in 0..num_prefixed_wires {
        let wire_name = format!("{}{:02}", prefix, i);
        let wire_value = wires.get(&wire_name).unwrap().unwrap();
        if wire_value {
            decimal += 2_u64.pow(i as u32);
        }
    }
    decimal
}

fn compute(wires: &mut HashMap<String, Option<bool>>, gates: &Vec<Gate>) -> Result<(), String> {
    let mut queue: VecDeque<Gate> = VecDeque::new();
    for gate in gates {
        queue.push_back(gate.clone());
    }

    let mut fails = 0;
    while let Some(g) = queue.pop_front() {
        let val_a = wires.get(&g.inputs.0).unwrap();
        let val_b = wires.get(&g.inputs.1).unwrap();
        if val_a.is_none() || val_b.is_none() {
            queue.push_back(g);
            fails += 1;
            if fails >= queue.len() {
                return Err(String::from("Couldn't compute wire values"));
            }
            continue;
        }
        fails = 0;

        let val_out = g.op.apply(val_a.unwrap(), val_b.unwrap());
        wires.insert(g.output, Some(val_out));
    }
    Ok(())
}

fn part1(wires: &mut HashMap<String, Option<bool>>, gates: &Vec<Gate>) {
    compute(wires, &gates).unwrap();
    let decimal = wires_to_decimal(&wires, "z");
    println!("Output in decimal: {}", decimal);
}

fn build_input_map(wires: &Vec<String>, gates: &Vec<Gate>) -> HashMap<String, Vec<Gate>> {
    let mut map = HashMap::new();
    for wire in wires {
        let mut gates_in = Vec::new();
        for gate in gates {
            if &gate.inputs.0 == wire || &gate.inputs.1 == wire {
                gates_in.push(gate.clone());
            }
        }
        map.insert(wire.to_string(), gates_in);
    }
    map
}

fn find_nth_bit_adder(n: usize, input_map: &HashMap<String, Vec<Gate>>) {
    let x_name = format!("x{:02}", n);
    let y_name = format!("y{:02}", n);

    let mut gates: HashSet<Gate> = HashSet::new();
    for gate in input_map.get(&x_name).unwrap() {
        gates.insert(gate.clone());
    }
    for gate in input_map.get(&y_name).unwrap() {
        gates.insert(gate.clone());
    }
    let mut next_gates: HashSet<Gate> = HashSet::new();
    for gate in &gates {
        if !gate.output.starts_with("z") {
            for next_gate in input_map.get(&gate.output).unwrap() {
                next_gates.insert(next_gate.clone());
            }
        }
    }
    gates.extend(next_gates);

    for gate in gates {
        gate.print();
        println!(" is part of bit {}", n);
    }
}

fn is_xyz(wire: &str) -> bool {
    wire.starts_with("x") || wire.starts_with("y") || wire.starts_with("z")
}

fn or_gates_no_xyz(gates: &Vec<Gate>) -> Vec<String> {
    let mut bad_wires = Vec::new();
    for gate in gates {
        if gate.op != GateOp::OR {
            continue;
        }
        if is_xyz(&gate.inputs.0) {
            bad_wires.push(gate.inputs.0.clone());
        }
        if is_xyz(&gate.inputs.1) {
            bad_wires.push(gate.inputs.1.clone());
        }
        if is_xyz(&gate.output) {
            if gate.output != "z45" {
                bad_wires.push(gate.output.clone());
            }
        }
    }
    bad_wires
}

fn and_gates_no_xyz_output(gates: &Vec<Gate>) -> Vec<String> {
    let mut bad_wires = Vec::new();
    for gate in gates {
        if gate.op != GateOp::AND {
            continue;
        }
        if is_xyz(&gate.output) {
            bad_wires.push(gate.output.clone());
        }
    }
    bad_wires
}

fn and_xor_gates_both_xyz_or_none(gates: &Vec<Gate>) -> Vec<String> {
    let mut bad_wires = Vec::new();
    for gate in gates {
        if ![GateOp::AND, GateOp::XOR].contains(&gate.op) {
            continue;
        }
        if (is_xyz(&gate.inputs.0) && !is_xyz(&gate.inputs.1))
            || (!is_xyz(&gate.inputs.0) && is_xyz(&gate.inputs.1))
        {
            bad_wires.push(gate.inputs.0.clone());
            bad_wires.push(gate.inputs.1.clone());
        }
    }
    bad_wires
}

fn and_output_is_or_input(
    gates: &Vec<Gate>,
    input_map: &HashMap<String, Vec<Gate>>,
) -> Vec<String> {
    let mut bad_wires = Vec::new();
    for gate in gates {
        if gate.op != GateOp::AND
            // the output of the 0th bit doesn't fulfill this condition
            || gate.inputs == (String::from("x00"), String::from("y00"))
            || gate.inputs == (String::from("y00"), String::from("x00"))
        {
            continue;
        }
        let next_gates = input_map.get(&gate.output).unwrap();
        if next_gates.len() != 1 {
            bad_wires.push(gate.output.clone());
            continue;
        }
        if next_gates[0].op != GateOp::OR {
            bad_wires.push(gate.output.clone());
            continue;
        }
    }
    bad_wires
}

fn or_output_goes_in_one_and_one_xor(
    gates: &Vec<Gate>,
    input_map: &HashMap<String, Vec<Gate>>,
) -> Vec<String> {
    let mut bad_wires = Vec::new();
    for gate in gates {
        if gate.op != GateOp::OR || gate.output == "z45" {
            continue;
        }
        let next_gates = input_map.get(&gate.output).unwrap();
        if next_gates.len() != 2 {
            bad_wires.push(gate.output.clone());
            continue;
        }
        if !((next_gates[0].op == GateOp::AND && next_gates[1].op == GateOp::XOR)
            || (next_gates[0].op == GateOp::XOR && next_gates[1].op == GateOp::AND))
        {
            bad_wires.push(gate.output.clone());
            continue;
        }
    }
    bad_wires
}

fn xor_output_non_z_goes_in_one_and_one_xor(
    gates: &Vec<Gate>,
    input_map: &HashMap<String, Vec<Gate>>,
) -> Vec<String> {
    let mut bad_wires = Vec::new();
    for gate in gates {
        if gate.op != GateOp::XOR || gate.output.starts_with("z") {
            continue;
        }
        let next_gates = input_map.get(&gate.output).unwrap();
        if next_gates.len() != 2 {
            bad_wires.push(gate.output.clone());
            continue;
        }
        if !((next_gates[0].op == GateOp::AND && next_gates[1].op == GateOp::XOR)
            || (next_gates[0].op == GateOp::XOR && next_gates[1].op == GateOp::AND))
        {
            bad_wires.push(gate.output.clone());
            continue;
        }
    }
    bad_wires
}

fn xor_with_non_xy_in_has_z_out(gates: &Vec<Gate>) -> Vec<String> {
    let mut bad_wires = Vec::new();
    for gate in gates {
        if gate.op != GateOp::XOR || is_xyz(&gate.inputs.0) || is_xyz(&gate.inputs.1) {
            continue;
        }

        if !gate.output.starts_with("z") {
            bad_wires.push(gate.output.clone());
        }
    }
    bad_wires
}

// With a bit of help from Reddit, I read that the circuit represents
// a Ripple Carry adder. My solution was to draw an adder for 1-bit
// on a piece of paper and come up with some heuristics of what type
// of wires can and can't go into a gate. The goal was to find all
// the bad wires by finding wires that break the heuristics.
// E.g. I figured out that the OR gate can't ever have an xNN, yNN
// or zNN connection, unless it's the last bit.
fn part2(wires: HashMap<String, Option<bool>>, gates: &Vec<Gate>) {
    let input_map = build_input_map(&wires.keys().cloned().collect(), &gates);
    for n in 0..45 {
        find_nth_bit_adder(n, &input_map);
        println!("");
    }

    let mut bad_wires: HashSet<String> = HashSet::new();
    let bad = or_gates_no_xyz(&gates);
    println!("CHECK: OR gates can't have xyz wires in or out: {:?}", bad);
    bad_wires.extend(bad);

    let bad = and_gates_no_xyz_output(&gates);
    println!("CHECK: AND gates can't have xyz outputs: {:?}", bad);
    bad_wires.extend(bad);

    let bad = and_xor_gates_both_xyz_or_none(&gates);
    println!(
        "CHECK: AND/XOR gate inputs are both or neither xyz: {:?}",
        bad
    );
    bad_wires.extend(bad);

    let bad = and_output_is_or_input(&gates, &input_map);
    println!("CHECK: AND outputs are followed by a single OR: {:?}", bad);
    bad_wires.extend(bad);

    let bad = or_output_goes_in_one_and_one_xor(&gates, &input_map);
    println!(
        "CHECK: OR outputs go in exactly one AND & one XOR: {:?}",
        bad
    );
    bad_wires.extend(bad);

    let bad = xor_output_non_z_goes_in_one_and_one_xor(&gates, &input_map);
    println!(
        "CHECK: Non-z XOR outputs go in exactly one AND & one XOR: {:?}",
        bad
    );
    bad_wires.extend(bad);

    let bad = xor_with_non_xy_in_has_z_out(&gates);
    println!("CHECK: XOR with non-xy inputs has z output: {:?}", bad);
    bad_wires.extend(bad);

    let mut sorted: Vec<String> = bad_wires.into_iter().collect();
    sorted.sort();
    println!("");
    println!("Bad wires: {}", sorted.join(","))
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let (wires, gates) = parse_input(&input);
    part1(&mut wires.clone(), &gates);
    part2(wires.clone(), &gates);
}
