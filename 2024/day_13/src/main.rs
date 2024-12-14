use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";
const SURPLUS: i64 = 10000000000000;

#[derive(Debug, Clone)]
struct Button {
    x: i64,
    y: i64,
}

impl Button {
    fn from_str(str: &str) -> Button {
        let colon = str.find(":").unwrap();
        let comma = str.find(",").unwrap();
        let x: i64 = str[colon + 4..comma].parse().unwrap();
        let y: i64 = str[comma + 4..].parse().unwrap();
        Button { x, y }
    }
}

#[derive(Debug, Clone)]
struct Prize {
    x: i64,
    y: i64,
}

impl Prize {
    fn from_str(str: &str) -> Prize {
        let colon = str.find(":").unwrap();
        let comma = str.find(",").unwrap();
        let x: i64 = str[colon + 4..comma].parse().unwrap();
        let y: i64 = str[comma + 4..].parse().unwrap();
        Prize { x, y }
    }
}

#[derive(Debug, Clone)]
struct Claw {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

fn parse_input(input: &str) -> Vec<Claw> {
    let mut claws: Vec<Claw> = Vec::new();
    let lines: Vec<&str> = input.trim().lines().collect();
    for i in (0..lines.len()).step_by(4) {
        let button_a = Button::from_str(lines[i]);
        let button_b = Button::from_str(lines[i + 1]);
        let prize = Prize::from_str(lines[i + 2]);
        claws.push(Claw {
            button_a,
            button_b,
            prize,
        });
    }

    claws
}

fn find_solution(claw: &Claw) -> (i64, i64) {
    // obtained with basic system of 2 equations
    let a = ((claw.button_b.y * claw.prize.x) - (claw.button_b.x * claw.prize.y))
        / ((claw.button_b.y * claw.button_a.x) - (claw.button_b.x * claw.button_a.y));
    let b = ((claw.button_a.y * claw.prize.x) - (claw.button_a.x * claw.prize.y))
        / ((claw.button_a.y * claw.button_b.x) - (claw.button_a.x * claw.button_b.y));
    (a, b)
}

fn part_one(claws: &Vec<Claw>) {
    let mut tokens = 0;
    for claw in claws {
        let (a, b) = find_solution(claw);
        if ((a * claw.button_a.x) + (b * claw.button_b.x) == claw.prize.x)
            && ((a * claw.button_a.y) + (b * claw.button_b.y) == claw.prize.y)
        {
            tokens += (3 * a) + b
        }
    }
    println!("Minimum required tokens: {}", tokens);
}

fn part_two(claws: &Vec<Claw>) {
    let mut tokens = 0;
    for claw in claws {
        let mut claw = claw.clone();
        claw.prize.x += SURPLUS;
        claw.prize.y += SURPLUS;
        let (a, b) = find_solution(&claw);
        if ((a * claw.button_a.x) + (b * claw.button_b.x) == claw.prize.x)
            && ((a * claw.button_a.y) + (b * claw.button_b.y) == claw.prize.y)
        {
            tokens += (3 * a) + b
        }
    }
    println!("Minimum required tokens: {}", tokens);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let claws = parse_input(&input);

    part_one(&claws);
    part_two(&claws);
}
