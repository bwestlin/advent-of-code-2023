use std::cmp;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;

use anyhow::{Context, Result};
use regex::Regex;

use utils::measure;

type Input = Vec<ModuleConfig>;

#[derive(Debug, PartialEq, Eq)]
enum Module {
    FlipFlop,
    Conjunction,
    Broadcast,
}

#[derive(Debug)]
struct ModuleConfig {
    name: String,
    module: Module,
    destination: Vec<String>,
}

#[derive(Debug)]
enum ModuleState {
    FlipFlop { state: bool },
    Conjunction { memory: HashMap<String, Pulse> },
    Broadcast,
}

impl ModuleState {
    fn receive(&mut self, source: &String, pulse: Pulse) -> Option<Pulse> {
        match self {
            ModuleState::FlipFlop { state } => {
                if pulse == Pulse::Low {
                    *state = !*state;
                    Some(if *state { Pulse::High } else { Pulse::Low })
                } else {
                    None
                }
            }
            ModuleState::Conjunction { memory } => {
                memory.insert(source.clone(), pulse);

                if memory.values().all(|&p| p == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            ModuleState::Broadcast => Some(pulse),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Pulse {
    Low,
    High,
}

impl ModuleState {
    fn new(conf: &ModuleConfig, configs: &Vec<ModuleConfig>) -> Self {
        match &conf.module {
            Module::FlipFlop => Self::FlipFlop { state: false },
            Module::Conjunction => {
                let memory = configs
                    .iter()
                    .filter(|c| c.destination.contains(&conf.name))
                    .map(|c| (c.name.clone(), Pulse::Low))
                    .collect::<HashMap<_, _>>();

                Self::Conjunction { memory }
            }
            Module::Broadcast => Self::Broadcast,
        }
    }
}

fn part1(input: &Input) -> i32 {
    // dbg!(input);

    let mut states = BTreeMap::new();

    for conf in input {
        let state = ModuleState::new(conf, input);
        // println!("conf={conf:?}, state={state:?}");
        states.insert(&conf.name, (conf, state));
    }

    let mut pulse_count = HashMap::<Pulse, i32>::new();
    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        let str_button = "button".to_string();
        let str_broadcaster = "broadcaster".to_string();

        queue.push_back((&str_button, &str_broadcaster, Pulse::Low));

        while let Some((source, target, pulse)) = queue.pop_front() {
            println!("{source} -{pulse:?}-> {target}");
            *pulse_count.entry(pulse).or_default() += 1;

            if let Some((conf, state)) = states.get_mut(target) {
                if let Some(next_pulse) = state.receive(source, pulse) {
                    for d in &conf.destination {
                        queue.push_back((target, d, next_pulse));
                    }
                }
            }
        }
    }
    println!("pulse_count={pulse_count:?}");

    pulse_count.values().product()
}

fn part2(input: &Input) -> u64 {
    // dbg!(input);

    let mut states = BTreeMap::new();

    for conf in input {
        let state = ModuleState::new(conf, input);
        // println!("conf={conf:?}, state={state:?}");
        states.insert(&conf.name, (conf, state));
    }
    let str_button = "button".to_string();
    let str_broadcaster = "broadcaster".to_string();
    let str_qn = "qn".to_string();

    let qn = states.get(&str_qn).unwrap();
    println!("qn.1={:?}", qn.1);
    let qn_inputs = match &qn.1 {
        ModuleState::FlipFlop { .. } => vec![],
        ModuleState::Conjunction { memory } => memory.keys().cloned().collect::<Vec<_>>(),
        ModuleState::Broadcast => vec![],
    };
    println!("qn_inputs={qn_inputs:?}");
    // All qn_inputs should be high

    for n in &qn_inputs {
        let inputs = match &states.get(n).unwrap().1 {
            ModuleState::FlipFlop { state } => vec![],
            ModuleState::Conjunction { memory } => memory.keys().collect::<Vec<_>>(),
            ModuleState::Broadcast => vec![],
        };
        println!("  {n} inputs {:?}", inputs);
    }

    // let conjunctions = states
    //     .iter()
    //     .filter(|(n, (conf, state))| conf.module == Module::Conjunction)
    //     .map(|(name, _)| *name)
    //     .collect::<Vec<_>>();

    let mut queue = VecDeque::new();
    for i in 1usize..=4021 {
        if i % 1000000 == 0 {
            println!("i={i}");
        }
        // let mut rx_pulse_count = HashMap::<Pulse, i32>::new();

        queue.push_back((&str_button, &str_broadcaster, Pulse::Low));

        while let Some((source, target, pulse)) = queue.pop_front() {
            // println!("{source} -{pulse:?}-> {target}");
            // if target == "rx" {
            //     *rx_pulse_count.entry(pulse).or_default() += 1;
            // }

            if let Some((conf, state)) = states.get_mut(target) {
                if let Some(next_pulse) = state.receive(source, pulse) {
                    for d in &conf.destination {
                        queue.push_back((target, d, next_pulse));
                    }
                }
            }

            if target == &str_qn && pulse == Pulse::High {
                println!(
                    "i={i} qn got {pulse:?} from {source} state={:?}",
                    states.get(&str_qn).unwrap().1
                );
            }
        }

        // println!("rx_pulse_count={rx_pulse_count:?}");

        // for n in qn_inputs.iter().skip(1).take(1) {
        //     if let Some((conf, state)) = states.get(n) {
        //         match state {
        //             ModuleState::FlipFlop { state } => {}
        //             ModuleState::Conjunction { memory } => {
        //                 if memory.values().all(|&p| p == Pulse::High) {
        //                     println!(" i={i}, n={n} ={state:?}");
        //                 }
        //             }
        //             ModuleState::Broadcast => {}
        //         }
        //     }
        // }

        // if let Some((conf, state)) = states.get(&str_qn) {
        //     match state {
        //         ModuleState::FlipFlop { state } => {}
        //         ModuleState::Conjunction { memory } => {
        //             // if memory.values().any(|&p| p == Pulse::High) {
        //             println!("qn={state:?}");
        //             // }
        //         }
        //         ModuleState::Broadcast => {}
        //     }
        // }

        // if rx_pulse_count.len() == 1 {
        //     match rx_pulse_count.get(&Pulse::Low) {
        //         Some(&1) => {
        //             println!("Found it!, i={i}");
        //             break;
        //         }
        //         Some(low) => {
        //             println!("low={low}, i={i}");
        //         }
        //         _ => {}
        //     }
        // }
    }

    // qz = 3911
    // jx = 3907
    // cq = 4021
    // tt = 3931

    let all_steps = vec![3911, 3907, 4021, 3931];

    all_steps
        .iter()
        .skip(1)
        .fold(all_steps[0], |acc, &s| lcm(acc, s))
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    loop {
        if a == b || b == 0 {
            break a;
        } else if a == 0 {
            break b;
        } else if b > a {
            std::mem::swap(&mut a, &mut b);
        }
        a %= b;
    }
}

// fn both_parts(input: &Input) -> (i32, i32) {
//     dbg!(input);
//     (0, 0)
// }

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        // let (part1, part2) = both_parts(&input);
        // println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
        Ok(())
    })
}

impl FromStr for ModuleConfig {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.splitn(3, ' ');

