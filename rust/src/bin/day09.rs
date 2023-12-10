use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use anyhow::{Context, Result};

use utils::measure;

type Input = Vec<History>;

#[derive(Debug)]
struct History {
    values: Vec<i32>,
}

fn both_parts(input: &Input) -> (i32, i32) {
    let mut p1 = 0;
    let mut p2 = 0;

    for History { values } in input {
        let mut diffs = vec![];

        for w in values.windows(2) {
            let mut d = w[1] - w[0];
            let mut idx = 0;
            let mut cont = true;
            let n_diffs = diffs.len();

            while cont {
                let diff = if let Some(diff) = diffs.get_mut(idx) {
                    diff
                } else {
                    diffs.push(vec![]);
                    diffs.get_mut(idx).unwrap()
                };

                diff.push(d);

                d = if diff.len() >= 2 {
                    let a = diff[diff.len() - 1];
                    let b = diff[diff.len() - 2];
                    let diff = a - b;
                    cont = idx < n_diffs;
                    diff
                } else {
                    cont = false;
                    0
                };

                idx += 1;
            }
        }

        // Part 1
        let mut last = *diffs[diffs.len() - 1].last().unwrap();
        for i in (0..(diffs.len() - 1)).rev() {
            let next = diffs[i].last().unwrap() + last;
            last = next;
        }
        let next_val = values.last().unwrap() + last;
        p1 += next_val;

        // Part 2
        let mut last = *diffs[diffs.len() - 1].first().unwrap();
        for i in (0..(diffs.len() - 1)).rev() {
            let next = diffs[i].first().unwrap() - last;
            last = next;
        }
        let next_val = values.first().unwrap() - last;
        p2 += next_val;
    }

    (p1, p2)
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

impl FromStr for History {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split(' ')
            .map(|s| s.parse::<i32>().context("Incorrect value"))
            .collect::<Result<_>>()?;
        Ok(History { values })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.parse::<History>()
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

    const INPUT: &str = "
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";

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
        assert_eq!(both_parts(&as_input(INPUT)?).0, 114);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(both_parts(&as_input(INPUT)?).1, 2);
        Ok(())
    }
}
