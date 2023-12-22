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

type Input = Contraption;

#[derive(Debug)]
struct Contraption {
    elements: HashMap<Pos, char>,
}

impl Contraption {
    fn energized(&self, beam: (Pos, Dir)) -> HashSet<Pos> {
        let mut visited = HashSet::new();
        let max_x = self.max_x();
        let max_y = self.max_y();

        let mut queue = VecDeque::new();
        queue.push_back(beam);

        // let mut cnt = 0;

        while let Some((pos, dir)) = queue.pop_front() {
            // cnt += 1;
            // if cnt > 200 {
            //     break;
            // }

            if pos.x < 0
                || pos.x > max_x
                || pos.y < 0
                || pos.y > max_y
                || visited.contains(&(pos, dir))
            {
                continue;
            }
            visited.insert((pos, dir));

            // let (dx, dy) = match dir {
            //     Dir::Up => (0, -1),
            //     Dir::Down => (0, 1),
            //     Dir::Left => (-1, 0),
            //     Dir::Right => (1, 0),
            // };

            // let next_pos = Pos::new(pos.x + dx, pos.y + dy);
            // if next_pos.x < 0 || next_pos.x > max_x || next_pos.y < 0 || next_pos.y > max_y {
            //     continue;
            // }

            match self.elements.get(&pos).cloned() {
                Some('/') => {
                    let (dx, dy, next_dir) = match dir {
                        Dir::Up => (1, 0, Dir::Right),
                        Dir::Down => (-1, 0, Dir::Left),
                        Dir::Left => (0, 1, Dir::Down),
                        Dir::Right => (0, -1, Dir::Up),
                    };
                    queue.push_back((Pos::new(pos.x + dx, pos.y + dy), next_dir));
                }
                Some('\\') => {
                    let (dx, dy, next_dir) = match dir {
                        Dir::Up => (-1, 0, Dir::Left),
                        Dir::Down => (1, 0, Dir::Right),
                        Dir::Left => (0, -1, Dir::Up),
                        Dir::Right => (0, 1, Dir::Down),
                    };
                    queue.push_back((Pos::new(pos.x + dx, pos.y + dy), next_dir));
                }
                Some('|') => {
                    let next = match dir {
                        Dir::Up => vec![(0, -1, Dir::Up)],
                        Dir::Down => vec![(0, 1, Dir::Down)],
                        Dir::Left | Dir::Right => vec![(0, -1, Dir::Up), (0, 1, Dir::Down)],
                    };

                    for (dx, dy, next_dir) in next {
                        queue.push_back((Pos::new(pos.x + dx, pos.y + dy), next_dir));
                    }
                }
                Some('-') => {
                    let next = match dir {
                        Dir::Left => vec![(-1, 0, Dir::Left)],
                        Dir::Right => vec![(1, 0, Dir::Right)],
                        Dir::Up | Dir::Down => vec![(-1, 0, Dir::Left), (1, 0, Dir::Right)],
                    };

                    for (dx, dy, next_dir) in next {
                        queue.push_back((Pos::new(pos.x + dx, pos.y + dy), next_dir));
                    }
                }
                _ => {
                    let (dx, dy) = match dir {
                        Dir::Up => (0, -1),
                        Dir::Down => (0, 1),
                        Dir::Left => (-1, 0),
                        Dir::Right => (1, 0),
                    };
                    let next_pos = Pos::new(pos.x + dx, pos.y + dy);
                    queue.push_back((next_pos, dir));
                }
            }
        }

        visited.into_iter().map(|(p, _)| p).collect()
    }

    fn max_x(&self) -> i32 {
        self.elements.keys().map(|p| p.x).max().unwrap_or_default()
    }

    fn max_y(&self) -> i32 {
        self.elements.keys().map(|p| p.y).max().unwrap_or_default()
    }

    fn print(&self) {
        let max_x = self.max_x();
        let max_y = self.max_y();

        for y in 0..=max_y {
            for x in 0..=max_x {
                let pos = Pos::new(x, y);
                let c = self.elements.get(&pos).unwrap_or(&'.');
                print!("{c}");
            }
            println!();
        }
    }

    fn print_energized(&self, energized: &HashSet<Pos>) {
        let max_x = self.max_x();
        let max_y = self.max_y();

        for y in 0..=max_y {
            for x in 0..=max_x {
                let pos = Pos::new(x, y);
                let c = if energized.contains(&pos) { '#' } else { '.' };
                print!("{c}");
            }
            println!();
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn part1(input: &Input) -> usize {
    // dbg!(input);
    input.print();
    let energized = input.energized((Pos::new(0, 0), Dir::Right));
    // dbg!(&energized);
    input.print_energized(&energized);
    energized.len()
}

fn part2(input: &Input) -> usize {
    // dbg!(input);
    input.print();

    let max_x = input.max_x();
    let max_y = input.max_y();

    let north = (0..=max_x).map(|x| (Pos::new(x, 0), Dir::Down));
    let south = (0..=max_x).map(|x| (Pos::new(x, max_y), Dir::Up));
    let east = (0..=max_y).map(|y| (Pos::new(0, y), Dir::Right));
    let west = (0..=max_y).map(|y| (Pos::new(max_x, y), Dir::Left));

    let mut max = 0;
    for beam in north.chain(south).chain(east).chain(west) {
        let energized = input.energized(beam);
        if energized.len() > max {
            max = energized.len();
        }
    }

    // dbg!(&energized);
    // input.print_energized(&energized);
    // energized.len()
    max
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
    let mut elements = HashMap::new();

    for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                elements.insert(Pos::new(x as i32, y as i32), c);
            }
        }
    }

    Ok(Contraption { elements })
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....";

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
    //     assert_eq!(part1(&as_input(INPUT)?), 46);
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 51);
        Ok(())
    }
}
