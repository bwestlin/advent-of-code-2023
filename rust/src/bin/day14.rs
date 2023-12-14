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

type Input = Map;

#[derive(Debug, Clone)]
struct Map {
    rounded_rocks: Vec<Pos>,
    cubed_rocks: Vec<Pos>,
}

impl Map {
    fn tilt_north(&mut self) {
        let mut roundex_idx = (0..self.rounded_rocks.len()).collect::<Vec<_>>();
        roundex_idx.sort_by_key(|&idx| self.rounded_rocks[idx].y);

        for idx in roundex_idx {
            let rock = self.rounded_rocks[idx];
            let mut rock_y = rock.y;

            for y in (0..=rock_y).rev() {
                let pos = Pos::new(rock.x, y);
                if self.cubed_rocks.iter().any(|&cr| cr == pos)
                    || self.rounded_rocks.iter().any(|&rr| rr != rock && rr == pos)
                {
                    break;
                }
                rock_y = y;
            }

            self.rounded_rocks[idx].y = rock_y;
        }
    }

    fn tilt_west(&mut self) {
        let mut roundex_idx = (0..self.rounded_rocks.len()).collect::<Vec<_>>();
        roundex_idx.sort_by_key(|&idx| self.rounded_rocks[idx].x);

        for idx in roundex_idx {
            let rock = self.rounded_rocks[idx];
            let mut rock_x = rock.x;

            for x in (0..=rock_x).rev() {
                let pos = Pos::new(x, rock.y);
                if self.cubed_rocks.iter().any(|&cr| cr == pos)
                    || self.rounded_rocks.iter().any(|&rr| rr != rock && rr == pos)
                {
                    break;
                }
                rock_x = x;
            }

            self.rounded_rocks[idx].x = rock_x;
        }
    }

    fn tilt_south(&mut self) {
        let mut roundex_idx = (0..self.rounded_rocks.len()).collect::<Vec<_>>();
        roundex_idx.sort_by_key(|&idx| self.rounded_rocks[idx].y);
        roundex_idx.reverse();
        let max_y = self.max_y();

        for idx in roundex_idx {
            let rock = self.rounded_rocks[idx];
            let mut rock_y = rock.y;

            for y in (rock_y..=max_y) {
                let pos = Pos::new(rock.x, y);
                if self.cubed_rocks.iter().any(|&cr| cr == pos)
                    || self.rounded_rocks.iter().any(|&rr| rr != rock && rr == pos)
                {
                    break;
                }
                rock_y = y;
            }

            self.rounded_rocks[idx].y = rock_y;
        }
    }

    fn tilt_east(&mut self) {
        let mut roundex_idx = (0..self.rounded_rocks.len()).collect::<Vec<_>>();
        roundex_idx.sort_by_key(|&idx| self.rounded_rocks[idx].x);
        roundex_idx.reverse();
        let max_x = self.max_x();

        for idx in roundex_idx {
            let rock = self.rounded_rocks[idx];
            let mut rock_x = rock.x;

            for x in (rock_x..=max_x) {
                let pos = Pos::new(x, rock.y);
                if self.cubed_rocks.iter().any(|&cr| cr == pos)
                    || self.rounded_rocks.iter().any(|&rr| rr != rock && rr == pos)
                {
                    break;
                }
                rock_x = x;
            }

            self.rounded_rocks[idx].x = rock_x;
        }
    }

    fn north_load(&self) -> usize {
        let max_y = self.max_y();
        let mut load = 0;
        for y in 0..=max_y {
            let cnt = self.rounded_rocks.iter().filter(|rr| rr.y == y).count();

            load += cnt * ((max_y - y) + 1);
        }

        load
    }

    fn max_x(&self) -> usize {
        self.rounded_rocks
            .iter()
            .chain(self.cubed_rocks.iter())
            .map(|p| p.x)
            .max()
            .unwrap_or_default()
    }

    fn max_y(&self) -> usize {
        self.rounded_rocks
            .iter()
            .chain(self.cubed_rocks.iter())
            .map(|p| p.y)
            .max()
            .unwrap_or_default()
    }

    fn print(&self) {
        let max_x = self.max_x();
        let max_y = self.max_y();

        for y in 0..=max_y {
            for x in 0..=max_x {
                let pos = Pos::new(x, y);
                let c = if self.rounded_rocks.iter().any(|&p| p == pos) {
                    'O'
                } else if self.cubed_rocks.iter().any(|&p| p == pos) {
                    '#'
                } else {
                    '.'
                };
                print!("{c}");
            }
            println!();
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn part1(input: &Input) -> usize {
    // dbg!(input);
    let mut input = input.clone();

    input.print();
    input.tilt_north();
    println!("Tilted");
    input.print();
    input.north_load()
}

fn part2(input: &Input) -> usize {
    // dbg!(input);
    let mut input = input.clone();

    let mut history = vec![];

    for i in 0..1000 {
        // println!();
        // println!("Initial:");
        // input.print();

        // println!();
        // println!("Tilted north:");
        input.tilt_north();
        // input.print();

        // println!();
        // println!("Tilted west:");
        input.tilt_west();
        // input.print();

        // println!();
        // println!("Tilted south:");
        input.tilt_south();
        // input.print();

        // println!();
        // println!("Tilted east:");
        input.tilt_east();
        // input.print();

        let north_load = input.north_load();
        println!("i={i}, north_load={north_load}");

        if let Some((prev_i, _)) = history
            .iter()
            .enumerate()
            .rev()
            .find(|&(_, nl)| *nl == north_load)
        {
            let delta = i - prev_i;
            if delta > 5 && prev_i as i32 - delta as i32 > 0 {
                if (prev_i..(prev_i + delta)).all(|idx| history[idx] == history[idx - delta]) {
                    println!("  delta={delta} prev_i={prev_i}");
                    let target_i = prev_i + ((1000000000 - (prev_i + 1)) % delta);
                    dbg!(target_i);
                    return history[target_i];
                }
            }
        }

        history.push(north_load);
    }

    // 90218 not right
    // 90176
    dbg!(history);
    0
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
    let mut rounded_rocks = vec![];
    let mut cubed_rocks = vec![];

    for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'O' {
                rounded_rocks.push(Pos::new(x, y))
            }
            if c == '#' {
                cubed_rocks.push(Pos::new(x, y))
            }
        }
    }

    Ok(Map {
        rounded_rocks,
        cubed_rocks,
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
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....";

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

    // #[test]
    // fn test_part1() -> Result<()> {
    //     assert_eq!(part1(&as_input(INPUT)?), 136);
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 64);
        Ok(())
    }
}
