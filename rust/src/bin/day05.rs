use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{Context, Result};

use utils::measure;

type Input = Almanac;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Map>,
}

#[derive(Debug)]
struct Map {
    #[allow(dead_code)]
    source: String,
    #[allow(dead_code)]
    destination: String,
    conversions: Vec<Conversion>,
}

#[derive(Debug)]
struct Conversion {
    dst_range_start: i64,
    src_range_start: i64,
    range_length: i64,
}

impl Almanac {
    fn seeds_to_min_location(&self) -> i64 {
        self.seeds
            .iter()
            .map(|&seed| self.maps.iter().fold(seed, |v, map| map.apply(v)))
            .min()
            .unwrap_or_default()
    }

    /*
    // Initial brute force solution for part 2 that didn't have bugs
    fn seed_ranges_to_min_location_brute(&self) -> Vec<i64> {
        self.seeds
            .chunks(2)
            .flat_map(|chunk| {
                let start = chunk[0];
                let range = chunk[1];
                println!("Attemping {start}:{range}");
                (start..(start + range))
                    .into_iter()
                    .map(|seed| self.maps.iter().fold(seed, |v, map| map.apply(v)))
                    .min()
            })
            .collect()
    }
    */

    fn seed_ranges_to_min_location(&self) -> i64 {
        self.seeds
            .chunks(2)
            .flat_map(|chunk| {
                let start = chunk[0];
                let range = chunk[1];
                let mut ranges = vec![(start, range)];

                for map in &self.maps {
                    let mut next_ranges = map.apply_ranges(ranges.clone());
                    std::mem::swap(&mut next_ranges, &mut ranges);
                }

                ranges.into_iter().map(|(start, _)| start).min()
            })
            .min()
            .unwrap_or_default()
    }
}

impl Map {
    fn apply(&self, v: i64) -> i64 {
        for &Conversion {
            dst_range_start,
            src_range_start,
            range_length,
        } in &self.conversions
        {
            if (src_range_start..(src_range_start + range_length)).contains(&v) {
                let offset = v - src_range_start;
                return dst_range_start + offset;
            }
        }

        v
    }

    fn apply_ranges(&self, ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
        let mut src_ranges = ranges;
        let mut res_ranges = vec![];

        for &Conversion {
            dst_range_start,
            src_range_start,
            range_length,
        } in &self.conversions
        {
            for (start, len) in std::mem::take(&mut src_ranges) {
                let SubRanges { within, outside } =
                    sub_ranges((start, len), (src_range_start, range_length));

                if let Some((start, len)) = within {
                    let offset = start - src_range_start;
                    res_ranges.push((dst_range_start + offset, len));
                }

                for outside in outside {
                    src_ranges.push(outside);
                }
            }
        }

        for r in src_ranges {
            res_ranges.push(r);
        }
        res_ranges
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SubRanges {
    within: Option<(i64, i64)>,
    outside: Vec<(i64, i64)>,
}

fn sub_ranges(range: (i64, i64), divider: (i64, i64)) -> SubRanges {
    let mut within = None;
    let mut outside = Vec::with_capacity(2);
    if range.0 < divider.0 {
        if range.0 + range.1 <= divider.0 {
            outside.push((range.0, range.1));
        } else {
            outside.push((range.0, divider.0 - range.0));
            if range.0 + range.1 <= divider.0 + divider.1 {
                within = Some((divider.0, range.0 + range.1 - divider.0));
            } else {
                within = Some((divider.0, divider.1));
                outside.push((
                    divider.0 + divider.1,
                    range.0 + range.1 - (divider.0 + divider.1),
                ));
            }
        }
    } else if range.0 < divider.0 + divider.1 {
        if range.0 + range.1 <= divider.0 + divider.1 {
            within = Some((range.0, range.1));
        } else {
            within = Some((range.0, divider.0 + divider.1 - range.0));
            outside.push((
                divider.0 + divider.1,
                range.0 + range.1 - (divider.0 + divider.1),
            ));
        }
    } else {
        outside.push((range.0, range.1));
    }

    SubRanges { within, outside }
}

fn part1(input: &Input) -> i64 {
    input.seeds_to_min_location()
}

fn part2(input: &Input) -> i64 {
    input.seed_ranges_to_min_location()
}

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
        Ok(())
    })
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    let mut lines = reader.lines().map_while(Result::ok);

    let seeds_line = lines.next().context("no seeds line")?;
    let seeds = seeds_line
        .split(": ")
        .nth(1)
        .context("no seeds")?
        .split(' ')
        .flat_map(|s| s.parse::<i64>().ok())
        .collect();

    let _ = lines.next();

    let mut maps = vec![];
    while let Some(map) = read_map(&mut lines)? {
        maps.push(map);
    }

    Ok(Almanac { seeds, maps })
}

