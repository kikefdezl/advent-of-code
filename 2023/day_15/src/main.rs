use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

fn hash(s: &str) -> usize {
    let mut current = 0;

    for c in s.chars() {
        current += (c as u8) as usize;
        current *= 17;
        current %= 256;
    }
    current
}

#[derive(Debug)]
enum Operation {
    Add(u8),
    Remove,
}

#[derive(Debug)]
struct Instruction {
    label: String,
    operation: Operation,
}

impl Instruction {
    fn box_idx(&self) -> usize {
        hash(&self.label)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u8,
}

#[derive(Debug, Clone)]
struct LensBox {
    lenses: Vec<Lens>,
}
impl LensBox {
    fn new() -> LensBox {
        LensBox { lenses: Vec::new() }
    }

    fn contains(&self, label: &str) -> bool {
        for lens in &self.lenses {
            if lens.label == label {
                return true;
            }
        }
        false
    }

    fn remove(&mut self, label: &str) {
        self.lenses.retain(|l| l.label != label);
    }

    fn replace(&mut self, label: &str, new_focal_length: u8) {
        for lens in &mut self.lenses {
            if lens.label == label {
                lens.focal_length = new_focal_length;
                return;
            }
        }
    }
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for ins in s.trim().split(',') {
        if ins.contains('-') {
            let minus_idx = ins.len() - 1;
            let label = ins[..minus_idx].to_string();
            instructions.push(Instruction {
                label,
                operation: Operation::Remove,
            })
        } else {
            let equal_idx = ins.find('=').unwrap();
            let label = ins[..equal_idx].to_string();
            let focal_length: u8 = ins[equal_idx + 1..].parse().unwrap();
            instructions.push(Instruction {
                label,
                operation: Operation::Add(focal_length),
            });
        }
    }
    instructions
}

fn part1(s: &str) {
    let mut sum = 0;
    for instruction in s.trim().split(',') {
        sum += hash(instruction);
    }
    println!("Sum of instruction hashes {}", sum);
}

fn part2(instructions: &Vec<Instruction>) {
    let mut boxes: Vec<LensBox> = vec![LensBox::new(); 256];

    for instruction in instructions {
        let box_idx = instruction.box_idx();

        match instruction.operation {
            Operation::Remove => boxes[box_idx].remove(&instruction.label),
            Operation::Add(fl) => {
                if boxes[box_idx].contains(&instruction.label) {
                    boxes[box_idx].replace(&instruction.label, fl)
                } else {
                    boxes[box_idx].lenses.push(Lens {
                        label: instruction.label.clone(),
                        focal_length: fl,
                    })
                }
            }
        }
    }

    let mut focusing_power = 0;
    for (box_idx, b) in boxes.iter().enumerate() {
        for (lens_idx, lens) in b.lenses.iter().enumerate() {
            focusing_power += (1 + box_idx) * (1 + lens_idx) * lens.focal_length as usize
        }
    }
    println!("Total focusing power: {}", focusing_power);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    part1(&input);

    let instructions = parse_instructions(&input);
    part2(&instructions);
}
