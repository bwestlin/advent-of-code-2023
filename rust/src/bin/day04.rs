use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use anyhow::{Context, Result};

use utils::measure;

type Input = Vec<Card>;

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<i32>,
    numbers_you_have: HashSet<i32>,
}

fn both_parts(input: &Input) -> (i32, i32) {
    let mut p1 = 0;
    let mut copies = vec![1; input.len()];

    for (idx, card) in input.iter().enumerate() {
        let mut points = 0;
        let mut matches = 0;

        for n in &card.numbers_you_have {
            if card.winning_numbers.contains(n) {
                points = if points == 0 { 1 } else { points * 2 };
                matches += 1;
            }
        }

        p1 += points;

        for nidx in ((idx + 1)..copies.len()).take(matches) {
            copies[nidx] += copies[idx];
        }
    }

    (p1, copies.into_iter().sum())
}

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        let (part1, part2) = both_parts(&input);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
        Ok(())
    })
}

impl FromStr for Card {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.splitn(2, ':');
        let mut numbers_part = split.nth(1).context("no reveal part")?.split('|');

        let winning_numbers = numbers_part
            .next()
            .context("no winning numbers")?
            .trim()
            .split(' ')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let numbers_you_have = numbers_part
            .next()
            .context("no numbers you have ")?
            .trim()
            .split(' ')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        Ok(Card {
            winning_numbers,
            numbers_you_have,
        })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.parse::<Card>().context("Unable to parse input line"))
        .collect()
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

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
        assert_eq!(both_parts(&as_input(INPUT)?).0, 13);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(both_parts(&as_input(INPUT)?).1, 30);
        Ok(())
    }
}
