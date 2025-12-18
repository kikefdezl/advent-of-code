use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";
const OUTPUT: &str = "rx";
const BUTTON_PRESSES: u16 = 1000;

type ModuleName = String;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum PulseFrequency {
    Low,
    High,
}

#[derive(Debug)]
struct Pulse {
    freq: PulseFrequency,
    src: ModuleName,
    dst: ModuleName,
}

impl Pulse {
    fn initial() -> Pulse {
        Pulse {
            freq: PulseFrequency::Low,
            src: "button".to_string(),
            dst: "broadcaster".to_string(),
        }
    }
}

#[derive(Clone)]
enum ModuleType {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcaster(Broadcaster),
    Output(ModuleName),
}

impl ModuleType {
    fn name(&self) -> String {
        match self {
            ModuleType::FlipFlop(f) => f.name.to_string(),
            ModuleType::Conjunction(c) => c.name.to_string(),
            ModuleType::Broadcaster(b) => b.name.to_string(),
            ModuleType::Output(o) => o.to_string(),
        }
    }
}

#[derive(Clone)]
struct FlipFlop {
    name: ModuleName,
    state: bool,
    connects_to: Vec<ModuleName>,
}

impl FlipFlop {
    fn from_str(s: &str) -> FlipFlop {
        let arrow = s.find("->").unwrap();
        let name = s[1..arrow].trim().to_string();
        let connects_to = s[arrow + 2..]
            .split(",")
            .map(|m| m.trim().to_string())
            .collect();
        FlipFlop {
            name,
            state: false,
            connects_to,
        }
    }

    fn toggle(&mut self) {
        match self.state {
            true => self.state = false,
            false => self.state = true,
        }
    }

    fn process(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        match pulse.freq {
            PulseFrequency::Low => {
                self.toggle();
                let frequency = match self.state {
                    false => PulseFrequency::Low,
                    true => PulseFrequency::High,
                };
                self.connects_to
                    .iter()
                    .map(|mn| Pulse {
                        freq: frequency,
                        src: self.name.clone(),
                        dst: mn.clone(),
                    })
                    .collect()
            }
            PulseFrequency::High => Vec::new(),
        }
    }
}

#[derive(Clone)]
struct Conjunction {
    name: ModuleName,
    memory: HashMap<ModuleName, PulseFrequency>,
    connects_to: Vec<ModuleName>,
}

impl Conjunction {
    fn from_str(s: &str) -> Conjunction {
        let arrow = s.find("->").unwrap();
        let name = s[1..arrow].trim().to_string();
        let connects_to = s[arrow + 2..]
            .split(",")
            .map(|m| m.trim().to_string())
            .collect();
        Conjunction {
            name,
            memory: HashMap::new(),
            connects_to,
        }
    }

    fn init_inputs(&mut self, inputs: Vec<ModuleName>) {
        for input in inputs {
            self.memory.insert(input.clone(), PulseFrequency::Low);
        }
    }

    fn all_high(&self) -> bool {
        self.memory.values().all(|m| *m == PulseFrequency::High)
    }

    fn process(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        self.memory.insert(pulse.src.clone(), pulse.freq);

        let freq = if self.all_high() {
            PulseFrequency::Low
        } else {
            PulseFrequency::High
        };

        self.connects_to
            .iter()
            .map(|mn| Pulse {
                freq,
                src: self.name.clone(),
                dst: mn.clone(),
            })
            .collect()
    }
}

#[derive(Clone)]
struct Broadcaster {
    name: ModuleName,
    connects_to: Vec<ModuleName>,
}

impl Broadcaster {
    fn from_str(s: &str) -> Broadcaster {
        let arrow = s.find("->").unwrap();
        let name = s[..arrow].trim().to_string();
        let connects_to = s[arrow + 2..]
            .split(",")
            .map(|m| m.trim().to_string())
            .collect();
        Broadcaster { name, connects_to }
    }

    fn process(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        let mut pulses: Vec<Pulse> = Vec::new();
        for dest in &self.connects_to {
            pulses.push(Pulse {
                freq: pulse.freq,
                src: self.name.clone(),
                dst: dest.to_string(),
            })
        }
        pulses
    }
}

