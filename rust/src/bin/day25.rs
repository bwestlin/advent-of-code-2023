use std::borrow::Borrow;
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

type Input = Vec<Connection>;

#[derive(Debug)]
struct Connection {
    name: String,
    other: Vec<String>,
}

fn component_groups<'a>(
    connections: &'a Vec<BTreeSet<String>>,
    skip: &'a Vec<usize>,
) -> Vec<BTreeSet<&'a String>> {
    let n_connections = connections.len() - skip.len();
    let mut groups = Vec::<BTreeSet<&String>>::new();
    let first_idx = (0..connections.len())
        .filter(|i| !skip.contains(i))
        .next()
        .unwrap();
    groups.push(connections[first_idx].iter().collect());
    let mut grouped_connections = [first_idx].into_iter().collect::<HashSet<_>>();

    let mut cnt = 0;
    while grouped_connections.len() < n_connections {
        // cnt += 1;
        // if cnt > 100000 {
        //     print!("Uh oh!");
        //     break;
        // }

        let mut any_grouped = false;
        let last_group_idx = groups.len() - 1;
        if let Some(group) = groups.get_mut(last_group_idx) {
            for cidx in 0..connections.len() {
                if grouped_connections.contains(&cidx) || skip.contains(&cidx) {
                    continue;
                }
                let c = &connections[cidx];

                if group.len() == 0 || c.iter().any(|n| group.contains(n)) {
                    for n in c {
                        group.insert(n);
                    }
                    grouped_connections.insert(cidx);
                    any_grouped = true;
                }
            }
        }

        if !any_grouped && grouped_connections.len() < n_connections {
            groups.push(BTreeSet::new());
        }
    }

    groups
}

struct Combinations {
    values: Vec<usize>,
    indexes: Vec<usize>,
    n: usize,
    first: bool,
}

impl Combinations {
    fn new(values: &Vec<usize>, n: usize) -> Combinations {
        Combinations {
            values: values.clone(),
            indexes: (0..n).collect(),
            n: n,
            first: true,
        }
    }
}

impl Iterator for Combinations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(
                self.indexes
                    .iter()
                    .map(|i| self.values[*i])
                    .collect::<Self::Item>(),
            );
        }

        let n_values = self.values.len();
        let n = self.n;
        {
            let v = &mut self.indexes;
            let l = n - 1;
            for i in 0..n {
                if v[l - i] == n_values - 1 - i {
                    if i == n - 1 {
                        return None;
                    }
                    v[l - i] = v[l - i - 1] + 2;
                    if i > 0 {
                        for j in (0..=(i - 1)).rev() {
                            v[l - j] = v[l - (j + 1)] + 1;
                        }
                    }
                } else {
                    v[l - i] += 1;
                    break;
                }
            }
        }

        // This is faster than: Some(self.indexes.iter().map(|i| self.values[*i]).collect::<Self::Item>())
        let mut next = Vec::with_capacity(n_values);
        for i in 0..n {
            next.push(self.values[self.indexes[i]]);
        }
        Some(next)
    }
}

fn part1(input: &Input) -> usize {
    // dbg!(input);

    let connections = input
        .iter()
        .flat_map(|Connection { name, other }| {
            other.iter().map(|o| {
                [name.clone(), o.clone()]
                    .into_iter()
                    .collect::<BTreeSet<_>>()
            })
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    // for c in [["hfx", "pzl"], ["bvb", "cmg"], ["nvd", "jqt"]] {
    //     let c = c.into_iter().map(str::to_owned).collect::<BTreeSet<_>>();
    //     let i = connections
    //         .iter()
    //         .enumerate()
    //         .find(|(_, c2)| c == **c2)
    //         .unwrap()
    //         .0;
    //     connections.remove(i);
    // }

    // dbg!(&connections);

    // let mut combinations = BTreeSet::new(); // TODO HashSet
    // for a in 0..connections.len() {
    //     for b in 0..connections.len() {
    //         for c in 0..connections.len() {
    //             let triple = [a, b, c].into_iter().collect::<BTreeSet<_>>();
    //             if triple.len() == 3 {
    //                 let mut iter = triple.into_iter();
    //                 let triple = [
    //                     iter.next().unwrap_or_default(),
    //                     iter.next().unwrap_or_default(),
    //                     iter.next().unwrap_or_default(),
    //                 ];
    //                 combinations.insert(triple);
    //             }
    //         }
    //     }
    // }

    // // dbg!(&combinations);
    // dbg!(combinations.len());

    // let n_combinations = combinations.len();

    let combinations =
        Combinations::new(&(0..connections.len()).into_iter().collect::<Vec<_>>(), 3);

    for (i, comb) in combinations.enumerate() {
        // comb.sort();
        // let a = comb[0];
        // let b = comb[1];
        // let c = comb[2];
        println!("{i} {comb:?}");
        // let mut connections = connections.clone();
        // connections.remove(c);
        // connections.remove(b);
        // connections.remove(a);

        let groups = component_groups(&connections, &comb);
        if groups.len() == 2 {
            dbg!(&groups);
            return groups[0].len() * groups[1].len();
        }
    }

    // dbg!(combinations.len());

    // Testing...
    // let connections = connections
    //     .into_iter()
    //     .map(|c| {
    //         let mut iter = c.into_iter();
    //         [iter.next().unwrap(), iter.next().unwrap()]
    //     })
    //     .collect::<Vec<_>>();

    // let mut freq = BTreeMap::<&String, BTreeSet<&String>>::new();

    // for [a, b] in &connections {
    //     freq.entry(a).or_default().insert(b);
    //     freq.entry(b).or_default().insert(a);
    // }

    // dbg!(&freq);

    0
}

fn part2(input: &Input) -> i32 {
    // dbg!(input);
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

impl FromStr for Connection {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');
        let name = split.next().unwrap().trim().to_owned();
        let split = split.next().unwrap().trim().split(' ');
        let other = split.into_iter().map(str::to_owned).collect();
        Ok(Connection { name, other })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.parse::<Connection>()
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
        jqt: rhn xhk nvd
        rsh: frs pzl lsr
        xhk: hfx
        cmg: qnr nvd lhk bvb
        rhn: xhk bvb hfx
        bvb: xhk hfx
        pzl: lsr hfx nvd
        qnr: nvd
        ntq: jqt hfx bvb xhk
        nvd: lhk
        lsr: lhk
        rzs: qnr cmg lsr rsh
        frs: qnr lhk lsr";

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
        assert_eq!(part1(&as_input(INPUT)?), 54);
        Ok(())
    }

    // #[test]
    // fn test_part2() -> Result<()> {
    //     assert_eq!(part2(&as_input(INPUT)?), 1337);
    //     Ok(())
    // }
}
