use std::cmp;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;
use std::sync::atomic::AtomicI32;

use anyhow::{Context, Result};
use regex::Regex;

use utils::measure;

type Input = Vec<SpringRow>;

#[derive(Debug)]
struct SpringRow {
    condition_record: Vec<char>,
    damage_groups: Vec<i32>,
}

impl SpringRow {
    fn five_copies(&self) -> Self {
        let mut condition_record = vec![];
        let mut damage_groups = vec![];

        for i in 0..5 {
            if i > 0 {
                condition_record.push('?');
            }
            let mut tmp = self.condition_record.clone();
            condition_record.append(&mut tmp);

            let mut tmp = self.damage_groups.clone();
            damage_groups.append(&mut tmp);
        }

        Self {
            condition_record,
            damage_groups,
        }
    }

    fn possibble_arrangements(&self) -> i64 {
        self.possible_arrangements_group(0, 0, 0 /*, String::new() */)
    }

    fn possible_arrangements_group(
        &self,
        record_offset: usize,
        group_offset: usize,
        level: usize,
        // debug: String,
    ) -> i64 {
        // let fill = str::repeat("    ", level);
        // println!("{fill}---------------------------------------------------------------------------------------");
        // println!("{fill}possible_arrangements_group, record_offset={record_offset}, group_offset={group_offset}");

        if record_offset >= self.condition_record.len() {
            return 0;
        }

        let records = &self.condition_record[record_offset..];
        let groups = &self.damage_groups[group_offset..];
        // println!("{fill}  records={}", records.iter().collect::<String>());
        // println!("{fill}  groups={groups:?}");

        let group_len = groups[0];

        let mut possible = 0;
        for i in 0..(records.len() as i32 - (group_len - 1)) {
            if i > 0 && records[(i - 1) as usize] == '#' {
                break;
            }

            let g = &records[(i as usize)..((i + group_len) as usize)];
            // println!("{fill}  i={i}, g={g:?}");
            if g.iter().all(|&c| c == '#' || c == '?')
                && ((i as usize + group_len as usize) >= records.len()
                    || ['.', '?']
                        .into_iter()
                        .any(|c| records[i as usize + group_len as usize] == c))
            {
                // println!("{fill}  i={i}, g={g:?}");
                // println!("{fill}  matches");

                // let mut debug = debug.clone();
                // debug.push_str(&records.iter().take(i as usize).collect::<String>());
                // debug.push_str(&"#".repeat(group_len as usize));
                if group_offset < self.damage_groups.len() - 1 {
                    // debug.push_str(
                    //     &records
                    //         .iter()
                    //         .skip((i + group_len) as usize)
                    //         .take(1)
                    //         .collect::<String>(),
                    // );

                    let p = self.possible_arrangements_group(
                        record_offset + (group_len as usize) + (i as usize) + 1,
                        group_offset + 1,
                        level + 1,
                        // debug,
                    );

                    possible += p;
                } else {
                    // debug.push(' ');
                    // debug.push('!');
                    // debug.push_str(
                    //     &records
                    //         .iter()
                    //         .skip((i + group_len) as usize)
                    //         .collect::<String>(),
                    // );

                    let is_possible = !records
                        .iter()
                        .skip((i + group_len) as usize)
                        .any(|&c| c == '#');

                    // println!("{fill}  debug={debug} {is_possible}");

                    if is_possible {
                        possible += 1;
                    }
                }

                // Figure out if to break or not
                if i > 0 && records[(i - 1) as usize] == '#' {
                    break;
                }
            }
        }

        // println!("{fill}  possible={possible:?}");

        possible
    }
}

fn part1(input: &Input) -> i64 {
    // dbg!(input);
    let mut res = 0;
    for row in input
    /* .iter().skip(0).take(1)*/
    {
        println!("------------------------------------------------------------------------------------------------------");
        println!(
            "{} {:?}",
            row.condition_record.iter().collect::<String>(),
            row.damage_groups
        );
        let a = row.possibble_arrangements();
        println!("  arrangements: {a}",);
        res += a;
    }

    // 7676 too high
    res
}

fn part2(input: &Input) -> i64 {
    use rayon::prelude::*;
    use std::sync::atomic::{AtomicI32, Ordering};

    let mut cnt = AtomicI32::new(input.len() as i32);
    let rows = input.iter().enumerate().collect::<Vec<_>>();
    let arrangements: i64 =  rows.par_iter().map(|(i, row)| {
        let row = row.five_copies();
        let a = row.possibble_arrangements();
        let rem =  cnt.fetch_sub(1, Ordering::SeqCst) ;
        println!(
            "------------------------------------------------------------------------------------------------------\n{i}: {} {:?}\n  arrangements: {a}, remaining: {rem}",
            row.condition_record.iter().collect::<String>(),
            row.damage_groups
        );
        a
    }).sum();

    arrangements
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

impl FromStr for SpringRow {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');

        let mut condition_record = split.next().unwrap().chars().collect::<Vec<_>>();
        // condition_record.push('.');

        let damage_groups = split
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<i32>().context("Invalid grpup ength"))
            .collect::<Result<_>>()?;

        Ok(SpringRow {
            condition_record,
            damage_groups,
        })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.parse::<SpringRow>()
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
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1";

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
    //     assert_eq!(part1(&as_input(INPUT)?), 21);
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 525152);
        Ok(())
    }
}
