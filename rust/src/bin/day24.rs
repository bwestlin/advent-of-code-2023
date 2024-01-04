use std::cmp;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

use anyhow::{Context, Result};
use regex::Regex;

use utils::measure;

type Input = Vec<HailstoneData>;

#[derive(Debug)]
struct HailstoneData {
    position: Vec3,
    velocity: Vec3,
}

impl std::fmt::Display for HailstoneData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}", self.position, self.velocity)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    // fn adjacent(&self) -> impl Iterator<Item = Vec3> + '_ {
    //     [(1, 0), (-1, 0), (0, 1), (0, -1)]
    //         .into_iter()
    //         .map(|(dx, dy)| Vec3::new(self.x + dx, self.y + dy))
    // }

    fn rev(&self) -> Vec3 {
        Vec3::new(self.x * -1., self.y * -1., self.z * -1.)
    }

    fn dist(&self, other: &Vec3) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;

        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    fn dist_xy(&self, other: &Vec3) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        (dx * dx + dy * dy).sqrt()
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
    }
}

fn line_formula_xy(hd: &HailstoneData) -> (f64, f64) {
    // y - y1 = m(x - x1)
    let p1 = hd.position;
    let m = hd.velocity.y / hd.velocity.x;
    let c = (-p1.x * m) + p1.y;
    (m, c)
}

fn intersection_xy(a: &HailstoneData, b: &HailstoneData) -> Option<Vec3> {
    if a.velocity == b.velocity || a.velocity == b.velocity.rev() {
        None
    } else {
        // Line formula a
        let (a, c) = line_formula_xy(a);
        // Line formula b
        let (b, d) = line_formula_xy(b);

        let x = (d - c) / (a - b);
        let y = (a * ((d - c) / (a - b))) + c;
        let z = 0.;

        if x.is_nan() || y.is_nan() {
            None
        } else {
            Some(Vec3::new(x, y, z))
        }
    }
}

fn in_the_past(a: &HailstoneData, intersect: &Vec3) -> bool {
    let dx = a.position.x - intersect.x;
    let dy = a.position.y - intersect.y;
    let dz = a.position.z - intersect.z;

    // println!("in_the_past({a}, {intersect})");
    // println!("  dx={dx}, dy={dy}");

    dx.signum() == a.velocity.x.signum()
        || dy.signum() == a.velocity.y.signum()
        || dz.signum() == a.velocity.z.signum()
}

fn part1(input: &Input) -> i32 {
    dbg!(input);

    let a = &input[0];
    let b = &input[1];

    let mut combinations = BTreeSet::new(); // TODO HashSet
    for a in 0..input.len() {
        for b in 0..input.len() {
            let pair = [a, b].into_iter().collect::<BTreeSet<_>>();
            if pair.len() == 2 {
                let mut iter = pair.into_iter();
                let pair = [
                    iter.next().unwrap_or_default(),
                    iter.next().unwrap_or_default(),
                ];
                combinations.insert(pair);
            }
        }
    }

    dbg!(&combinations);

    #[cfg(not(test))]
    let (test_x, test_y) = (
        200000000000000.0..=400000000000000.0,
        200000000000000.0..=400000000000000.0,
    );
    #[cfg(test)]
    let (test_x, test_y) = (7.0..=27.0, 7.0..=27.0);

    let mut res = 0;
    for [a, b] in combinations {
        let a = &input[a];
        let b = &input[b];
        let intersect = intersection_xy(a, b);

        println!("Hailstone A: {a}");
        println!("Hailstone B: {b}");

        if let Some(intersect) = intersect {
            // Check if it's in the past
            let past_a = in_the_past(a, &intersect);
            let past_b = in_the_past(b, &intersect);

            if past_a && past_b {
                println!("Hailstones' paths crossed in the past for both hailstones.");
            } else if past_a {
                println!("Hailstones' paths crossed in the past for hailstone A.");
            } else if past_b {
                println!("Hailstones' paths crossed in the past for hailstone B.");
            } else {
                if test_x.contains(&intersect.x) && test_y.contains(&intersect.y) {
                    println!(
                        "Hailstones' paths will cross inside the test area (at x={}, y={}).",
                        intersect.x, intersect.y
                    );
                    res += 1;
                } else {
                    println!(
                        "Hailstones' paths will cross outside the test area (at x={}, y={}).",
                        intersect.x, intersect.y
                    );
                }
            }
        } else {
            println!("Hailstones' paths are parallel; they never intersect.");
        }

        println!();
    }

    res
}

