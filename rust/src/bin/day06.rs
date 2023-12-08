use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{Context, Result};

use utils::measure;

type Input = Vec<Race>;

#[derive(Debug)]
struct Race {
    time_ms: i64,
    dist_ms: i64,
}

impl Race {
    fn record_ways(&self) -> i64 {
        let &Race { time_ms, dist_ms } = self;

        let t_until_above = (0..=time_ms)
            .take_while(|t| {
                let travelled = t * (time_ms - t);
                travelled <= dist_ms
            })
            .count() as i64;

        time_ms + 1 - (t_until_above * 2)
    }
}

fn part1(input: &Input) -> i64 {
    input.iter().map(Race::record_ways).product()
}

fn part2(input: &Input) -> i64 {
    let mut time_buf = String::new();
    let mut dist_buf = String::new();

    for &Race { time_ms, dist_ms } in input {
        time_buf.push_str(&format!("{time_ms}"));
        dist_buf.push_str(&format!("{dist_ms}"));
    }

    let time_ms = time_buf.parse::<i64>().unwrap_or_default();
    let dist_ms = dist_buf.parse::<i64>().unwrap_or_default();

    Race { dist_ms, time_ms }.record_ways()
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
    let mut races = vec![];

    let times_line = lines.next().context("No times line")?;
    let times = times_line
        .split(':')
        .nth(1)
        .context("No times")?
        .trim()
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<i64>().ok());

    let distances_line = lines.next().context("No distances line")?;
    let distances = distances_line
        .split(':')
        .nth(1)
        .context("No distances")?
        .trim()
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<i64>().ok());

    for (time_ms, dist_ms) in times.zip(distances) {
        races.push(Race { time_ms, dist_ms });
    }

    Ok(races)
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        Time:      7  15   30
        Distance:  9  40  200";

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
        assert_eq!(part1(&as_input(INPUT)?), 288);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 71503);
        Ok(())
    }
}
