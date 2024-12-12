use std::fs::read_to_string;
use std::iter::zip;

const INPUT_FILE: &str = "input.txt";

struct Race {
    time: u64,
    distance: u64,
}

fn run(races: &Vec<Race>) {
    let mut total_ways = 1;

    for race in races {
        let mut ways = 0;
        for t in 1..race.time {
            let press_time = t;
            let move_time = race.time - t;

            let distance = press_time * move_time;

            if distance > race.distance {
                ways += 1;
            }
        }
        total_ways *= ways;
    }

    println!("total ways: {}", total_ways);
}

fn part_one(times: &Vec<String>, distances: &Vec<String>) {
    let mut races = Vec::new();
    for (t, d) in zip(times.into_iter(), distances.into_iter()) {
        races.push(Race {
            time: t.parse().unwrap(),
            distance: d.parse().unwrap(),
        })
    }
    print!("Part one ");
    run(&races);
}

fn part_two(times: &Vec<String>, distances: &Vec<String>) {
    let time: u64 = times.join("").parse().unwrap();
    let distance: u64 = distances.join("").parse().unwrap();
    let races = vec![Race { time, distance }];
    print!("Part two ");
    run(&races);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let times: Vec<String> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(String::from)
        .collect();

    let distances: Vec<String> = lines[1]
        .split_whitespace()
        .skip(1)
        .map(String::from)
        .collect();

    part_one(&times, &distances);
    part_two(&times, &distances);
}