fn parse_modules(input: &str) -> HashMap<ModuleName, ModuleType> {
    let lines: Vec<&str> = input.lines().collect();
    let mut modules: HashMap<ModuleName, ModuleType> = HashMap::new();
    for line in lines {
        let module: ModuleType = match line.chars().next().unwrap() {
            '%' => ModuleType::FlipFlop(FlipFlop::from_str(line)),
            '&' => ModuleType::Conjunction(Conjunction::from_str(line)),
            _ => ModuleType::Broadcaster(Broadcaster::from_str(line)),
        };
        modules.insert(module.name(), module);
    }

    let mut connections: HashMap<ModuleName, Vec<ModuleName>> = HashMap::new();
    for line in input.lines() {
        let arrow = line.find("->").unwrap();
        let src = line[..arrow]
            .trim()
            .trim_start_matches(['&', '%'])
            .to_string();
        let dests: Vec<String> = line[arrow + 2..]
            .split(",")
            .map(|m| m.trim().to_string())
            .collect();

        for dest in dests {
            connections
                .entry(dest.to_string())
                .or_default()
                .push(src.clone());

            // insert outputs
            if !modules.contains_key(&dest) {
                modules.insert(dest.to_string(), ModuleType::Output(dest.to_string()));
            }
        }
    }

    for module in modules.values_mut() {
        if let ModuleType::Conjunction(conj) = module {
            conj.init_inputs(connections.get(&conj.name).unwrap().to_vec());
        }
    }

    modules
}

fn part1(modules: &mut HashMap<ModuleName, ModuleType>) {
    let mut lows = 0;
    let mut highs = 0;
    for _ in 0..BUTTON_PRESSES {
        let mut pulses = VecDeque::from(vec![Pulse::initial()]);
        lows += 1;
        while let Some(pulse) = pulses.pop_front() {
            let module = modules.get_mut(&pulse.dst).unwrap();
            let new_pulses = match module {
                ModuleType::Broadcaster(b) => b.process(&pulse),
                ModuleType::FlipFlop(f) => f.process(&pulse),
                ModuleType::Conjunction(c) => c.process(&pulse),
                ModuleType::Output(_) => Vec::new(),
            };
            for pulse in &new_pulses {
                match pulse.freq {
                    PulseFrequency::Low => lows += 1,
                    PulseFrequency::High => highs += 1,
                }
            }
            pulses.extend(new_pulses);
        }
    }
    println!("Low pulses sent: {lows}");
    println!("High pulses sent: {highs}");
    println!("Multiplied: {}", lows * highs);
}

fn find_last_conjunction(modules: &HashMap<ModuleName, ModuleType>) -> Conjunction {
    for mod_ in modules.values() {
        if let ModuleType::Conjunction(c) = mod_
            && c.connects_to.contains(&OUTPUT.to_string())
        {
            return c.clone();
        }
    }
    unreachable!();
}

fn mcm(numbers: &[usize]) -> usize {
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 { a } else { gcd(b, a % b) }
    }
    fn mcm_single(a: usize, b: usize) -> usize {
        (a * b) / gcd(a, b)
    }
    numbers.iter().copied().reduce(mcm_single).unwrap()
}

fn find_lowest_presses(modules: &mut HashMap<ModuleName, ModuleType>) -> usize {
    let last_conjunction = find_last_conjunction(modules);
    let num_inputs = last_conjunction.memory.len();
    let mut last_high_pulses: HashMap<ModuleName, usize> = HashMap::new();
    let mut cycles: HashMap<ModuleName, usize> = HashMap::new();

    let mut presses = 0;
    loop {
        presses += 1;
        let mut pulses = VecDeque::from(vec![Pulse::initial()]);
        while let Some(pulse) = pulses.pop_front() {
            let module = modules.get_mut(&pulse.dst).unwrap();
            let new_pulses = match module {
                ModuleType::Broadcaster(b) => b.process(&pulse),
                ModuleType::FlipFlop(f) => f.process(&pulse),
                ModuleType::Conjunction(c) => c.process(&pulse),
                ModuleType::Output(_) => Vec::new(),
            };
            for pulse in &new_pulses {
                if pulse.dst == last_conjunction.name && pulse.freq == PulseFrequency::High {
                    match last_high_pulses.get(&pulse.src) {
                        Some(v) => cycles.insert(pulse.src.to_string(), presses - v),
                        None => last_high_pulses.insert(pulse.src.to_string(), presses),
                    };

                    if cycles.len() == num_inputs { 
                        // we found the cycles for all the last conjunction inputs
                        let values: Vec<usize> = cycles.values().cloned().collect();
                        return mcm(&values);
                    }
                }
            }
            pulses.extend(new_pulses);
        }
    }
}

fn part2(modules: &mut HashMap<ModuleName, ModuleType>) {
    let presses = find_lowest_presses(modules);
    println!("{} presses until low -> {}", presses, OUTPUT);
}

fn main() {
    let input = read_to_string(INPUT_FILE).unwrap();
    let mut modules = parse_modules(&input);

    part1(&mut modules.clone());
    part2(&mut modules);
}
