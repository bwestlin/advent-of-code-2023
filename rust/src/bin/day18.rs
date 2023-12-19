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

type Input = Vec<DigPlan>;

#[derive(Debug)]
struct DigPlan {
    dir: Dir,
    meters: i32,
    color: String,
}

impl DigPlan {
    fn converted(&self) -> Self {
        let dir = match &self.color[6..] {
            "0" => Dir::Right,
            "1" => Dir::Down,
            "2" => Dir::Left,
            "3" => Dir::Up,
            __ => unreachable!(),
        };
        let meters = i32::from_str_radix(&self.color[1..=5], 16).unwrap();
        let color = "".to_string();
        Self { dir, meters, color }
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
    Right,
    Left,
}

impl Dir {
    fn side_deltas(&self) -> [(i32, i32); 2] {
        match self {
            Dir::Up => [(-1, 0), (1, 0)],
            Dir::Down => [(1, 0), (-1, 0)],
            Dir::Right => [(0, -1), (0, 1)],
            Dir::Left => [(0, 1), (0, -1)],
        }
    }
}

fn part1(input: &Input) -> usize {
    dbg!(input);

    let mut ground = HashSet::new();
    let mut pos = Pos::new(0, 0);
    ground.insert(pos);
    let mut sides = [HashSet::new(), HashSet::new()];

    for DigPlan { dir, meters, color } in input {
        for (i, (dx, dy)) in dir.side_deltas().into_iter().enumerate() {
            let p = Pos::new(pos.x + dx, pos.y + dy);
            if !ground.contains(&p) {
                sides[i].insert(p);
            }
        }

        let (dx, dy) = match dir {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        };

        for _ in 0..*meters {
            pos = Pos::new(pos.x + dx, pos.y + dy);
            ground.insert(pos);
            for (i, (dx, dy)) in dir.side_deltas().into_iter().enumerate() {
                sides[i].remove(&pos);
                let p = Pos::new(pos.x + dx, pos.y + dy);
                if !ground.contains(&p) {
                    sides[i].insert(p);
                }
            }
        }
    }

    dbg!(&ground);
    dbg!(&sides);

    let min_x = ground.iter().map(|p| p.x).min().unwrap_or_default();
    let min_y = ground.iter().map(|p| p.y).min().unwrap_or_default();
    let max_x = ground.iter().map(|p| p.x).max().unwrap_or_default();
    let max_y = ground.iter().map(|p| p.y).max().unwrap_or_default();

    /*
    for &sp in &sides[1] {
        let mut queue = VecDeque::new();
        queue.push_back(sp);

        while let Some(p) = queue.pop_front() {
            if ground.contains(&p) || (p.x < min_x || p.x > max_x || p.y < min_y || p.y > max_y) {
                continue;
            }

            ground.insert(p);

            for adj in p.adjacent() {
                queue.push_back(adj);
            }
        }
    }
    */

    let min_x = min_x - 2;
    let min_y = min_y - 2;
    let max_x = max_x + 2;
    let max_y = max_y + 2;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pos = Pos::new(x, y);
            let mut c = if ground.iter().any(|&p| p == pos) {
                '#'
            } else {
                '.'
            };

            // if let Some(_) = sides[0].get(&pos) {
            //     c = '0';
            // }
            // if let Some(_) = sides[1].get(&pos) {
            //     c = '1';
            // }

            print!("{c}");
        }
        println!();
    }

    ground.len()
}

