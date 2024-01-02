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
    rocks: HashSet<Pos>,
    start: Pos,
    max_x: usize,
    max_y: usize,
}

impl Map {
    fn rock_at_inf(&self, mut x: i32, mut y: i32) -> bool {
        let w = (self.max_x + 1) as i32;
        let h = (self.max_y + 1) as i32;

        if x < 0 {
            x += ((x * -1) / w) * w + w;
        }
        if y < 0 {
            y += ((y * -1) / h) * h + w;
        }

        let p = Pos::new(((x) % w) as i32, ((y) % h) as i32);
        self.rocks.contains(&p)
    }

    fn print(&self, factor: usize) {
        for y in 0..=((self.max_y + 1) * factor - 1) {
            for x in 0..=((self.max_x + 1) * factor - 1) {
                let c = if self.rock_at_inf(x as i32, y as i32) {
                    '#'
                } else if Pos::new(x as i32, y as i32) == self.start {
                    'S'
                } else {
                    '.'
                };
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

fn reachable_garden_plots(map: &Map, steps: usize) -> usize {
    let mut positions = HashSet::new();
    positions.insert(map.start);

    let mut last = 0;
    for s in 0..steps {
        let mut next_positions = HashSet::new();

        for p in positions.iter() {
            for np in p.adjacent() {
                if map.rock_at_inf(np.x, np.y) {
                    continue;
                }
                next_positions.insert(np);
            }
        }

        std::mem::swap(&mut positions, &mut next_positions);

        let reachable = positions.len();

        let diff = reachable - last;
        println!(
            "steps={s}, reachable={reachable}, (diff-steps)={}, diff={diff}",
            diff as i32 - s as i32
        );
        last = reachable;
    }

    // dbg!(&positions);
    // dbg!(positions.len());

    positions.len()
}

fn part1(input: &Input) -> usize {
    // dbg!(input);s
    input.print(1);

    #[cfg(not(test))]
    let steps = 64;
    #[cfg(test)]
    let steps = 6;

    reachable_garden_plots(input, steps)
}

fn reachable_garden_plots_p2(map: &Map, steps: usize) -> i64 {
    let mut positions = HashSet::new();
    positions.insert(map.start);

    let mut reachable_history = vec![];
    let mut stepdiff_history = vec![];

    let mut last = 0;

    let mut step_accs = vec![];
    let mut step_last_stepdiff = vec![];
    let mut step_last_reachable = 0;
    let mut step_offset = 0;
    let mut step_len = 0;

    'outer: for s in 0..steps {
        let mut next_positions = HashSet::new();

        for p in positions.iter() {
            for np in p.adjacent() {
                if map.rock_at_inf(np.x, np.y) {
                    continue;
                }
                next_positions.insert(np);
            }
        }

        std::mem::swap(&mut positions, &mut next_positions);

        let reachable = positions.len() as i64;

        let diff = reachable - last;
        let stepdiff = (diff as i64) - (s as i64);

        if s > 50 {
            'checking: for d in 0..=1 {
                for ss in 5..(stepdiff_history.len() / 3 - 1) {
                    // println!("s={s} ss={ss} d={d} stepdiff={stepdiff}");
                    if stepdiff_history[s - ss] == stepdiff - d
                        && stepdiff_history[s - (ss * 2)] == stepdiff_history[s - ss] - d
                        && stepdiff_history[s - (ss * 3)] == stepdiff_history[s - (ss * 2)] - d
                    // && stepdiff_history[s - (ss * 4)] == stepdiff_history[s - (ss * 3)] - d
                    {
                        println!(
                            "found! steps={s}, reachable={reachable}, stepdiff={stepdiff} ss={ss} d={d}"
                        );
                        // dbg!(&stepdiff_history);

                        let mut accs = vec![d];

                        for i in (stepdiff_history.len() - ss + 1)..stepdiff_history.len() {
                            accs.push(stepdiff_history[i] - stepdiff_history[i - ss]);
                        }

                        // Validate
                        for i in 0..accs.len() {
                            let ai = stepdiff_history.len() - ss + i;
                            let bi = stepdiff_history.len() - (ss * 2) + i;
                            let ci = stepdiff_history.len() - (ss * 3) + i;
                            let d1 = stepdiff_history[ai] - stepdiff_history[bi];
                            let d2 = stepdiff_history[bi] - stepdiff_history[ci];

                            println!(
                                "ai={ai} bi={bi} ci={ci} i={i} d={d} accs[i]={} d1={d1} d2={d2}",
                                accs[i]
                            );

                            if d1 != d2 {
                                println!("Not valid, continuing!");
                                continue 'checking;
                            }
                        }

                        step_last_reachable = reachable_history[stepdiff_history.len() - 1];
                        for i in (stepdiff_history.len() - ss)..stepdiff_history.len() {
                            step_last_stepdiff.push(stepdiff_history[i]);
                        }

                        dbg!(&accs, accs.len());

                        std::mem::swap(&mut accs, &mut step_accs);
                        step_offset = s;
                        step_len = ss;

                        assert_eq!(step_last_stepdiff.len(), step_len);

                        break 'outer;
                    }
                }
            }
        }
        println!("steps={s}");

        // let occurrances_idx = stepdiff_history
        //     .iter()
        //     .enumerate()
        //     .filter(|&(_, sd)| *sd == stepdiff)
        //     .map(|(i, _)| i)
        //     .collect::<Vec<_>>();
        // let occurrances_idx_delta = occurrances_idx
        //     .windows(2)
        //     .map(|w| w[1] - w[0])
        //     .collect::<Vec<_>>();
        // println!("steps={s}, reachable={reachable}, stepdiff={stepdiff}, diff={diff}, occurrances_idx={occurrances_idx:?}, occurrances_idx_delta={occurrances_idx_delta:?}");

        last = reachable;
        reachable_history.push(reachable);
        stepdiff_history.push(stepdiff);
    }

    // dbg!(&positions);
    // dbg!(positions.len());

    dbg!(
        &step_offset,
        &step_accs,
        &step_last_stepdiff,
        &step_last_reachable
    );

    for _ in 0.. {
        for ss in 0..step_len {
            let s = (step_offset + ss) as i64;
            step_last_stepdiff[ss] += step_accs[ss];
            step_last_reachable += s + step_last_stepdiff[ss];

            let stepdiff = step_last_stepdiff[ss];
            let reachable = step_last_reachable;
            let diff = s + step_last_stepdiff[ss];

            // println!("steps={s}, reachable={reachable}, stepdiff={stepdiff}, diff={diff}");
            if s == (steps - 1) as i64 {
                println!("steps={s}, reachable={reachable}, stepdiff={stepdiff}, diff={diff}");
                println!("Found!");
                return reachable;
            }
        }

        step_offset += step_len;
        // println!();
    }

    0
}

fn part2(input: &Input) -> i64 {
    // dbg!(input);
    input.print(2);

    // let mut last = 0;
    // for steps in 1..1000 {
    //     let reachable = reachable_garden_plots(input, steps);
    //     let diff = reachable - last;
    //     println!("steps={steps}, reachable={reachable}, diff={diff}");
    //     last = reachable;
    // }

    reachable_garden_plots_p2(input, 26501365)
}

// fn both_parts(input: &Input) -> (i32, i32) {
//     dbg!(input);
//     (0, 0)
// }

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        // let (part1, part2) = both_parts(&input);
        // println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
        Ok(())
    })
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    let lines = reader.lines().map_while(Result::ok);
    let mut rocks = HashSet::new();
    let mut start = Pos::new(0, 0);

    let mut max_x = 0usize;
    let mut max_y = 0usize;

    for (y, line) in lines.into_iter().enumerate() {
        max_y = y;
        let y = y as i32;

        for (x, c) in line.chars().enumerate() {
            max_x = x;
            let x = x as i32;
            if c == '#' {
                rocks.insert(Pos::new(x, y));
            } else if c == 'S' {
                start = Pos::new(x, y);
            }
        }
    }

    // let max_x = rocks.iter().map(|p| p.x).max().unwrap_or_default() as usize + 1;
    // let max_y = rocks.iter().map(|p| p.y).max().unwrap_or_default() as usize + 1;

    Ok(Map {
        rocks,
        start,
        max_x,
        max_y,
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
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........";

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
    //     assert_eq!(part1(&as_input(INPUT)?), 16);
    //     Ok(())
    // }

    // #[test]
    // fn test_reachable_garden_plots() -> Result<()> {
    //     let map = as_input(INPUT)?;
    //     assert_eq!(reachable_garden_plots(&map, 6), 16);
    //     assert_eq!(reachable_garden_plots(&map, 10), 50);
    //     assert_eq!(reachable_garden_plots(&map, 50), 1594);
    //     assert_eq!(reachable_garden_plots(&map, 100), 6536);
    //     // assert_eq!(reachable_garden_plots(&map, 500), 167004);
    //     // assert_eq!(reachable_garden_plots(&map, 1000), 668697);
    //     Ok(())
    // }

    // #[test]
    // fn test_reachable_garden_plots_p2() -> Result<()> {
    //     let map = as_input(INPUT)?;
    //     assert_eq!(reachable_garden_plots_p2(&map, 500), 167004);
    //     assert_eq!(reachable_garden_plots_p2(&map, 1000), 668697);
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 16733044);
        Ok(())
    }
}
