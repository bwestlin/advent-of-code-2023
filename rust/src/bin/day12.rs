use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use anyhow::{Context, Result};

use utils::measure;

type Input = Vec<SpringRow>;

#[derive(Debug)]
struct SpringRow {
    condition_record: Vec<char>,
    damage_groups: Vec<usize>,
}

impl SpringRow {
    fn five_copies(&self) -> Self {
        let mut condition_record = vec![];
        let mut damage_groups = vec![];

        for i in 0..5 {
            if i > 0 {
                condition_record.push('?');
            }
            condition_record.append(&mut self.condition_record.clone());
            damage_groups.append(&mut self.damage_groups.clone());
        }

        Self {
            condition_record,
            damage_groups,
        }
    }

    fn possible_arrangements(&self) -> i64 {
        let mut cache = HashMap::new();
        self.possible_arrangements_group(0, 0, &mut cache)
    }

    fn possible_arrangements_group(
        &self,
        record_offset: usize,
        group_offset: usize,
        cache: &mut HashMap<(usize, usize), i64>,
    ) -> i64 {
        if record_offset >= self.condition_record.len() {
            return 0;
        }
        if let Some(a) = cache.get(&(record_offset, group_offset)) {
            return *a;
        }

        let records = &self.condition_record[record_offset..];
        let groups = &self.damage_groups[group_offset..];

        let group_len = groups[0];
        let min_groups_len_rem = groups.iter().sum::<usize>() + groups.len() - 1;

        let mut possible = 0;
        for i in 0..(records.len() - (group_len - 1)) {
            if records.len() - i < min_groups_len_rem {
                break;
            }
            if i > 0 && records[i - 1] == '#' {
                break;
            }

            let g = &records[i..(i + group_len)];
            if g.iter().all(|&c| c == '#' || c == '?')
                && ((i + group_len) >= records.len()
                    || ['.', '?'].into_iter().any(|c| records[i + group_len] == c))
            {
                if group_offset < self.damage_groups.len() - 1 {
                    let p = self.possible_arrangements_group(
                        record_offset + group_len + i + 1,
                        group_offset + 1,
                        cache,
                    );

                    possible += p;
                } else {
                    let is_possible = !records.iter().skip(i + group_len).any(|&c| c == '#');

                    if is_possible {
                        possible += 1;
                    }
                }
            }
        }

        cache.insert((record_offset, group_offset), possible);

        possible
    }
}

fn part1(input: &Input) -> i64 {
    input.iter().map(|row| row.possible_arrangements()).sum()
}

fn part2(input: &Input) -> i64 {
    input
        .iter()
        .map(|row| row.five_copies().possible_arrangements())
        .sum()
}

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
        Ok(())
    })
}

impl FromStr for SpringRow {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let condition_record = split.next().unwrap().chars().collect::<Vec<_>>();
        let damage_groups = split
            .next()
            .context("No groups")?
            .split(',')
            .map(|s| s.parse::<usize>().context("Invalid group length"))
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

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(&as_input(INPUT)?), 21);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 525152);
        Ok(())
    }
}