fn part2(input: &Input) -> usize {
    // dbg!(input);

    let mut pos = Pos::new(0, 0);

    let mut verticals = vec![];

    for dp in input {
        let converted = dp.converted();
        println!("dp={dp:?}, converted={converted:?}");

        let DigPlan { dir, meters, color } = converted;

        let (dx, dy) = match dir {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        };

        let next_pos = Pos::new(pos.x + (dx * meters), pos.y + (dy * meters));

        if pos.x == next_pos.x {
            verticals.push((
                pos.x,
                (if pos.y < next_pos.y {
                    (pos.y, next_pos.y)
                } else {
                    (next_pos.y, pos.y)
                }),
            ));
        }

        pos = next_pos;
    }

    verticals.sort_by_key(|(x, _)| *x);

    // dbg!(&verticals);

    let min_x = verticals
        .iter()
        .map(|(x, _)| x)
        .min()
        .cloned()
        .unwrap_or_default();
    let max_x = verticals
        .iter()
        .map(|(x, _)| x)
        .max()
        .cloned()
        .unwrap_or_default();
    let min_y = verticals
        .iter()
        .map(|(_, (y, _))| y)
        .min()
        .cloned()
        .unwrap_or_default();
    let max_y = verticals
        .iter()
        .map(|(_, (_, y))| y)
        .max()
        .cloned()
        .unwrap_or_default();

    dbg!(&min_y, &max_y);

    let mut expected = [7, 7, 7, 5, 5, 7, 5, 7, 6, 6]
        .into_iter()
        .collect::<VecDeque<_>>();

    let mut tot_n_inside = 0;
    for y in min_y..=max_y {
        // println!("--------------------------------------------");
        // println!("y={y}");
        let mut n_inside = 0;

        let mut inside = false;
        let mut was_on_edge = false;

        let intersecting = verticals
            .iter()
            .filter(|(_, (y1, y2))| y >= *y1 && y <= *y2)
            .collect::<Vec<_>>();

        for pair in intersecting.windows(2) {
            let &(lx, (ly1, ly2)) = pair[0];
            let &(rx, (ry1, ry2)) = pair[1];

            if ly1 == y && ry1 == y && !was_on_edge {
                /*
                Cases:

                Outside:
                   O       O
                ->O#########O
                  O#I     I#O

                Inside_
                   I       I
                ->I#########I
                  I#O     O#I
                */

                // println!("  1) ly1 == y && ry1 == y, lx={lx}, rx={rx}");

                let mut add = rx - lx + 1;
                if inside {
                    add -= 2;
                }

                // println!("    += {}", add);
                n_inside += add;
                inside = !inside;
                was_on_edge = true;
            } else if ly2 == y && ry2 == y && !was_on_edge {
                /*
                Cases:

                Outside:
                  O#I     I#O
                ->O#########O

                Inside_
                  I#O     O#I
                ->I#########I
                */

                // println!("  2) ly2 == y && ry2 == y, lx={lx}, rx={rx}, inside={inside}");

                let mut add = rx - lx + 1;
                if inside {
                    add -= 2;
                }

                // println!("    += {}", add);
                n_inside += add;
                inside = !inside;
                was_on_edge = true;
            } else if ly1 == y && ry2 == y && !was_on_edge {
                /*
                Cases:

                Outside:
                          O#I
                ->O#########I
                  O#

                Inside:
                          I#O
                ->I#########O
                  I#

                Inside between
                   #   #
                ->O##II##O
                    #   #
                */

                // println!("  3) ly1 == y && ry2 == y, lx={lx}, rx={rx}, inside={inside}");
                let add = rx - lx;
                // println!("    += {}", add);
                n_inside += add;
                was_on_edge = true;
            } else if ly2 == y && ry1 == y && !was_on_edge {
                /*
                Cases:

                Outside:
                   #
                ->O#########I
                           #

                Inside_
                   #
                ->I#########O
                           #

                Inside between
                    #   #
                ->O##II##O
                   #   #
                */

                // println!("  4) ly2 == y && ry1 == y, lx={lx}, rx={rx}, inside={inside}");
                let add = rx - lx;
                // println!("    += {}", add);
                n_inside += add;
                was_on_edge = true;
            } else {
                /*
                Cases:

                Outside:
                ->O#IIIIIII#O

                Inside_
                ->I#OOOOOOO#I
                */

                // println!("  5) else, inside={inside}, lx={lx}, rx={rx}");
                inside = !inside;

                if inside {
                    // println!("    += {}", rx - lx + 1);
                    n_inside += rx - lx + 1;
                }
                was_on_edge = false;
            }
        }

        // if n_inside > 0 && n_toggles > 1 {
        //     n_inside -= n_toggles - 1;
        // }

        // let mut last: Option<(i32, (i32, i32))> = None;
        // for vi in 0..verticals.len() {
        //     let &(x, (y1, y2)) = &verticals[vi];
        //     // let last = verticals.get(vi - 1);
        //     let next = verticals.get(vi + 1);

        //     if y >= y1 && y <= y2 {
        //         match last {
        //             Some((_, (ly1, ly2))) if ly2 == y1 => {
        //                 continue;
        //             }
        //             _ => {}
        //         }
        //         match next {
        //             Some(&(_, (ny1, ny2))) if (/*y2 == ny1 ||*/y1 == ny2) && inside => {
        //                 continue;
        //             }
        //             _ => {}
        //         }
        //         println!("  x={x}, last={last:?}, inside={inside}, y1={y1}, y2={y2}");

        //         if inside {
        //             if let Some((last_x, _)) = last {
        //                 n_inside += (x as usize - last_x as usize + 1) as usize;
        //             }
        //         } else {
        //             last = Some((x, (y1, y2)));
        //         }
        //         inside = !inside;
        //     }
        // }

        // println!("  n_inside={n_inside}");
        // let e = expected.pop_front();

        // if e != Some(n_inside) {
        //     println!("  !!!!!!!!!!!!!!!!!!!!!!!! {n_inside} != {e:?}");
        // }

        tot_n_inside += n_inside as usize;
    }

    tot_n_inside
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

impl FromStr for DigPlan {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let dir = split.next().unwrap().parse().unwrap();
        let meters = split.next().unwrap().parse().unwrap();
        let color = split
            .next()
            .unwrap()
            .chars()
            .skip(1)
            .take_while(|&c| c != ')')
            .collect::<String>();

        Ok(DigPlan { dir, meters, color })
    }
}

impl FromStr for Dir {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => anyhow::bail!("Unknown dir: {s}"),
        };
        Ok(dir)
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.parse::<DigPlan>()
                .context("Unable to parse input line")
        })
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
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)";

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
    //     assert_eq!(part1(&as_input(INPUT)?), 62);
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 62);
        // assert_eq!(part2(&as_input(INPUT)?), 952408144115);
        Ok(())
    }
}
