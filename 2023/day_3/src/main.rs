use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

const DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),   // right
    (-1, 0),  // left
    (0, 1),   // down
    (0, -1),  //up
    (1, 1),   // right down
    (1, -1),  // right up
    (-1, 1),  // left down
    (-1, -1), // left up
];

struct Matrix {
    data: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

#[derive(PartialEq)]
struct Range {
    x: (usize, usize),
    y: usize,
}

fn get_adjacent(matrix: &Matrix, x: usize, y: usize, dx: isize, dy: isize) -> Option<char> {
    let nx = x as isize + dx;
    let ny = y as isize + dy;
    if nx < 0 || nx >= matrix.width as isize || ny < 0 || ny >= matrix.height as isize {
        return None;
    }
    Some(matrix.data[ny as usize][nx as usize])
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_digit(10)
}

fn part_one(matrix: &Matrix) {
    let mut sum = 0;
    for y in 0..matrix.height {
        let mut x = 0;
        while x < matrix.width {
            let mut number_len = 0;
            while x + number_len < matrix.width && matrix.data[y][x + number_len].is_digit(10) {
                number_len += 1;
            }

            if number_len == 0 {
                x += 1;
                continue;
            }

            let number_as_str: String = matrix.data[y][x..x + number_len].iter().collect();
            let number: u32 = number_as_str.parse().unwrap();

            'adjacent_symbol_search: for l in 0..number_len {
                for direction in DIRECTIONS {
                    let adj = get_adjacent(&matrix, x + l, y, direction.0, direction.1);
                    if adj.is_none() {
                        continue;
                    };
                    if is_symbol(adj.unwrap()) {
                        sum += number;
                        break 'adjacent_symbol_search;
                    }
                }
            }

            x += number_len;
        }
    }
    println!("Total sum is {}", sum);
}

fn part_two(matrix: &Matrix) {
    // find all the surrounding digits for each '*', and expand the numbers
    // outwards to obtain their coordinate range. Keep only unique ranges,
    // and add the product if there are exactly 2 unique numbers.
    let mut sum = 0;
    for y in 0..matrix.height {
        for x in 0..matrix.width {
            if matrix.data[y][x] != '*' {
                continue;
            }

            let mut ranges: Vec<Range> = Vec::new();
            for direction in DIRECTIONS {
                let adj = get_adjacent(&matrix, x, y, direction.0, direction.1);
                if adj.is_none() || !adj.unwrap().is_digit(10) {
                    continue;
                };
                let nx = x as isize + direction.0;
                let ny = y as isize + direction.1;
                let range = get_number_range(&matrix, nx as usize, ny as usize);
                ranges.push(range);
            }

            let mut unique_ranges = Vec::new();
            for range in ranges {
                if !unique_ranges.contains(&range) {
                    unique_ranges.push(range);
                }
            }

            if unique_ranges.len() == 2 {
                let number0_as_str: String = matrix.data[unique_ranges[0].y]
                    [unique_ranges[0].x.0..unique_ranges[0].x.1 + 1]
                    .iter()
                    .collect();
                let number0: u32 = number0_as_str.parse().unwrap();
                let number1_as_str: String = matrix.data[unique_ranges[1].y]
                    [unique_ranges[1].x.0..unique_ranges[1].x.1 + 1]
                    .iter()
                    .collect();
                let number1: u32 = number1_as_str.parse().unwrap();
                sum += number0 * number1
            }
        }
    }
    println!("Total sum gear ratios is {}", sum);
}

fn get_number_range(matrix: &Matrix, x: usize, y: usize) -> Range {
    let mut r = x;
    let mut l = x;

    while matrix.data[y][r + 1].is_digit(10) {
        r += 1;
        if r + 1 == matrix.width {
            break;
        }
    }
    while matrix.data[y][l - 1].is_digit(10) {
        l -= 1;
        if l == 0 {
            break;
        }
    }
    Range { x: (l, r), y }
}

fn main() {
    let contents = read_to_string(INPUT_FILE).unwrap();

    let data: Vec<Vec<char>> = contents
        .trim()
        .split("\n")
        .map(|x| x.chars().collect())
        .collect();
    let height = data.len();
    let width = data[0].len();
    let matrix = Matrix {
        data,
        height,
        width,
    };

    part_one(&matrix);
    part_two(&matrix);
}
