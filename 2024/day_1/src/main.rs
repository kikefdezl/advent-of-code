use std::fs;

const INPUT_FILE: &str = "input.txt";

fn main() {
    let contents = fs::read_to_string(INPUT_FILE).unwrap();

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for part in contents.trim().split("\n") {
        let subparts: Vec<&str> = part.split("   ").collect();
        left_list.push(subparts[0].parse::<i32>().unwrap());
        right_list.push(subparts[1].parse::<i32>().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let mut total_distance: i32 = 0;
    for (l, r) in left_list.iter().zip(right_list.iter()) {
        let distance = l - r;
        total_distance += distance.abs();
    }
    println!("Total distance: {}", total_distance);

    let right_list_len = right_list.len();
    let mut total_similarity_score: i32 = 0;
    let mut r_idx: usize = 0;
    for l in &left_list {
        while r_idx < right_list_len && &right_list[r_idx] < l {
            r_idx += 1;
        }

        let mut n_appearances = 0;
        while r_idx < right_list_len && &right_list[r_idx] == l {
            n_appearances += 1;
            r_idx += 1;
        }
        total_similarity_score += l * n_appearances;
    }

    println!("Total similarity score: {}", total_similarity_score);
}
