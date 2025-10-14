use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

fn parts_one_and_two(entries: Vec<Vec<isize>>) {
    let mut lsum = 0;
    let mut rsum = 0;
    for entry in entries {
        let mut history = vec![entry.clone()];
        while !history.last().unwrap().iter().all(|x| *x == 0) {
            let last = history.last().unwrap();
            let mut differences: Vec<isize> = Vec::new();
            for i in 0..(last.len() - 1) {
                differences.push(last[i + 1] - last[i])
            }
            history.push(differences);
        }

        let mut ldiff = 0;
        let mut rdiff = 0;
        for i in (0..history.len() - 1).rev() {
            ldiff = history[i][0] - ldiff;
            rdiff += history[i].last().unwrap();
        }

        rsum += rdiff;
        lsum += ldiff;
    }
    println!("Sum of future values: {}", rsum);
    println!("Sum of past values: {}", lsum);
}

fn main() {
    let data = read_to_string(INPUT_FILE).unwrap();

    let mut entries: Vec<Vec<isize>> = Vec::new();
    for line in data.lines() {
        let parsed: Vec<isize> = line
            .split_whitespace()
            .map(|x| x.parse::<isize>().expect("Should parse to a number"))
            .collect();
        entries.push(parsed);
    }

    parts_one_and_two(entries);
}
