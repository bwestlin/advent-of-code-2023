use std::collections::{BTreeSet, HashSet};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{Context, Result};

use utils::measure;

type Input = Space;

#[derive(Debug, Clone)]
struct Space {
    galaxies: Vec<Pos>,
}

impl Space {
    fn expand(&mut self, amount: i64) {
        let step = amount - 1;

        let mut max_x = self.galaxies.iter().map(|p| p.x).max().unwrap_or_default();
        let mut x = 0;

        while x < max_x {
            let found = self.galaxies.iter().any(|g| g.x == x);
            if found {
                x += 1;
                continue;
            }

            let mut any = false;
            for g in self.galaxies.iter_mut() {
                if g.x > x {
                    g.x += step;
                    any = true;
                }
            }

            if any {
                x += step;
                max_x += step;
            }
            x += 1;
        }

        let mut max_y = self.galaxies.iter().map(|p| p.y).max().unwrap_or_default();
        let mut y = 0;

        while y < max_y {
            let found = self.galaxies.iter().any(|g| g.y == y);
            if found {
                y += 1;
                continue;
            }

            let mut any = false;
            for g in self.galaxies.iter_mut() {
                if g.y > y {
                    g.y += step;
                    any = true;
                }
            }

            if any {
                y += step;
                max_y += step;
            }
            y += 1;
        }
    }

    fn galaxy_pairs(&self) -> impl Iterator<Item = (usize, usize)> {
        let mut unique = HashSet::new();

        for a in 0..self.galaxies.len() {
            for b in 0..self.galaxies.len() {
                let pair = [a, b].into_iter().collect::<BTreeSet<_>>();
                unique.insert(pair);
            }
        }

        unique
            .into_iter()
            .filter(|pair| pair.len() == 2)
            .map(|pair| {
                let mut p = pair.into_iter();
                (p.next().unwrap_or_default(), p.next().unwrap_or_default())
            })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn manh_dist(&self, other: &Pos) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn part1(input: &Input) -> i64 {
    let mut space = input.clone();
    space.expand(2);

    space
        .galaxy_pairs()
        .map(|(a, b)| space.galaxies[a].manh_dist(&space.galaxies[b]))
        .sum()
}

fn part2(input: &Input) -> i64 {
    let mut space = input.clone();

    #[cfg(not(test))]
    space.expand(1000000);
    #[cfg(test)]
    space.expand(100);

    space
        .galaxy_pairs()
        .map(|(a, b)| space.galaxies[a].manh_dist(&space.galaxies[b]))
        .sum()
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
    let lines = reader.lines().map_while(Result::ok);

    let mut galaxies = vec![];

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                let p = Pos::new(x as i64, y as i64);
                galaxies.push(p);
            }
        }
    }

    Ok(Space { galaxies })
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";

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
        assert_eq!(part1(&as_input(INPUT)?), 374);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 8410);
        Ok(())
    }
}
