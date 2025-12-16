use std::{collections::HashMap, fs::read_to_string};

const INPUT_FILE: &str = "input.txt";
const FIRST: &str = "in";
const MAX: usize = 4000;

#[derive(Debug)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum Op {
    GreaterThan,
    LessThan,
}

#[derive(Debug, Clone)]
enum Destination {
    Accepted,
    Rejected,
    Workflow(String),
}

#[derive(Debug)]
struct Rule {
    category: Category,
    op: Op,
    amount: usize,
    destination: Destination,
}

impl Rule {
    fn applies(&self, part: &Part) -> bool {
        let value = match self.category {
            Category::X => part.x,
            Category::M => part.m,
            Category::A => part.a,
            Category::S => part.s,
        };

        match self.op {
            Op::GreaterThan => value > self.amount,
            Op::LessThan => value < self.amount,
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default: Destination,
}

impl Workflow {
    fn from_str(s: &str) -> Vec<Workflow> {
        let mut workflows: Vec<Workflow> = Vec::new();
        for line in s.lines() {
            let open = line.find("{").unwrap();
            let name = line[..open].to_string();

            let close = line.find("}").unwrap();
            let parts: Vec<&str> = line[open + 1..close].split(",").collect();

            let mut rules: Vec<Rule> = Vec::new();
            for part in &parts[..parts.len() - 1] {
                let category = match part.chars().nth(0).unwrap() {
                    'x' => Category::X,
                    'm' => Category::M,
                    'a' => Category::A,
                    's' => Category::S,
                    _ => unreachable!(),
                };

                let op = match part.chars().nth(1).unwrap() {
                    '>' => Op::GreaterThan,
                    '<' => Op::LessThan,
                    _ => unreachable!(),
                };

                let colon = part.find(":").unwrap();
                let amount: usize = part[2..colon].parse().unwrap();

                let destination = match &part[colon + 1..] {
                    "A" => Destination::Accepted,
                    "R" => Destination::Rejected,
                    wf => Destination::Workflow(wf.to_string()),
                };

                let rule = Rule {
                    category,
                    op,
                    amount,
                    destination,
                };
                rules.push(rule)
            }

            let default = match parts.last().cloned().unwrap() {
                "A" => Destination::Accepted,
                "R" => Destination::Rejected,
                wf => Destination::Workflow(wf.to_string()),
            };

            let wf = Workflow {
                name,
                rules,
                default,
            };
            workflows.push(wf);
        }
        workflows
    }

    fn process(&self, part: &Part) -> Destination {
        for rule in &self.rules {
            if rule.applies(part) {
                return rule.destination.clone();
            }
        }
        self.default.clone()
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn from_str(s: &str) -> Vec<Part> {
        let mut parts: Vec<Part> = Vec::new();
        // assumes the order is alwaws x , m , a , s
        for line in s.lines() {
            let pieces: Vec<&str> = line[1..line.len() - 1].split(",").collect();

            let x: usize = pieces[0][2..].parse().unwrap();
            let m: usize = pieces[1][2..].parse().unwrap();
            let a: usize = pieces[2][2..].parse().unwrap();
            let s: usize = pieces[3][2..].parse().unwrap();
            parts.push(Part { x, m, a, s });
        }
        parts
    }
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn part1(parts: &Vec<Part>, workflows: &[Workflow]) {
    let wf_map: HashMap<&str, &Workflow> = workflows.iter().map(|w| (w.name.as_str(), w)).collect();

    let mut sum = 0;
    for part in parts {
        let mut dest = wf_map.get(FIRST).unwrap().process(part);
        while let Destination::Workflow(name) = dest {
            dest = wf_map.get(name.as_str()).unwrap().process(part);
        }
        match dest {
            Destination::Accepted => sum += part.sum(),
            Destination::Rejected => {}
            _ => unreachable!(),
        }
    }
    println!("Total sum of accepted parts: {sum}")
}

// ranges are both inclusive
#[derive(Clone, Debug)]
struct PartRange {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl PartRange {
    fn new() -> PartRange {
        PartRange {
            x: (1, MAX),
            m: (1, MAX),
            a: (1, MAX),
            s: (1, MAX),
        }
    }

    // returns tuple (valid, invalid)
    fn split_for(&self, rule: &Rule) -> (Option<PartRange>, Option<PartRange>) {
        let range = self.get_range(&rule.category);

        let (valid, invalid) = match rule.op {
            Op::GreaterThan => {
                if rule.amount >= range.1 {
                    (None, Some(range))
                } else if rule.amount < range.0 {
                    (Some(range), None)
                } else {
                    (
                        Some((rule.amount + 1, range.1)),
                        Some((range.0, rule.amount)),
                    )
                }
            }
            Op::LessThan => {
                if rule.amount <= range.0 {
                    (None, Some(range))
                } else if rule.amount > range.1 {
                    (Some(range), None)
                } else {
                    (
                        Some((range.0, rule.amount - 1)),
                        Some((rule.amount, range.1)),
                    )
                }
            }
        };

        let valid = valid.map(|r| self.with_range(&rule.category, r));
        let invalid = invalid.map(|r| self.with_range(&rule.category, r));

        (valid, invalid)
    }

    fn get_range(&self, category: &Category) -> (usize, usize) {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    fn with_range(&self, category: &Category, range: (usize, usize)) -> PartRange {
        let mut cloned = self.clone();
        match category {
            Category::X => cloned.x = range,
            Category::M => cloned.m = range,
            Category::A => cloned.a = range,
            Category::S => cloned.s = range,
        }
        cloned
    }

    fn combinations(&self) -> usize {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }
}

fn get_accepted_ranges(
    workflows: &HashMap<&str, &Workflow>,
    workflow: &str,
    mut range: PartRange,
) -> Vec<PartRange> {
    let workflow = workflows.get(workflow).unwrap();

    let mut accepted: Vec<PartRange> = Vec::new();
    for rule in &workflow.rules {
        let (pass, reject) = range.split_for(rule);
        if let Some(s) = pass {
            match &rule.destination {
                Destination::Accepted => accepted.push(s),
                Destination::Rejected => {}
                Destination::Workflow(w) => accepted.extend(get_accepted_ranges(workflows, w, s)),
            }
        }

        match reject {
            None => return Vec::new(),
            Some(k) => range = k,
        }
    }

    match &workflow.default {
        Destination::Accepted => accepted.push(range),
        Destination::Rejected => {}
        Destination::Workflow(w) => accepted.extend(get_accepted_ranges(workflows, w, range)),
    }

    accepted
}

fn part2(workflows: &[Workflow]) {
    let wf_map: HashMap<&str, &Workflow> = workflows.iter().map(|w| (w.name.as_str(), w)).collect();
    let accepted = get_accepted_ranges(&wf_map, FIRST, PartRange::new());
    let mut sum = 0;
    for range in &accepted {
        sum += range.combinations();
    }
    println!("Accepted combinations {}", sum);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let split_idx = input.find("\n\n").expect("Should have a double newline");

    let workflows = Workflow::from_str(&input[..split_idx]);
    let parts = Part::from_str(&input[split_idx + 2..]);
    part1(&parts, &workflows);
    part2(&workflows);
}
