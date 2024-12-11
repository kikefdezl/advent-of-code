use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

const MAX_RED_CUBES: u8 = 12;
const MAX_GREEN_CUBES: u8 = 13;
const MAX_BLUE_CUBES: u8 = 14;

struct Set {
    count: u8,
    color: String,
}

struct Game {
    id: u16,
    sets: Vec<Set>,
}

fn part_one(games: &Vec<Game>) {
    let mut sum: u16 = 0;

    for game in games {
        if is_game_possible(&game) {
            sum += game.id;
        }
    }
    println!("Sum of possible game IDs: {}", sum);
}

fn is_game_possible(game: &Game) -> bool {
    for set in &game.sets {
        if set.color == "red" && set.count > MAX_RED_CUBES
            || set.color == "green" && set.count > MAX_GREEN_CUBES
            || set.color == "blue" && set.count > MAX_BLUE_CUBES
        {
            return false;
        }
    }
    true
}

fn part_two(games: &Vec<Game>) {
    let mut sum_of_powers: u32 = 0;

    for game in games {
        sum_of_powers += power_of_minimum_possible_cubes(&game)
    }
    println!("Sum of powers: {}", sum_of_powers);
}

fn power_of_minimum_possible_cubes(game: &Game) -> u32 {
    let mut min_red_cubes: u32 = 0;
    let mut min_green_cubes: u32 = 0;
    let mut min_blue_cubes: u32 = 0;

    for set in &game.sets {
        let count = set.count as u32;

        if set.color == "red" && count > min_red_cubes {
            min_red_cubes = count;
        }
        if set.color == "green" && count > min_green_cubes {
            min_green_cubes = count;
        }
        if set.color == "blue" && count > min_blue_cubes {
            min_blue_cubes = count;
        }
    }
    min_red_cubes * min_green_cubes * min_blue_cubes
}

fn main() {
    let content = read_to_string(INPUT_FILE).unwrap();

    let mut games: Vec<Game> = Vec::new();

    for line in content.trim().split("\n") {
        let rest = line.strip_prefix("Game ").unwrap();

        let mut split = rest.split(":");
        let game_id = split.next().unwrap().trim().parse::<u16>().unwrap();

        let rest = split.next().unwrap().trim();

        let mut sets: Vec<Set> = Vec::new();

        let shows: Vec<&str> = rest.split(";").collect();
        for set in shows {
            let subsets: Vec<&str> = set.split(",").collect();
            for subset in subsets {
                let mut subset = subset.trim().split(" ");
                let count = subset.next().unwrap().parse::<u8>().unwrap();
                let color = subset.next().unwrap();
                sets.push(Set {
                    count,
                    color: color.to_string(),
                });
            }
        }

        games.push(Game { id: game_id, sets });
    }

    part_one(&games);
    part_two(&games);
}