fn part2(input: &Input) -> i64 {
    let mut combinations = BTreeSet::new(); // TODO HashSet
    for a in 0..input.len() {
        for b in 0..input.len() {
            let pair = [a, b].into_iter().collect::<BTreeSet<_>>();
            if pair.len() == 2 {
                let mut iter = pair.into_iter();
                let pair = [
                    iter.next().unwrap_or_default(),
                    iter.next().unwrap_or_default(),
                ];
                combinations.insert(pair);
            }
        }
    }

    // dbg!(&combinations);

    let d_ns = 13;

    let mut res = 0;

    for d_ns in 1.. {
        println!("d_ns={d_ns}");

        for [ai, bi] in &combinations {
            'outer: for [ai, bi] in [[ai, bi], [bi, ai]] {
                let a = &input[*ai];
                let b = &input[*bi];

                // c is b d_ns ns forward in time
                let f = 1.;
                let c = &HailstoneData {
                    position: Vec3::new(
                        a.position.x + (a.velocity.x * f),
                        a.position.y + (a.velocity.y * f),
                        a.position.z + (a.velocity.z * f),
                    ),
                    velocity: a.velocity.clone(),
                };

                // d is b d_ns ns forward in time
                let f = d_ns as f64 + 1.;
                let d = &HailstoneData {
                    position: Vec3::new(
                        b.position.x + (b.velocity.x * f),
                        b.position.y + (b.velocity.y * f),
                        b.position.z + (b.velocity.z * f),
                    ),
                    velocity: b.velocity.clone(),
                };

                // d is the line of the test throw
                let f = d_ns as f64;
                let tt = HailstoneData {
                    position: c.position,
                    velocity: Vec3::new(
                        (d.position.x - c.position.x) / f,
                        (d.position.y - c.position.y) / f,
                        (d.position.z - c.position.z) / f,
                    ),
                };

                let unit_dist = Vec3::new(0., 0., 0.).dist_xy(&tt.velocity);

                // Now both a and b should intersect with d, otherwise something is off
                // let intersect_a = intersection_xy(a, &d);
                // let intersect_b = intersection_xy(b, &d);

                // println!(" intersect_a={intersect_a:?}, intersect_b={intersect_b:?}");

                let mut intersections = HashMap::new();

                // Check if all hailstones intersects
                for i in 0..input.len() {
                    if let Some(x) = intersection_xy(&input[i], &tt) {
                        intersections.insert(i, x);
                    } else {
                        continue 'outer;
                    }
                }

                let mut max_past_dist = 0.;
                let mut max_future_dist = 0.;
                let mut dist_facs = HashMap::new();

                for (i, x) in &intersections {
                    let dist = x.dist_xy(&tt.position);
                    dist_facs.insert(i, dist / unit_dist);
                    if in_the_past(&tt, x) {
                        if dist > max_past_dist {
                            max_past_dist = dist;
                        }
                    } else {
                        if dist > max_future_dist {
                            max_future_dist = dist;
                        }
                    }
                }

                if !dist_facs.iter().all(|(_, df)| df.fract().abs() < 0.01) {
                    continue;
                }

                // Validate distance factors
                for (&&i, df) in &dist_facs {
                    let hd = &input[i];
                    let hd_f = 1. + df;
                    let hd_p = Vec3::new(
                        hd.position.x + (hd.velocity.x * hd_f),
                        hd.position.y + (hd.velocity.y * hd_f),
                        hd.position.z + (hd.velocity.z * hd_f),
                    );

                    let tt_p = Vec3::new(
                        tt.position.x + (tt.velocity.x * df),
                        tt.position.y + (tt.velocity.y * df),
                        tt.position.z + (tt.velocity.z * df),
                    );

                    let dx = (hd_p.x - tt_p.x).abs();
                    let dy = (hd_p.y - tt_p.y).abs();
                    let dz = (hd_p.z - tt_p.z).abs();

                    if dx >= 0.01 || dy >= 0.01 || dz >= 0.01 {
                        continue 'outer;
                    }
                }

                let tt_origin = Vec3::new(
                    tt.position.x - tt.velocity.x,
                    tt.position.y - tt.velocity.y,
                    tt.position.z - tt.velocity.z,
                );

                let max_past_dist_fac = max_past_dist / unit_dist;
                let max_future_dist_fac = max_future_dist / unit_dist;

                println!("Hailstone A: {a} ({ai})");
                println!("  repositioned: {c}");
                println!("Hailstone B: {b} ({bi})");
                println!("  repositioned: {d}");
                println!("Hailstone TT: {tt}");
                println!("  All intersects: {intersections:?}");
                println!("  unit_dist: {unit_dist}");
                println!("  dist_facs: {dist_facs:?}");
                println!("  max_past_dist: {max_past_dist}, max_future_dist={max_future_dist}");
                println!(
                    "  max_past_dist_fac: {max_past_dist_fac}, \
                   max_future_dist_fac={max_future_dist_fac}"
                );
                println!("  tt_origin={tt_origin}");

                return (tt_origin.x + tt_origin.y + tt_origin.z) as i64;

                // Translate back to the first

                // println!();
            }
        }
    }

    // 2147483647 too low
    // 501923552029592 too low
    res
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

impl FromStr for HailstoneData {
    type Err = ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('@');
        let position = split.next().unwrap().trim().parse()?;
        let velocity = split.next().unwrap().trim().parse()?;
        Ok(HailstoneData { position, velocity })
    }
}

impl FromStr for Vec3 {
    type Err = ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        let x = split.next().unwrap().trim().parse()?;
        let y = split.next().unwrap().trim().parse()?;
        let z = split.next().unwrap().trim().parse()?;
        Ok(Vec3 { x, y, z })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.parse::<HailstoneData>()
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
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3";

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
    //     assert_eq!(part1(&as_input(INPUT)?), 2);
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 47);
        Ok(())
    }
}
