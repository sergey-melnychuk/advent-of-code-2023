use std::collections::{HashMap, VecDeque};
use advent_of_code_2023::*;

fn main() {
    let gates = parse(&lines());

    println!("{}", part1(&gates));
    // Farewell, AoC. It was fun.
}

fn part1(gates: &HashMap<Name, Gate>) -> usize {
    let mut gates = gates.to_owned();
    let (mut lo, mut hi) = (0, 0);
    const N: usize = 1000;
    for _ in 0..N {
        let r = hit(&mut gates);
        lo += r.0;
        hi += r.1;
    }
    lo * hi
}

type Name = [char; 2];

#[derive(Clone, Copy, Debug)]
enum Pulse {
    Hi,
    Lo,
}

#[derive(Clone, Debug)]
enum Gate {
    FlipFlop {
        on: bool,
        inputs: Vec<Name>,
    },
    Conjunct {
        inputs: Vec<Name>,
        states: HashMap<Name, Pulse>,
    },
    Broadcast {
        inputs: Vec<Name>,
    }
}

fn hit(gates: &mut HashMap<Name, Gate>) -> (usize, usize) {
    let (mut lo, mut hi) = (0, 0);
    let mut queue: VecDeque<(Name, Pulse, Name)> = VecDeque::new();
    queue.push_back((BC, Pulse::Lo, OO));
    while let Some((name, pulse, from)) = queue.pop_front() {
        println!("name={} pulse={pulse:?} from={}", s(name), s(from));
        match pulse {
            Pulse::Lo => { lo += 1; }
            Pulse::Hi => { hi += 1; }
        };
        if let Some(gate) = gates.get_mut(&name) {
            for (to, pulse) in gate.handle(pulse, from) {
                queue.push_back((to, pulse, name));
            }    
        }
    }
    // "module configuration" has cycles apparently
    // panic!("Single hit completed!");
    (lo, hi)
}

fn s(name: Name) -> String {
    format!("{}{}", name[0], name[1])
}

impl Gate {
    fn handle(&mut self, pulse: Pulse, from: Name) -> Vec<(Name, Pulse)> {
        match self {
            Self::Broadcast { inputs } => {
                inputs.iter().cloned().map(|input| (input, pulse)).collect()
            }
            Self::FlipFlop { on, inputs } if matches!(pulse, Pulse::Lo) => {
                let p = if *on {
                    *on = false;
                    Pulse::Lo
                } else {
                    *on = true;
                    Pulse::Hi
                };
                inputs.iter().cloned().map(|input| (input, p)).collect()
            }
            Self::Conjunct { inputs, states } => {
                states.entry(from).or_insert(pulse);
                let p = if states.values().all(|p| matches!(p, Pulse::Hi)) {
                    Pulse::Lo
                } else {
                    Pulse::Hi
                };
                inputs.iter().cloned().map(|input| (input, p)).collect()
            }
            _ => vec![]
        }
    }
}

fn parse(lines: &[String]) -> HashMap<Name, Gate> {
    lines.iter()
        .map(|line| parse_gate(line))
        .collect()
}

const BC: Name = ['B', 'C'];
const OO: Name = ['0', '0'];

fn parse_gate(line: &str) -> (Name, Gate) {
    if line.starts_with("broadcaster") {
        let inputs = line.strip_prefix("broadcaster -> ").unwrap()
            .split(", ")
            .map(as_name)
            .collect();
        (BC, Gate::Broadcast { inputs })
    } else if line.starts_with('%') {
        let mut it = line.strip_prefix('%').unwrap().split(" -> ");
        let name = as_name(&it.next().unwrap());
        let inputs = it.next().unwrap().split(", ").map(as_name).collect();
        (name, Gate::FlipFlop { on: false, inputs })
    } else if line.starts_with('&') {
        let mut it = line.strip_prefix('&').unwrap().split(" -> ");
        let name = as_name(&it.next().unwrap());
        let inputs: Vec<Name> = it.next().unwrap().split(", ").map(as_name).collect();
        let states = HashMap::new();
        (name, Gate::Conjunct { inputs, states })
    } else {
        panic!("invalid gate: {line}");
    }
}

fn as_name(chunk: &str) -> Name {
    assert_eq!(chunk.len(), 2, "invalid chunk: {}", chunk);
    let fst = chunk.chars().nth(0).unwrap();
    let snd = chunk.chars().nth(1).unwrap();
    [fst, snd]
}

// cargo run --bin day20 < day20.txt
// cargo run --bin day20 < txt/day20.txt
// cargo run --release --bin day20 < txt/day20.txt
// cargo test --package advent-of-code-2023 --bin day20 -- day20 --nocapture

#[cfg(test)]
mod day20 {
    #[test]
    fn test_it() {
        assert!(true);
    }
}
