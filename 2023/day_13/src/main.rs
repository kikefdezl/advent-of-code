use std::cmp::min;
use std::fs::{read_to_string, remove_file};

const INPUT_FILE: &str = "input.txt";

#[derive(Clone, Debug)]
struct Pattern {
    blocks: Vec<char>,
}

impl Pattern {
    fn from_str(s: &str) -> Pattern {
        let blocks = s.trim().chars().collect::<Vec<char>>();
        Pattern { blocks }
    }

    fn print(&self) {
        for c in &self.blocks {
            print!("{c}");
        }
        println!();
    }

    fn matches(&self, other: &Pattern) -> bool {
        let length = self.blocks.len();
        if length != other.blocks.len() {
            return false;
        }
        for i in 0..length {
            if self.blocks[i] != other.blocks[i] {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone)]
struct Mirror {
    patterns: Vec<Pattern>,
}

impl Mirror {
    fn find_reflection_rows(&self) -> Vec<usize> {
        Mirror::_find_reflection_idx(&self.patterns)
    }

    fn find_reflection_cols(&self) -> Vec<usize> {
        // rearrange into columns
        let height = self.patterns.len();
        let mut columns: Vec<Pattern> = Vec::new();

        // assume width is constant so we take the first row
        for i in 0..self.patterns[0].blocks.len() {
            let blocks: Vec<char> = (0..height).map(|j| self.patterns[j].blocks[i]).collect();
            columns.push(Pattern { blocks })
        }

        Mirror::_find_reflection_idx(&columns)
    }

    fn _find_reflection_idx(patterns: &[Pattern]) -> Vec<usize> {
        let height = patterns.len();
        let mut indexes = Vec::new();
        'outer: for i in 0..(height - 1) {
            let reflection_size = min(i + 1, height - i - 1);
            for j in 0..reflection_size {
                if !patterns[i - j].matches(&patterns[i + 1 + j]) {
                    continue 'outer;
                }
            }
            indexes.push(i);
        }
        indexes
    }

    fn toggle(&mut self, x: usize, y: usize) {
        match self.patterns[y].blocks[x] {
            '.' => self.patterns[y].blocks[x] = '#',
            '#' => self.patterns[y].blocks[x] = '.',
            _ => unreachable!(),
        }
    }

    fn print(&self) {
        for pattern in &self.patterns {
            pattern.print();
        }
    }
}

#[derive(Debug)]
struct Mirrors {
    inner: Vec<Mirror>,
}

impl Mirrors {
    fn from_text(s: &str) -> Mirrors {
        let mut mirrors: Vec<Mirror> = Vec::new();
        let lines: Vec<&str> = s.lines().collect();

        let mut patterns: Vec<Pattern> = Vec::new();
        for line in lines {
            if line.trim().is_empty() {
                mirrors.push(Mirror {
                    patterns: patterns.clone(),
                });
                patterns.clear();
            } else {
                patterns.push(Pattern::from_str(line));
            }
        }
        mirrors.push(Mirror {
            patterns: patterns.clone(),
        });
        Mirrors { inner: mirrors }
    }
}

fn part1(mirrors: &Mirrors) {
    let mut sum = 0;
    for mirror in &mirrors.inner {
        let reflection_rows = mirror.find_reflection_rows();
        if !reflection_rows.is_empty() {
            sum += (reflection_rows[0] + 1) * 100
        }

        let reflection_cols = mirror.find_reflection_cols();
        if !reflection_cols.is_empty() {
            sum += reflection_cols[0] + 1
        }
    }
    println!("Sum before smudge removal: {}", sum);
}

fn part2(mirrors: &Mirrors) {
    let mut sum = 0;
    for mirror in &mirrors.inner {
        let original_rows = mirror.find_reflection_rows();
        let original_cols = mirror.find_reflection_cols();

        'outer: for (y, pattern) in mirror.patterns.iter().enumerate() {
            for x in 0..pattern.blocks.len() {
                let mut cloned = mirror.clone();
                cloned.toggle(x, y);

                let mut new_reflection_rows: Vec<usize> = cloned.find_reflection_rows();
                if !original_rows.is_empty() {
                    new_reflection_rows.retain(|&r| r != original_rows[0]);
                }
                if !new_reflection_rows.is_empty() {
                    sum += (new_reflection_rows[0] + 1) * 100;
                    break 'outer;
                }

                let mut new_reflection_cols: Vec<usize> = cloned.find_reflection_cols();
                if !original_cols.is_empty() {
                    new_reflection_cols.retain(|&c| c != original_cols[0]);
                }
                if !new_reflection_cols.is_empty() {
                    sum += new_reflection_cols[0] + 1;
                    break 'outer;
                }
            }
        }
    }
    println!("Sum after smudges are removed: {}", sum);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let mirrors = Mirrors::from_text(&input);
    for mirror in &mirrors.inner {
        mirror.print();
        println!();
    }
    part1(&mirrors);
    part2(&mirrors);
}
