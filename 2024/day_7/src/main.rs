use std::fs::read_to_string;
use std::time::Instant;

const INPUT_FILE: &str = "input.txt";

struct Equation {
    values: Vec<u64>,
    result: u64,
}

impl Equation {
    fn has_solution_for(&self, operators: Vec<char>) -> bool {
        let ops = Equation::generate_permutations(operators, self.values.len() - 1);
        for op in ops {
            let mut result = self.values[0];
            for i in 0..self.values.len() - 1 {
                result = apply_operation(result, self.values[i + 1], op[i]);

                // number will only keep on increasing so we can break early
                if result > self.result {
                    break;
                }
            }
            if result == self.result {
                return true;
            }
        }
        false
    }

    fn generate_permutations(chars: Vec<char>, n: usize) -> Vec<Vec<char>> {
        let mut permutations = Vec::new();
        let base: u64 = chars.len() as u64;
        let max = base.pow(n as u32);

        for i in 0..max {
            let mut permutation: Vec<char> = Vec::new();

            for j in 0..n {
                let digit = (i / base.pow(j as u32)) % base;
                permutation.push(chars[digit as usize]);
            }
            permutations.push(permutation);
        }
        permutations
    }
}

fn apply_operation(a: u64, b: u64, op: char) -> u64 {
    match op {
        '*' => a * b,
        '+' => a + b,
        '|' => (a.to_string() + &b.to_string()).parse::<u64>().unwrap(),
        _ => panic!(),
    }
}

fn part_one(equations: &Vec<Equation>) {
    let mut sum = 0;

    for equation in equations {
        if equation.has_solution_for(vec!['*', '+']) {
            sum += equation.result;
        }
    }

    println!("Total sum of possible equations: {}", sum);
}

fn part_two(equations: &Vec<Equation>) {
    let mut sum = 0;

    for equation in equations {
        if equation.has_solution_for(vec!['*', '+', '|']) {
            sum += equation.result;
        }
    }

    println!("Total sum with concatenation: {}", sum);
}

fn main() {
    let contents = read_to_string(INPUT_FILE).unwrap();

    let mut equations: Vec<Equation> = Vec::new();
    for line in contents.lines() {
        let colon = line.find(":").unwrap();
        let result: u64 = line[..colon].parse().unwrap();
        let values: Vec<u64> = line[colon + 1..]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        equations.push(Equation { result, values });
    }

    part_one(&equations);
    let start = Instant::now();
    part_two(&equations);
    println!("Elapsed {:.2?}", Instant::now() - start);
}