        let start = split.next().unwrap();

        let (module, name) = match (start.chars().next(), start) {
            (Some('%'), _) => (Module::FlipFlop, &start[1..]),
            (Some('&'), _) => (Module::Conjunction, &start[1..]),
            (_, "broadcaster") => (Module::Broadcast, start),
            _ => unreachable!(),
        };
        let name = name.to_owned();

        split.next();

        let destination = split
            .next()
            .unwrap()
            .split(", ")
            .into_iter()
            .map(|s| s.to_owned())
            .collect();

        Ok(ModuleConfig {
            name,
            module,
            destination,
        })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.parse::<ModuleConfig>()
                .context("Unable to parse input line")
        })
        .collect()
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "
        broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a";

    const INPUT2: &str = "
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output";

    fn as_input(s: &str) -> Result<Input> {
        read_input(BufReader::new(
            s.split('\n')
                .skip(1)
                .map(|s| s.trim())
                .collect::<Vec<_>>()
                .join("\n")
                .as_bytes(),
        ))
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(&as_input(INPUT1)?), 32000000);
        assert_eq!(part1(&as_input(INPUT2)?), 11687500);
        Ok(())
    }

    // #[test]
    // fn test_part2() -> Result<()> {
    //     assert_eq!(part2(&as_input(INPUT)?), 1337);
    //     Ok(())
    // }
}
