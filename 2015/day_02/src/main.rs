use std::{cmp::min, fs::read_to_string};

const INPUT_FILE: &str = "input.txt";

fn part_one(dimensions: &Vec<(u32, u32, u32)>) {
    let mut area = 0;
    for (x, y, z) in dimensions {
        let area_xy = x * y;
        let area_xz = x * z;
        let area_yz = y * z;
        let min = min(area_xy, min(area_xz, area_yz));
        area += 2 * area_xy + 2 * area_xz + 2 * area_yz + min;
    }
    println!("Total required paper: {} sqft", area);
}

fn part_two(dimensions: &Vec<(u32, u32, u32)>) {
    let mut ribbon = 0;
    for (x, y, z) in dimensions {
        let xy = 2 * x + 2 * y;
        let xz = 2 * x + 2 * z;
        let yz = 2 * y + 2 * z;
        let min = min(xy, min(xz, yz));
        ribbon += min + (x * y * z);
    }
    println!("Total required ribbon: {} ft", ribbon);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let mut dimensions: Vec<(u32, u32, u32)> = Vec::new();
    for line in input.trim().lines() {
        let numbers: Vec<u32> = line.split("x").map(|n| n.parse::<u32>().unwrap()).collect();
        dimensions.push((numbers[0], numbers[1], numbers[2]));
    }
    part_one(&dimensions);
    part_two(&dimensions);
}
