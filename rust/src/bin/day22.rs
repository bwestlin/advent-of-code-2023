use std::cmp;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::path::Display;
use std::str::FromStr;

use anyhow::{Context, Result};
use rayon::prelude::*;
use regex::Regex;

use utils::measure;

type Input = Vec<Brick>;

#[derive(Debug, Clone)]
struct Brick {
    p1: Pos,
    p2: Pos,
}

impl Brick {
    fn positions(&self) -> BTreeSet<Pos> {
        match (
            self.p2.x - self.p1.x,
            self.p2.y - self.p1.y,
            self.p2.z - self.p1.z,
        ) {
            (dx, 0, 0) => {
                let x1 = self.p1.x;
                let y = self.p2.y;
                let z = self.p2.z;
                let xs = dx.signum();

                (0..=dx.abs())
                    .map(|m| x1 + (xs * m))
                    .map(|x| Pos::new(x, y, z))
                    .collect()
            }
            (0, dy, 0) => {
                let y1 = self.p1.y;
                let x = self.p2.x;
                let z = self.p2.z;
                let ys = dy.signum();

                (0..=dy.abs())
                    .map(|m| y1 + (ys * m))
                    .map(|y| Pos::new(x, y, z))
                    .collect()
            }
            (0, 0, dz) => {
                let z1 = self.p1.z;
                let x = self.p2.x;
                let y = self.p2.y;
                let zs = dz.signum();

                (0..=dz.abs())
                    .map(|m| z1 + (zs * m))
                    .map(|z| Pos::new(x, y, z))
                    .collect()
            }
            _ => unreachable!("self={self:?}"),
        }
    }

    fn intersects(&self, other: &Brick) -> bool {
        let self_positions = self.positions();
        let other_positions = other.positions();
        self_positions.intersection(&other_positions).count() > 0
    }
}

impl std::fmt::Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}~{}", self.p1, self.p2)
    }
}

impl Brick {
    fn new(p1: Pos, p2: Pos) -> Self {
        Self { p1, p2 }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    // fn adjacent(&self) -> impl Iterator<Item = Pos> + '_ {
    //     [(1, 0), (-1, 0), (0, 1), (0, -1)]
    //         .into_iter()
    //         .map(|(dx, dy)| Pos::new(self.x + dx, self.y + dy))
    // }
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

fn collapse_bricks(bricks: &mut Vec<Brick>) -> HashSet<usize> {
    let mut brick_moved = true;
    let mut moved = HashSet::new();
    while brick_moved {
        brick_moved = false;
        'outer: for bidx in 0..bricks.len() {
            let brick = &bricks[bidx];
            if brick.p1.z == 1 || brick.p2.z == 1 {
                continue;
            }
            let mut test_brick = brick.clone();
            test_brick.p1.z -= 1;
            test_brick.p2.z -= 1;

            for oidx in (0..bricks.len()).filter(|&i| i != bidx) {
                let other_brick = &bricks[oidx];
                if other_brick.intersects(&test_brick) {
                    continue 'outer;
                }
            }

            bricks[bidx] = test_brick;
            brick_moved = true;
            moved.insert(bidx);
        }
    }
    moved
}

fn both_parts(input: &Input) -> (i32, usize) {
    let mut bricks = input.clone();
    // println!("bricks={bricks:?}");
    dbg_bricks(&bricks);

    // println!("bricks[0].positions()={:?}", bricks[0].positions());

    // Collapse
    let _ = collapse_bricks(&mut bricks);
    // println!("bricks={bricks:?}");
    dbg_bricks(&bricks);

    let mut supporting = HashMap::<usize, HashSet<usize>>::new();
    let mut n_supporting = HashMap::<usize, usize>::new();

    for bidx in 0..bricks.len() {
        let brick = &bricks[bidx];
        if brick.p1.z == 1 || brick.p2.z == 1 {
            continue;
        }
        let mut test_brick = brick.clone();
        test_brick.p1.z -= 1;
        test_brick.p2.z -= 1;

        for oidx in (0..bricks.len()).filter(|&i| i != bidx) {
            let other_brick = &bricks[oidx];
            if other_brick.intersects(&test_brick) {
                supporting.entry(oidx).or_default().insert(bidx);
                *n_supporting.entry(bidx).or_default() += 1;
            }
        }
    }

    dbg!(&supporting, &n_supporting);

    let mut p1 = 0;
    let mut non_safe = vec![];
    'outer: for bidx in 0..bricks.len() {
        for oidx in supporting.get(&bidx).cloned().unwrap_or_default() {
            if n_supporting.get(&oidx).cloned().unwrap_or_default() <= 1 {
                non_safe.push(bidx);
                continue 'outer;
            }
        }
        p1 += 1;
    }

    dbg!(&non_safe, non_safe.len());

    let p2 = non_safe
        .par_iter()
        .map(|&ns_idx| {
            let mut bricks = bricks.clone();
            bricks.remove(ns_idx);

            let moved = collapse_bricks(&mut bricks);

            dbg!(&ns_idx, &moved);
            moved.len()
        })
        .sum();

    (p1, p2)
}

fn dbg_bricks(bricks: &Vec<Brick>) {
    println!("Bricks:");
    let mut l = b'A';
    for b in bricks {
        let label = l as char;
        println!("  {label}={b}");

        l += 1;
    }
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

impl FromStr for Brick {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('~');
        let p1 = split.next().unwrap().parse()?;
        let p2 = split.next().unwrap().parse()?;
        Ok(Brick { p1, p2 })
    }
}

impl FromStr for Pos {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        let x = split.next().unwrap().parse()?;
        let y = split.next().unwrap().parse()?;
        let z = split.next().unwrap().parse()?;
        Ok(Pos { x, y, z })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.parse::<Brick>().context("Unable to parse input line"))
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
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9";

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
    //     assert_eq!(both_parts(&as_input(INPUT)?).0, 5);
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(both_parts(&as_input(INPUT)?).1, 7);
        Ok(())
    }
}