fn read_map(lines: &mut impl Iterator<Item = String>) -> Result<Option<Map>> {
    let header = lines.next();
    if header.is_none() {
        return Ok(None);
    }
    let header = header.context("no map header")?;

    let mut split = header.split("-to-");
    let source = split.next().context("no source")?.to_string();
    let destination = split
        .next()
        .context("no destination")?
        .split(' ')
        .next()
        .context("no destination")?
        .to_string();

    let mut conversions = vec![];

    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }
        let mut parts = line.split(' ');
        conversions.push(Conversion {
            dst_range_start: parts
                .next()
                .context("no destination range start")?
                .parse()
                .context("invalid destination range start")?,
            src_range_start: parts
                .next()
                .context("no source range start")?
                .parse()
                .context("invalid source range start")?,
            range_length: parts
                .next()
                .context("no range length")?
                .parse()
                .context("invalid range length")?,
        })
    }

    Ok(Some(Map {
        source,
        destination,
        conversions,
    }))
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";

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
        assert_eq!(part1(&as_input(INPUT)?), 35);
        Ok(())
    }

    #[test]
    fn test_sub_ranges() -> Result<()> {
        assert_eq!(
            sub_ranges((1, 1), (5, 5)),
            SubRanges {
                within: None,
                outside: vec![(1, 1)]
            }
        );
        assert_eq!(
            sub_ranges((1, 4), (5, 5)),
            SubRanges {
                within: None,
                outside: vec![(1, 4)]
            }
        );
        assert_eq!(
            sub_ranges((1, 5), (5, 5)),
            SubRanges {
                within: Some((5, 1)),
                outside: vec![(1, 4)]
            }
        );
        assert_eq!(
            sub_ranges((1, 6), (5, 5)),
            SubRanges {
                within: Some((5, 2)),
                outside: vec![(1, 4)]
            }
        );
        assert_eq!(
            sub_ranges((5, 5), (5, 5)),
            SubRanges {
                within: Some((5, 5)),
                outside: vec![]
            }
        );
        assert_eq!(
            sub_ranges((5, 6), (5, 5)),
            SubRanges {
                within: Some((5, 5)),
                outside: vec![(10, 1)]
            }
        );
        assert_eq!(
            sub_ranges((6, 6), (5, 5)),
            SubRanges {
                within: Some((6, 4)),
                outside: vec![(10, 2)]
            }
        );
        assert_eq!(
            sub_ranges((9, 1), (5, 5)),
            SubRanges {
                within: Some((9, 1)),
                outside: vec![]
            }
        );
        assert_eq!(
            sub_ranges((9, 2), (5, 5)),
            SubRanges {
                within: Some((9, 1)),
                outside: vec![(10, 1)]
            }
        );
        assert_eq!(
            sub_ranges((10, 1), (5, 5)),
            SubRanges {
                within: None,
                outside: vec![(10, 1)]
            }
        );
        assert_eq!(
            sub_ranges((1, 11), (5, 5)),
            SubRanges {
                within: Some((5, 5)),
                outside: vec![(1, 4), (10, 2)]
            }
        );
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 46);
        Ok(())
    }
}
