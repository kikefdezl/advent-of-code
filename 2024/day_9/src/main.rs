use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

const EMPTY: &str = ".";

fn part_one(disk_map: &Vec<u8>) {
    let mut expanded_map: Vec<String> = expand_disk_map(&disk_map);

    let mut l = 0;
    let mut r = expanded_map.len() - 1;
    loop {
        while &expanded_map[l] != EMPTY {
            l += 1;
        }
        while &expanded_map[r] == EMPTY {
            r -= 1;
        }
        if l >= r {
            break;
        }
        expanded_map[l] = expanded_map[r].clone();
        expanded_map[r] = EMPTY.to_string();
        l += 1;
        r -= 1;
    }

    let checksum = calculate_checksum(&expanded_map);
    println!("Checksum: {}", checksum);
}

fn part_two(disk_map: &Vec<u8>) {
    let mut expanded_map = expand_disk_map(&disk_map);

    let mut r = expanded_map.len() - 1;
    while r > 0 {
        // find location of next file (r + file_len)
        while r > 1 && expanded_map[r] == EMPTY {
            r -= 1;
        }

        let file_id = &expanded_map[r];
        let mut file_len = 1;
        while r > 1 && expanded_map[r - 1] == *file_id {
            r -= 1;
            file_len += 1;
        }

        // find next empty block of disk (e + empty_len)
        let mut e = 0;
        let mut empty_len = 0;
        while empty_len < file_len && e < r {
            if expanded_map[e] == EMPTY {
                empty_len += 1;
            } else {
                empty_len = 0;
            }
            e += 1;
        }
        e -= empty_len;

        // if spot found, move the block
        if empty_len >= file_len {
            for i in 0..file_len {
                expanded_map[e + i] = expanded_map[r + i].clone();
                expanded_map[r + i] = EMPTY.to_string();
            }
        }

        r -= 1;
    }

    let checksum = calculate_checksum(&expanded_map);
    println!("Checksum: {}", checksum);
}

fn expand_disk_map(disk_map: &Vec<u8>) -> Vec<String> {
    let mut expanded_map: Vec<String> = Vec::new();

    let mut is_file = 1;
    let mut file_id = 0;
    for value in disk_map {
        if is_file == 1 {
            for _ in 0..*value {
                expanded_map.push(file_id.to_string());
            }
            file_id += 1;
        } else {
            for _ in 0..*value {
                expanded_map.push(EMPTY.to_string());
            }
        }
        is_file *= -1;
    }
    expanded_map
}

fn calculate_checksum(disk_map: &Vec<String>) -> u64 {
    let mut checksum: u64 = 0;
    for i in 0..disk_map.len() {
        if disk_map[i] == EMPTY {
            continue;
        }
        checksum += i as u64 * disk_map[i].parse::<u64>().unwrap();
    }
    checksum
}

fn main() {
    let contents = read_to_string(INPUT_FILE).unwrap();
    let disk_map: Vec<u8> = contents
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect();
    part_one(&disk_map);
    part_two(&disk_map);
}
