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

#[derive(Debug)]
struct Map {
    rows: Vec<Vec<char>>,
}

impl Map {
    fn at(&self, pos: &Pos) -> Option<char> {
        self.rows
            .get(pos.y as usize)
            .and_then(|row| row.get(pos.x as usize))
            .cloned()
    }

    fn max_x(&self) -> i32 {
        self.rows.len() as i32 - 1
    }

    fn max_y(&self) -> i32 {
        self.rows[0].len() as i32 - 1
    }
}

impl Map {
    fn print(&self, visited: &HashSet<Pos>, curr: Option<&Pos>) {
        let min_x = std::cmp::min(visited.iter().map(|p| p.x).min().unwrap_or_default(), 0);
        let max_x = std::cmp::max(
            visited.iter().map(|p| p.x).max().unwrap_or_default(),
            self.max_x(),
        );
        let min_y = std::cmp::min(visited.iter().map(|p| p.y).min().unwrap_or_default(), 0);
        let max_y = std::cmp::max(
            visited.iter().map(|p| p.y).max().unwrap_or_default(),
            self.max_y(),
        );

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Pos::new(x, y);
                let mut c = self.at(&p).unwrap_or(' ');
                if visited.contains(&p) {
                    c = 'O';
                }
                if Some(&p) == curr {
                    c = 'C';
                }
                print!("{c}");
            }
            println!();
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn adjacent(&self) -> impl Iterator<Item = Pos> + '_ {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|(dx, dy)| Pos::new(self.x + dx, self.y + dy))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn rot_left(&self) -> Self {
        match self {
            Dir::Up => Self::Left,
            Dir::Down => Self::Right,
            Dir::Left => Self::Down,
            Dir::Right => Self::Up,
        }
    }
    fn rot_right(&self) -> Self {
        match self {
            Dir::Up => Self::Right,
            Dir::Down => Self::Left,
            Dir::Left => Self::Up,
            Dir::Right => Self::Down,
        }
    }
}

fn part1(input: &Input) -> usize {
    // dbg!(input);
    input.print(&HashSet::new(), None);
    let start = Pos::new(1, 0);
    let first = Pos::new(1, 1);
    let target = Pos::new(input.max_x() - 1, input.max_y());

    let mut queue = VecDeque::<(Pos, HashSet<Pos>)>::new();
    queue.push_back((first, [start].into_iter().collect()));

    let mut cnt = 0;

    let mut steps = vec![];

    while let Some((p, mut visited)) = queue.pop_front() {
        if p == target {
            println!("Reached {p:?} after {} steps", visited.len());
            input.print(&visited, Some(&p));
            steps.push(visited.len());
            continue;
        }

        // cnt += 1;
        // if cnt > 10 {
        //     break;
        // }

        // println!();
        // println!("p={p:?}, visited.len()={}", visited.len());
        // input.print(&visited, Some(&p));

        let c = input.at(&p);
        let np = match c {
            Some('^') => Some(Pos::new(p.x, p.y - 1)),
            Some('>') => Some(Pos::new(p.x + 1, p.y)),
            Some('v') => Some(Pos::new(p.x, p.y + 1)),
            Some('<') => Some(Pos::new(p.x - 1, p.y)),
            _ => None,
        };
        if let Some(np) = np {
            visited.insert(p);
            if !visited.contains(&np) {
                queue.push_back((np, visited));
            }
            continue;
        }

        let mut nps = Vec::with_capacity(3);
        for np in p.adjacent() {
            let nc = input.at(&np);

            if visited.contains(&np) || nc == Some('#') {
                continue;
            }

            nps.push(np);
        }

        if nps.len() == 1 {
            if let Some(np) = nps.pop() {
                visited.insert(p);
                queue.push_back((np, visited));
                continue;
            }
        }

        for np in nps {
            let mut visited = visited.clone();
            visited.insert(p);
            queue.push_back((np, visited));
        }
    }

    dbg!(&steps);

    steps.into_iter().max().unwrap_or_default()
}

fn part2(input: &Input) -> usize {
    // dbg!(input);
    // input.print(&HashSet::new(), None);
    let start = Pos::new(1, 0);
    let first = Pos::new(1, 1);
    let target = Pos::new(input.max_x() - 1, input.max_y());

    let mut queue = VecDeque::<(Pos, HashSet<Pos>)>::new();
    queue.push_back((first, [start].into_iter().collect()));

    let mut cnt = 0;

    let mut steps = vec![];

    while let Some((p, mut visited)) = queue.pop_front() {
        if p == target {
            println!("Reached {p:?} after {} steps", visited.len());
            // input.print(&visited, Some(&p));
            steps.push(visited.len());
            continue;
        }

        cnt += 1;
        // if cnt > 10 {
        //     break;
        // }

        // println!();
        // println!("p={p:?}, visited.len()={}", visited.len());
        // input.print(&visited, Some(&p));

        let mut nps = Vec::with_capacity(3);
        for np in p.adjacent() {
            let nc = input.at(&np);

            if visited.contains(&np) || nc == Some('#') {
                continue;
            }

            nps.push(np);
        }

        if nps.len() == 1 {
            if let Some(np) = nps.pop() {
                visited.insert(p);
                queue.push_front((np, visited));
                continue;
            }
        }

        for np in nps {
            let mut visited = visited.clone();
            visited.insert(p);
            queue.push_front((np, visited));
        }
    }

    dbg!(&steps, cnt);

    steps.into_iter().max().unwrap_or_default()
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
    let lines = reader.lines().map_while(Result::ok);
    let mut rows = vec![];

    for line in lines {
        let row = line.chars().collect();
        rows.push(row);
    }

    Ok(Map { rows })
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#";

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
    //     assert_eq!(part1(&as_input(INPUT)?), 94);
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 154);
        Ok(())
    }
}
