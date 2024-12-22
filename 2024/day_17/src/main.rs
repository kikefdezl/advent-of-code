use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

#[derive(Clone)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

fn parse_input(input: &str) -> (Registers, Vec<u8>) {
    let lines: Vec<&str> = input.lines().collect();
    let a: u64 = lines[0][12..].parse().unwrap();
    let b: u64 = lines[1][12..].parse().unwrap();
    let c: u64 = lines[2][12..].parse().unwrap();
    let registers = Registers { a, b, c };

    let program: Vec<u8> = lines[4][9..]
        .split(",")
        .map(|c| c.parse::<u8>().unwrap())
        .collect();

    (registers, program)
}

fn combo(registers: &Registers, operand: u8) -> u64 {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        _ => 0,
    }
}

// 0
fn adv(registers: &mut Registers, value: u64) {
    registers.a /= 2_u64.pow(value as u32);
}

// 1
fn bxl(registers: &mut Registers, value: u8) {
    registers.b ^= value as u64;
}

// 2
fn bst(registers: &mut Registers, value: u64) {
    registers.b = value % 8;
}

// 3
fn jnz(registers: &Registers, value: u8) -> Option<usize> {
    if registers.a == 0 {
        return None;
    }
    Some(value as usize)
}

// 4
fn bxc(registers: &mut Registers) {
    registers.b ^= registers.c;
}

// 5
fn out(val: u64) -> u8 {
    (val % 8) as u8
}

// 6
fn bdv(registers: &mut Registers, value: u64) {
    registers.b = registers.a / 2_u64.pow(value as u32);
}

// 7
fn cdv(registers: &mut Registers, value: u64) {
    registers.c = registers.a / 2_u64.pow(value as u32);
}

fn run_program(mut registers: &mut Registers, program: &Vec<u8>) -> Vec<u8> {
    let mut pointer = 0;
    let mut results: Vec<u8> = Vec::new();
    while pointer < program.len() {
        let value = program[pointer + 1];
        let combo_value = combo(&registers, value);
        match program[pointer] {
            0 => adv(&mut registers, combo_value),
            1 => bxl(&mut registers, value),
            2 => bst(&mut registers, combo_value),
            3 => {
                if let Some(jump_to) = jnz(&registers, value) {
                    pointer = jump_to;
                    continue;
                }
            }
            4 => bxc(&mut registers),
            5 => results.push(out(combo_value)),
            6 => bdv(&mut registers, combo_value),
            7 => cdv(&mut registers, combo_value),
            _ => panic!(),
        };
        pointer += 2;
    }
    results
}

fn part1(mut registers: Registers, program: &Vec<u8>) {
    let output = run_program(&mut registers, &program);

    let output_str = output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("Program output: {}", output_str);
}

fn part2(mut registers: Registers, program: &Vec<u8>) {
    // Not pretty, but I manually found the
    // range where the solution would be by tweaking
    // these two vars and brute-forcing.
    
    let step = 8_u64.pow(0);
    let mut a = 105_734_774_000_000;

    let b = registers.b;
    let c = registers.c;
    loop {
        registers.a = a;
        registers.b = b;
        registers.c = c;
        let output = run_program(&mut registers, &program);
        if output.eq(program) {
            break;
        }
        println!("For a={}: {:?} - {}", a, output, output.len());
        a += 1 * step;
    }
    println!("Program outputs itself for A = {}", a);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let (registers, program) = parse_input(&input);
    part1(registers.clone(), &program);
    part2(registers, &program);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut registers = Registers { a: 0, b: 0, c: 9 };
        let program: Vec<u8> = vec![2, 6];
        run_program(&mut registers, &program);
        assert_eq!(registers.b, 1);
    }
    #[test]
    fn test_case_2() {
        let mut registers = Registers { a: 10, b: 0, c: 0 };
        let program: Vec<u8> = vec![5, 0, 5, 1, 5, 4];
        assert_eq!(run_program(&mut registers, &program), vec![0, 1, 2]);
    }
    #[test]
    fn test_case_3() {
        let mut registers = Registers {
            a: 2024,
            b: 0,
            c: 0,
        };
        let program: Vec<u8> = vec![0, 1, 5, 4, 3, 0];
        let expected = vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0];
        assert_eq!(run_program(&mut registers, &program), expected);
        assert_eq!(registers.a, 0);
    }
    #[test]
    fn test_case_4() {
        let mut registers = Registers { a: 0, b: 29, c: 0 };
        let program: Vec<u8> = vec![1, 7];
        run_program(&mut registers, &program);
        assert_eq!(registers.b, 26);
    }
    #[test]
    fn test_case_5() {
        let mut registers = Registers {
            a: 0,
            b: 2024,
            c: 43690,
        };
        let program: Vec<u8> = vec![4, 0];
        run_program(&mut registers, &program);
        assert_eq!(registers.b, 44354);
    }
    #[test]
    fn test_case_6() {
        let mut registers = Registers {
            a: 117440,
            b: 0,
            c: 0,
        };
        let program: Vec<u8> = vec![0, 3, 5, 4, 3, 0];
        assert!(program.eq(&run_program(&mut registers, &program)));
    }
}
