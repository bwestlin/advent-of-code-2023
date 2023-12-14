use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{Context, Result};

use utils::measure;

type Input = Vec<Pattern>;

#[derive(Debug)]
struct Pattern {
    rows: Vec<Vec<char>>,
}

impl Pattern {
    fn vertical_reflection(&self) -> Option<usize> {
        'outer: for x in 1..(self.rows[0].len()) {
            for y in 0..self.rows.len() {
                let row = &self.rows[y];
                let lhs = &row[0..x];
                let rhs = &row[x..];

                if !lhs.iter().rev().zip(rhs.iter()).all(|(a, b)| a == b) {
                    continue 'outer;
                }
            }
            return Some(x);
        }
        None
    }

    fn horizontal_reflection(&self) -> Option<usize> {
        'outer: for y in 1..(self.rows.len()) {
            for x in 0..self.rows[y].len() {
                let uhs = (0..y).map(|y| self.rows[y][x]);
                let lhs = (y..self.rows.len()).map(|y| self.rows[y][x]);

                if !uhs.rev().zip(lhs).all(|(a, b)| a == b) {
                    continue 'outer;
                }
            }
            return Some(y);
        }
        None
    }

    fn vertical_reflection_smudge(&self) -> Option<usize> {
        'outer: for x in 1..(self.rows[0].len()) {
            let mut has_smudge = false;
            for y in 0..self.rows.len() {
                let row = &self.rows[y];
                let lhs = &row[0..x];
                let rhs = &row[x..];

                let diff = lhs
                    .iter()
                    .rev()
                    .zip(rhs.iter())
                    .filter(|(a, b)| a != b)
                    .count();

                if diff > 1 || (diff == 1 && has_smudge) {
                    continue 'outer;
                } else if diff == 1 {
                    has_smudge = true;
                }
            }
            if !has_smudge {
                continue;
            }
            return Some(x);
        }
        None
    }

    fn horizontal_reflection_smudge(&self) -> Option<usize> {
        'outer: for y in 1..(self.rows.len()) {
            let mut has_smudge = false;
            for x in 0..self.rows[y].len() {
                let uhs = (0..y).map(|y| self.rows[y][x]);
                let lhs = (y..self.rows.len()).map(|y| self.rows[y][x]);

                let diff = uhs.rev().zip(lhs).filter(|(a, b)| a != b).count();

                if diff > 1 || (diff == 1 && has_smudge) {
                    continue 'outer;
                } else if diff == 1 {
                    has_smudge = true;
                }
            }
            if !has_smudge {
                continue;
            }
            return Some(y);
        }
        None
    }
}

fn part1(input: &Input) -> usize {
    input.iter().fold(0, |acc, pattern| {
        if let Some(vert) = pattern.vertical_reflection() {
            return acc + vert;
        }
        if let Some(hor) = pattern.horizontal_reflection() {
            return acc + 100 * hor;
        }
        acc
    })
}

fn part2(input: &Input) -> usize {
    input.iter().fold(0, |acc, pattern| {
        if let Some(vert) = pattern.vertical_reflection_smudge() {
            return acc + vert;
        }
        if let Some(hor) = pattern.horizontal_reflection_smudge() {
            return acc + 100 * hor;
        }
        acc
    })
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
    let mut patterns = vec![];

    let mut rows = vec![];

    for line in reader.lines().map_while(Result::ok) {
        if line.is_empty() {
            let rows = std::mem::take(&mut rows);
            patterns.push(Pattern { rows });
            continue;
        }

        rows.push(line.chars().collect());
    }

    patterns.push(Pattern { rows });

    Ok(patterns)
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
        
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#";

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
        assert_eq!(part1(&as_input(INPUT)?), 405);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 400);
        Ok(())
    }
}
