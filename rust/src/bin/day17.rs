use std::cmp;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
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
    rows: Vec<Vec<u8>>,
}

impl Map {
    fn at(&self, pos: &Pos) -> Option<u8> {
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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

fn part1(input: &Input) -> i32 {
    // dbg!(input);

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Queued {
        heat_loss: i32,
        dir: Dir,
        pos: Pos,
        dir_count: i32,
    }

    let mut visited = HashMap::<Pos, HashMap<(Dir, i32), i32>>::new();
    // let mut queue = VecDeque::new();
    let mut queue = BinaryHeap::<Reverse<Queued>>::new();
    let start = Pos::new(0, 0);
    let dest = Pos::new(input.max_x(), input.max_y());

    // queue.push_back((Pos::new(1, 0), Dir::Right, 1, 0));
    // queue.push_back((Pos::new(0, 1), Dir::Down, 1, 0));
    queue.push(Reverse(Queued {
        pos: Pos::new(1, 0),
        dir: Dir::Right,
        dir_count: 1,
        heat_loss: 0,
    }));
    queue.push(Reverse(Queued {
        pos: Pos::new(0, 1),
        dir: Dir::Down,
        dir_count: 1,
        heat_loss: 0,
    }));

    let mut cnt = 0;

    // while let Some((pos, dir, dir_count, heat_loss)) = queue.pop_back() {
    while let Some(Reverse(Queued {
        heat_loss,
        dir,
        pos,
        dir_count,
    })) = queue.pop()
    {
        // println!("heat_loss={heat_loss}, dir={dir:?}, pos={pos:?}, dir_count={dir_count}");
        let Some(pos_heat_loss) = input.at(&pos) else {
            // println!("continue at {pos:?}");
            continue;
        };

        let actual_heat_loss = heat_loss + pos_heat_loss as i32;

        // cnt += 1;
        // if cnt > 100 {
        //     println!("cnt > 100!");
        //     break;
        // }

        let entry = visited.entry(pos).or_default();
        if let Some(hl) = entry.get(&(dir, dir_count)) {
            if *hl < actual_heat_loss {
                continue;
            }
        }
        entry.insert((dir, dir_count), actual_heat_loss);

        if pos == dest {
            println!("Rached dest!");
            break;
        }

        if dir_count < 3 {
            let (dx, dy) = match dir {
                Dir::Up => (0, -1),
                Dir::Down => (0, 1),
                Dir::Left => (-1, 0),
                Dir::Right => (1, 0),
            };
            let next_pos = Pos::new(pos.x + dx, pos.y + dy);

            // queue.push_back((next_pos, dir, dir_count + 1, actual_heat_loss));
            queue.push(Reverse(Queued {
                pos: next_pos,
                dir: dir,
                dir_count: dir_count + 1,
                heat_loss: actual_heat_loss,
            }));
        }

        for dir in [dir.rot_left(), dir.rot_right()] {
            let (dx, dy) = match dir {
                Dir::Up => (0, -1),
                Dir::Down => (0, 1),
                Dir::Left => (-1, 0),
                Dir::Right => (1, 0),
            };
            let next_pos = Pos::new(pos.x + dx, pos.y + dy);

            // queue.push_back((next_pos, dir, 1, actual_heat_loss));
            queue.push(Reverse(Queued {
                pos: next_pos,
                dir: dir,
                dir_count: 1,
                heat_loss: actual_heat_loss,
            }));
        }
    }

    // dbg!(&visited);

    dbg!(&visited.get(&Pos::new(0, 0)));
    dbg!(&visited.get(&dest));

    let foo = visited
        .get(&Pos::new(input.max_x(), input.max_y()))
        .and_then(|vis| vis.values().min())
        .cloned()
        .unwrap_or_default();

    // 972 too high
    // 971 wrong
    // 970 wrong
    // 969 wrong
    // 968 wrong
    // 967 wrong
    // 956 correct
    foo // + input.at(&dest).unwrap_or_default() as i32
}

fn part2(input: &Input) -> i32 {
    // dbg!(input);

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Queued {
        heat_loss: i32,
        dir: Dir,
        pos: Pos,
        dir_count: i32,
    }

    let mut visited = HashMap::<Pos, HashMap<(Dir, i32), i32>>::new();
    // let mut queue = VecDeque::new();
    let mut queue = BinaryHeap::<Reverse<Queued>>::new();
    let start = Pos::new(0, 0);
    let dest = Pos::new(input.max_x(), input.max_y());

    // queue.push_back((Pos::new(1, 0), Dir::Right, 1, 0));
    // queue.push_back((Pos::new(0, 1), Dir::Down, 1, 0));
    queue.push(Reverse(Queued {
        pos: Pos::new(1, 0),
        dir: Dir::Right,
        dir_count: 1,
        heat_loss: 0,
    }));
    queue.push(Reverse(Queued {
        pos: Pos::new(0, 1),
        dir: Dir::Down,
        dir_count: 1,
        heat_loss: 0,
    }));

    let mut cnt = 0;

    // while let Some((pos, dir, dir_count, heat_loss)) = queue.pop_back() {
    while let Some(Reverse(Queued {
        heat_loss,
        dir,
        pos,
        dir_count,
    })) = queue.pop()
    {
        // println!("heat_loss={heat_loss}, dir={dir:?}, pos={pos:?}, dir_count={dir_count}");
        let Some(pos_heat_loss) = input.at(&pos) else {
            // println!("continue at {pos:?}");
            continue;
        };

        let actual_heat_loss = heat_loss + pos_heat_loss as i32;

        // cnt += 1;
        // if cnt > 100 {
        //     println!("cnt > 100!");
        //     break;
        // }

        let entry = visited.entry(pos).or_default();
        if let Some(hl) = entry.get(&(dir, dir_count)) {
            if *hl < actual_heat_loss {
                continue;
            }
        }
        entry.insert((dir, dir_count), actual_heat_loss);

        if pos == dest && dir_count >= 4 {
            println!("Rached dest!");
            break;
        }

        if dir_count < 10 {
            let (dx, dy) = match dir {
                Dir::Up => (0, -1),
                Dir::Down => (0, 1),
                Dir::Left => (-1, 0),
                Dir::Right => (1, 0),
            };
            let next_pos = Pos::new(pos.x + dx, pos.y + dy);

            // queue.push_back((next_pos, dir, dir_count + 1, actual_heat_loss));
            queue.push(Reverse(Queued {
                pos: next_pos,
                dir: dir,
                dir_count: dir_count + 1,
                heat_loss: actual_heat_loss,
            }));
        }

        if dir_count >= 4 {
            for dir in [dir.rot_left(), dir.rot_right()] {
                let (dx, dy) = match dir {
                    Dir::Up => (0, -1),
                    Dir::Down => (0, 1),
                    Dir::Left => (-1, 0),
                    Dir::Right => (1, 0),
                };
                let next_pos = Pos::new(pos.x + dx, pos.y + dy);

                // queue.push_back((next_pos, dir, 1, actual_heat_loss));
                queue.push(Reverse(Queued {
                    pos: next_pos,
                    dir: dir,
                    dir_count: 1,
                    heat_loss: actual_heat_loss,
                }));
            }
        }
    }

    // dbg!(&visited);

    dbg!(&visited.get(&Pos::new(0, 0)));
    dbg!(&visited.get(&dest));

    let foo = visited
        .get(&Pos::new(input.max_x(), input.max_y()))
        .and_then(|vis| {
            vis.iter()
                .filter(|((_, dir_count), _)| *dir_count >= 4)
                .map(|(_, hl)| hl)
                .min()
        })
        .cloned()
        .unwrap_or_default();

    foo
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
        let row = line.chars().map(|c| (c as u8 - '0' as u8)).collect();
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
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533";

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
    //     assert_eq!(part1(&as_input(INPUT)?), 102);
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 94);
        Ok(())
    }
}
