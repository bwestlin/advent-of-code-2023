use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{Context, Result};

use utils::measure;

type Input = Engine;

#[derive(Debug)]
struct Engine {
    schematic: Vec<Vec<char>>,
}

#[derive(Debug)]
struct Examination {
    part_numbers: Vec<i32>,
    gear_ratios: Vec<i32>,
}

impl Engine {
    fn adjacent(&self, x: usize, y: usize) -> impl Iterator<Item = (char, usize, usize)> + '_ {
        [
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ]
        .into_iter()
        .filter_map(move |(dx, dy)| {
            let x = dx + x as i32;
            let y = dy + y as i32;
            (y >= 0
                && (y as usize) < self.schematic.len()
                && x >= 0
                && (x as usize) < self.schematic[y as usize].len())
            .then(|| {
                (
                    self.schematic[y as usize][x as usize],
                    x as usize,
                    y as usize,
                )
            })
        })
    }

    fn examine(&self) -> Examination {
        let mut part_numbers = vec![];
        let mut maybe_gears = HashMap::<(char, usize, usize), Vec<i32>>::new();

        for y in 0..self.schematic.len() {
            let mut num_buf = vec![];
            let mut sym_buf = HashSet::new();
            for x in 0..self.schematic[0].len() {
                let c = self.schematic[y][x];

                if Self::is_number(c) {
                    num_buf.push(c);
                    for (c, x, y) in self.adjacent(x, y).filter(|(c, _, _)| Self::is_symbol(*c)) {
                        sym_buf.insert((c, x, y));
                    }
                } else if !num_buf.is_empty() {
                    if !sym_buf.is_empty() {
                        let s = num_buf.iter().collect::<String>();
                        let n = s.parse::<i32>().unwrap_or_default();
                        part_numbers.push(n);
                        for &(c, x, y) in &sym_buf {
                            if c == '*' {
                                maybe_gears.entry((c, x, y)).or_default().push(n);
                            }
                        }
                    }
                    num_buf.clear();
                    sym_buf.clear();
                }
            }
            if !num_buf.is_empty() && !sym_buf.is_empty() {
                let s = num_buf.iter().collect::<String>();
                let n = s.parse::<i32>().unwrap_or_default();
                part_numbers.push(n);
                for &(c, x, y) in &sym_buf {
                    if c == '*' {
                        maybe_gears.entry((c, x, y)).or_default().push(n);
                    }
                }
            }
        }

        let gears = maybe_gears
            .into_iter()
            .filter(|(_, nrs)| nrs.len() == 2)
            .collect::<Vec<_>>();

        let gear_ratios = gears
            .into_iter()
            .map(|(_, nrs)| nrs.into_iter().product::<i32>())
            .collect();

        Examination {
            part_numbers,
            gear_ratios,
        }
    }

    fn is_number(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_symbol(c: char) -> bool {
        c != '.' && !Self::is_number(c)
    }
}

fn both_parts(input: &Input) -> (i32, i32) {
    let Examination {
        part_numbers,
        gear_ratios,
    } = input.examine();

    (
        part_numbers.into_iter().sum(),
        gear_ratios.into_iter().sum(),
    )
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

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    let schematic = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    Ok(Engine { schematic })
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

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
        assert_eq!(both_parts(&as_input(INPUT)?).0, 4361);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(both_parts(&as_input(INPUT)?).1, 467835);
        Ok(())
    }
}
