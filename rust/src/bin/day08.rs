use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{Context, Result};

use utils::measure;

type Input = Map;

#[derive(Debug)]
struct Map {
    instructions: Vec<char>,
    connections: HashMap<String, (String, String)>,
}

fn part1(input: &Input) -> i32 {
    let Map {
        instructions,
        connections,
    } = input;

    let mut steps = 0;
    let mut curr = &"AAA".to_string();
    for s in 0.. {
        if curr == "ZZZ" {
            break;
        }
        let i_idx = s % instructions.len();

        let dir = instructions[i_idx];
        let dirs = &connections[curr];

        match dir {
            'L' => curr = &dirs.0,
            'R' => curr = &dirs.1,
            _ => unreachable!(),
        }
        steps += 1;
    }
    steps
}

fn part2(input: &Input) -> u64 {
    let Map {
        instructions,
        connections,
    } = input;

    let currs = connections
        .keys()
        .filter(|s| s.ends_with('A'))
        .collect::<Vec<_>>();
    let mut all_steps = vec![0_u64; currs.len()];

    for i in 0..currs.len() {
        let mut steps = 0;
        let mut curr = currs[i];
        for s in 0.. {
            if curr.ends_with('Z') {
                break;
            }
            let i_idx = s % instructions.len();
            let dir = instructions[i_idx];

            let dirs = &connections[curr];

            match dir {
                'L' => curr = &dirs.0,
                'R' => curr = &dirs.1,
                _ => unreachable!(),
            }
            steps += 1;
        }
        all_steps[i] = steps;
    }

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

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
        Ok(())
    })
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    let mut lines = reader.lines().map_while(Result::ok);

    let instructions = lines.next().context("No instructions")?.chars().collect();
    let mut connections = HashMap::new();

    lines.next();

    for line in lines {
        let mut split = line.split(" = ");
        let from = split.next().context("No from")?.trim().to_string();

        let to_part = split
            .next()
            .context("No to")?
            .trim()
            .chars()
            .skip(1)
            .take_while(|&c| c != ')')
            .collect::<String>();
        let mut to_part = to_part.split(", ");

        let left = to_part.next().context("No left to")?.to_string();
        let right = to_part.next().context("No right to")?.to_string();

        connections.insert(from, (left, right));
    }

    Ok(Map {
        instructions,
        connections,
    })
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)";

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
        assert_eq!(part1(&as_input(INPUT)?), 6);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 6);
        Ok(())
    }
}
