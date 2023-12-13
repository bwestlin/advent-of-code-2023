use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{Context, Result};

use utils::measure;

type Input = Diagram;

#[derive(Debug)]
struct Diagram {
    pipes: Vec<Vec<char>>,
}

impl Diagram {
    fn start_pos(&self) -> Pos {
        for y in 0..self.pipes.len() {
            for x in 0..self.pipes[y].len() {
                if self.pipes[y][x] == 'S' {
                    return Pos::new(x as i32, y as i32);
                }
            }
        }
        unreachable!()
    }

    fn at(&self, x: i32, y: i32) -> char {
        self.pipes[y as usize][x as usize]
    }

    fn start_adjacent(&self, start_pos: &Pos) -> (Pos, Pos) {
        let mut adj = vec![];
        for (i, p) in start_pos
            .adjacent()
            .enumerate()
            .filter(|(_, p)| p.x >= 0 && p.y >= 0)
        {
            let c = self.at(p.x, p.y);
            match (i, c) {
                // Right
                (0, '-' | 'J' | '7') => adj.push(p),
                // Left
                (1, '-' | 'L' | 'F') => adj.push(p),
                // Down
                (2, '|' | 'J' | 'L') => adj.push(p),
                // Up
                (3, '|' | '7' | 'F') => adj.push(p),
                _ => {}
            }
        }

        assert_eq!(2, adj.len());
        (adj[0], adj[1])
    }

    fn connections_at(&self, x: i32, y: i32) -> (Pos, Pos) {
        let c = self.at(x, y);

        match c {
            '|' => (Pos::new(x, y - 1), Pos::new(x, y + 1)),
            '-' => (Pos::new(x - 1, y), Pos::new(x + 1, y)),
            'L' => (Pos::new(x, y - 1), Pos::new(x + 1, y)),
            'J' => (Pos::new(x, y - 1), Pos::new(x - 1, y)),
            '7' => (Pos::new(x, y + 1), Pos::new(x - 1, y)),
            'F' => (Pos::new(x, y + 1), Pos::new(x + 1, y)),
            _ => unreachable!(),
        }
    }

    fn within(&self, p: &Pos) -> bool {
        p.x >= 0 && p.x < self.pipes[0].len() as i32 && p.y >= 0 && p.y < self.pipes.len() as i32
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

    fn adjacent(&self) -> impl Iterator<Item = Pos> + '_ {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|(dx, dy)| Pos::new(self.x + dx, self.y + dy))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn left_of<'a>(&self, p: &'a Pos, pipe: char) -> impl Iterator<Item = Pos> + 'a {
        let deltas = match (self, pipe) {
            (Dir::Left, '-') => vec![(0, 1)],
            (Dir::Left, 'L') => vec![(0, 1), (-1, 1), (-1, 0)],
            (Dir::Left, 'F') => vec![],
            (Dir::Right, '-') => vec![(0, -1)],
            (Dir::Right, 'J') => vec![],
            (Dir::Right, '7') => vec![(0, -1), (1, -1), (1, 0)],
            (Dir::Up, '|') => vec![(-1, 0)],
            (Dir::Up, '7') => vec![],
            (Dir::Up, 'F') => vec![(-1, 0), (-1, -1), (0, -1)],
            (Dir::Down, '|') => vec![(1, 0)],
            (Dir::Down, 'J') => vec![(1, 0), (1, 1), (0, 1)],
            (Dir::Down, 'L') => vec![],
            _ => unreachable!(),
        };

        deltas
            .into_iter()
            .map(|(dx, dy)| Pos::new(p.x + dx, p.y + dy))
    }
}

fn both_parts(input: &Input) -> (usize, usize) {
    let start = input.start_pos();
    let start_adj = input.start_adjacent(&start);

    let mut dists = HashMap::<Pos, usize>::new();
    dists.insert(start, 0);
    dists.insert(start_adj.0, 1);
    dists.insert(start_adj.1, 1);

    let mut queue = VecDeque::new();
    queue.push_back((start_adj.0, 1));
    queue.push_back((start_adj.1, 1));

    while let Some((pos, dist)) = queue.pop_front() {
        let (a, b) = input.connections_at(pos.x, pos.y);

        for p in [a, b] {
            if dists.contains_key(&p) {
                continue;
            }

            dists.insert(p, dist + 1);
            queue.push_back((p, dist + 1));
        }
    }

    let p1 = dists.values().max().cloned().unwrap_or_default();

    let p2 = [start_adj.0, start_adj.1]
        .into_iter()
        .map(|start_adj| {
            let mut prev = start;
            let mut curr = start_adj;
            let mut enclosed = HashSet::new();

            while curr != start {
                let dir = match (curr.x - prev.x, curr.y - prev.y) {
                    (-1, 0) => Dir::Left,
                    (1, 0) => Dir::Right,
                    (0, -1) => Dir::Up,
                    (0, 1) => Dir::Down,
                    _ => unreachable!(),
                };

                let pipe = input.at(curr.x, curr.y);
                for l in dir.left_of(&curr, pipe) {
                    if !dists.contains_key(&l) {
                        enclosed.insert(l);
                    }
                }

                let (a, b) = input.connections_at(curr.x, curr.y);
                let next = if a == prev { b } else { a };

                prev = curr;
                curr = next;
            }

            // Expand
            let starting_points = enclosed.iter().cloned().collect::<Vec<_>>();
            let mut visited = HashSet::new();
            for sp in starting_points {
                let mut queue = VecDeque::new();
                queue.push_back(sp);

                while let Some(p) = queue.pop_front() {
                    if visited.contains(&p) || dists.contains_key(&p) || !input.within(&p) {
                        continue;
                    }

                    enclosed.insert(p);

                    for adj in p.adjacent() {
                        queue.push_back(adj);
                    }

                    visited.insert(p);
                }
            }

            enclosed.len()
        })
        .min()
        .unwrap_or_default();

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

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    let pipes = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    Ok(Diagram { pipes })
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...";

    const INPUT2: &str = "
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........";

    const INPUT3: &str = "
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...";

    const INPUT4: &str = "
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L";

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
        assert_eq!(both_parts(&as_input(INPUT)?).0, 8);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(both_parts(&as_input(INPUT2)?).1, 4);
        assert_eq!(both_parts(&as_input(INPUT3)?).1, 8);
        assert_eq!(both_parts(&as_input(INPUT4)?).1, 10);
        Ok(())
    }
}
