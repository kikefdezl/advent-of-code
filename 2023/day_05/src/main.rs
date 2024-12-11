// very inefficient. Full check actually takes too long, but the lowest number is found relatively
// quickly.

use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl Almanac {
    fn map_seed_to_location(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.map(seed);
        let fertilizer = self.soil_to_fertilizer.map(soil);
        let water = self.fertilizer_to_water.map(fertilizer);
        let light = self.water_to_light.map(water);
        let temperature = self.light_to_temperature.map(light);
        let humidity = self.temperature_to_humidity.map(temperature);
        self.humidity_to_location.map(humidity)
    }

    fn recalculate_seeds(&self) -> Vec<u64> {
        let mut new_seeds: Vec<u64> = Vec::new();

        for i in (0..self.seeds.len()).step_by(2) {
            let start = self.seeds[i];
            let end = start + self.seeds[i + 1];
            new_seeds.extend((start..end).collect::<Vec<u64>>());
        }
        new_seeds
    }
}

struct Submap {
    start: u64,
    source_start: u64,
    length: u64,
}

impl Submap {
    fn contains(&self, n: u64) -> bool {
        n >= self.source_start && n <= self.source_start + self.length
    }

    fn map(&self, n: u64) -> u64 {
        self.start + n - self.source_start
    }
}

struct Map {
    submaps: Vec<Submap>,
}

impl Map {
    fn map(&self, n: u64) -> u64 {
        for submap in &self.submaps {
            if submap.contains(n) {
                return submap.map(n);
            }
        }
        n
    }
}

fn parse_input(input: &str) -> Almanac {
    let mut lines: Vec<&str> = input.trim().lines().collect::<Vec<&str>>();
    lines.push(""); // make sure it ends with a blank line

    let seeds: Vec<u64> = lines[0]
        .trim()
        .split_whitespace()
        .filter(|x| x.parse::<u64>().is_ok())
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut idx = 2;
    assert_eq!(lines[idx], "seed-to-soil map:");
    let seed_to_soil = get_map(&lines[idx + 1..].to_vec());

    idx += seed_to_soil.submaps.len() + 2;
    assert_eq!(lines[idx], "soil-to-fertilizer map:");
    let soil_to_fertilizer = get_map(&lines[idx + 1..].to_vec());

    idx += soil_to_fertilizer.submaps.len() + 2;
    assert_eq!(lines[idx], "fertilizer-to-water map:");
    let fertilizer_to_water = get_map(&lines[idx + 1..].to_vec());

    idx += fertilizer_to_water.submaps.len() + 2;
    assert_eq!(lines[idx], "water-to-light map:");
    let water_to_light = get_map(&lines[idx + 1..].to_vec());

    idx += water_to_light.submaps.len() + 2;
    assert_eq!(lines[idx], "light-to-temperature map:");
    let light_to_temperature = get_map(&lines[idx + 1..].to_vec());

    idx += light_to_temperature.submaps.len() + 2;
    assert_eq!(lines[idx], "temperature-to-humidity map:");
    let temperature_to_humidity = get_map(&lines[idx + 1..].to_vec());

    idx += temperature_to_humidity.submaps.len() + 2;
    assert_eq!(lines[idx], "humidity-to-location map:");
    let humidity_to_location = get_map(&lines[idx + 1..].to_vec());

    Almanac {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

fn get_map(lines: &Vec<&str>) -> Map {
    let mut submaps: Vec<Submap> = Vec::new();
    let mut idx = 0;
    while lines[idx] != "" {
        let values: Vec<u64> = lines[idx]
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        submaps.push(Submap {
            start: values[0],
            source_start: values[1],
            length: values[2],
        });
        idx += 1;
    }
    Map { submaps }
}

fn part_one(almanac: &Almanac) {
    let mut lowest: u64 = u64::MAX;
    for seed in &almanac.seeds {
        let location = almanac.map_seed_to_location(*seed);
        if location < lowest {
            lowest = location;
        }
    }
    println!("Lowest location is {}", lowest);
}

fn part_two(almanac: &Almanac) {
    let mut lowest: u64 = u64::MAX;
    let seeds = almanac.recalculate_seeds();
    for seed in seeds {
        let location = almanac.map_seed_to_location(seed);
        if location < lowest {
            println!("New lowest {}", lowest);
            lowest = location;
        }
    }
    println!("Lowest location is {}", lowest);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let almanac = parse_input(&input);

    part_one(&almanac);
    part_two(&almanac);
}
