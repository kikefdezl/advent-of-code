use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

const WORD: &str = "XMAS";
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

fn part_one(matrix: &Matrix) {
    let mut sum: u32 = 0;
    for y in 0..matrix.height {
        for x in 0..matrix.width {
            for direction in DIRECTIONS {
                sum += search(WORD, &matrix, x, y, direction.0, direction.1) as u32;
            }
        }
    }

    println!("Total instances of {}: {}", WORD, sum);
}

fn search(word: &str, matrix: &Matrix, x: usize, y: usize, dx: isize, dy: isize) -> bool {
    // searches for 'word' in the matrix, given the current position (x, y) and the direction
    // (dx, dy). Returns true if found.
    let len = word.len();
    let (width, height) = (matrix.width as isize, matrix.height as isize);

    // check boundaries
    if (0..len).any(|i| {
        let nx = x as isize + dx * i as isize;
        let ny = y as isize + dy * i as isize;
        nx < 0 || nx >= width || ny < 0 || ny >= height
    }) {
        return false;
    }

    let found: String = (0..len)
        .map(|i| {
            let nx = (x as isize + dx * i as isize) as usize;
            let ny = (y as isize + dy * i as isize) as usize;
            matrix.data[ny][nx]
        })
        .collect();

    found == word
}

fn part_two(matrix: &Matrix) {
    let mut sum: u32 = 0;
    for y in 1..matrix.height - 1 {
        for x in 1..matrix.width - 1 {
            if matrix.data[y][x] != 'A' {
                continue;
            }

            let tl = matrix.data[y - 1][x - 1];
            let tr = matrix.data[y - 1][x + 1];
            let bl = matrix.data[y + 1][x - 1];
            let br = matrix.data[y + 1][x + 1];

            if ((tl == 'M' || tl == 'S') && (br == 'M' || br == 'S') && tl != br)
                && ((tr == 'M' || tr == 'S') && (bl == 'M' || bl == 'S') && tr != bl)
            {
                sum += 1;
            }
        }
    }

    println!("Total instances of X-MAS: {}", sum);
}

fn main() {
    let contents: String = read_to_string(INPUT_FILE).unwrap();
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
