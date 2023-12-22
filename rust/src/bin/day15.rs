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

type Input = Vec<Vec<u8>>;

fn hash(step: &[u8]) -> u8 {
    let mut hash = 0_u32;
    for b in step.iter() {
        hash += *b as u32;
        hash *= 17;
        hash = hash % 256;
    }

    hash as u8
}

fn part1(input: &Input) -> i32 {
    // dbg!(input);
    let mut ret = 0;
    for i in input {
        let hash = hash(i);
        println!("i={}, hash={}", String::from_utf8(i.clone()).unwrap(), hash);
        ret += hash as i32;
    }
    ret
}

fn part2(input: &Input) -> i32 {
    // dbg!(input);

    let mut boxes = BTreeMap::<u8, Vec<(String, i32)>>::new();

    for i in input {
        let label = i
            .iter()
            .take_while(|&b| *b != b'=' && *b != b'-')
            .cloned()
            .collect::<Vec<_>>();

        let hash = hash(&label);

        let label = label.into_iter().map(|b| b as char).collect::<String>();

        if i[label.len()] == b'=' {
            let focal_length = String::from_utf8((&i[(label.len() + 1)..]).to_vec())
                .unwrap()
                .parse::<i32>()
                .unwrap();
            // dbg!(focal_length);

            let entry = boxes.entry(hash).or_default();
            if let Some((i, _)) = entry.iter().enumerate().find(|(_, (l, _))| l == &label) {
                entry[i].1 = focal_length;
            } else {
                entry.push((label.clone(), focal_length));
            }
        } else if i[label.len()] == b'-' {
            let entry = boxes.entry(hash).or_default();

            if let Some((i, _)) = entry.iter().enumerate().find(|(_, (l, _))| l == &label) {
                entry.remove(i);
            }
        }

        println!(
            "i={}, label={}, hash={}",
            String::from_utf8(i.clone()).unwrap(),
            label,
            hash
        );
        // ret += hash as i32;

        println!("  boxes={boxes:?}");
    }

    let mut res = 0;
    for (bidx, slots) in boxes.into_iter() {
        for (sidx, slot) in slots.into_iter().enumerate() {
            res += (bidx as i32 + 1) * (sidx as i32 + 1) * slot.1;
        }
    }

    res
}

// fn both_parts(input: &Input) -> (i32, i32) {
//     dbg!(input);
//     (0, 0)
// }

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        // let (part1, part2) = both_parts(&input);
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
        Ok(())
    })
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    let line = reader
        .lines()
        .map_while(Result::ok)
        .next()
        .unwrap_or_default();
    let steps = line.split(',').map(|s| s.as_bytes().to_vec()).collect();
    Ok(steps)
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    fn as_input(s: &str) -> Result<Input> {
        read_input(BufReader::new(
            s.split('\n')
                //.skip(1)
                .map(|s| s.trim())
                .collect::<Vec<_>>()
                .join("\n")
                .as_bytes(),
        ))
    }

    // #[test]
    // fn test_part1() -> Result<()> {
    //     assert_eq!(part1(&as_input(INPUT)?), 1320);
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 145);
        Ok(())
    }
}
