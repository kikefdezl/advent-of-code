use std::collections::HashMap;
use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let lines: Vec<&str> = input.lines().collect();
    let towels: Vec<&str> = lines[0].split(",").map(|x| x.trim()).collect();
    let designs: Vec<&str> = lines[2..].iter().map(|x| x.trim()).collect();
    (towels, designs)
}

fn towel_arrangements(design: &str, towels: &[&str], memo: &mut HashMap<String, usize>) -> usize {
    if let Some(v) = memo.get(design) {
        return *v;
    }

    if design.is_empty() {
        return 1;
    }

    let mut arrangements = 0;
    for towel in towels {
        let towel_len = towel.len();
        if towel_len > design.len() {
            continue;
        }

        if &design[..towel_len] == *towel {
            let rest = &design[towel_len..];
            let next_arrangements = towel_arrangements(rest, towels, memo);
            memo.insert(rest.to_string(), next_arrangements);
            arrangements += next_arrangements;
        }
    }
    arrangements
}

fn solve(towels: &Vec<&str>, designs: &Vec<&str>) {
    let mut match_sum = 0;
    let mut arrangement_sum = 0;
    for design in designs {
        let mut memo = HashMap::new();
        let arrangements = towel_arrangements(&design, &towels, &mut memo);
        if arrangements > 0 {
            match_sum += 1;
        }
        arrangement_sum += arrangements;
    }
    println!("{} designs can be done with the given towels", match_sum);
    println!("Sum of different arrangements {} ", arrangement_sum);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let (towels, designs) = parse_input(&input);
    solve(&towels, &designs);
}
