use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use anyhow::{Context, Result};

use utils::measure;

type Input = Vec<Game>;

#[derive(Debug, Default)]
struct Cubes {
    red: i32,
    green: i32,
    blue: i32,
}

impl Cubes {
    fn values(&self) -> impl Iterator<Item = i32> {
        [self.red, self.green, self.blue].into_iter()
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    revealed_cubes: Vec<Cubes>,
}

impl Game {
    fn possible(&self, cubes: &Cubes) -> bool {
        for reveal in self.revealed_cubes.iter() {
            if reveal.red > cubes.red || reveal.green > cubes.green || reveal.blue > cubes.blue {
                return false;
            }
        }
        true
    }

    fn fewest_possible(&self) -> Cubes {
        let mut fewest = Cubes::default();

        for cubes in self.revealed_cubes.iter() {
            if fewest.red < cubes.red {
                fewest.red = cubes.red;
            }
            if fewest.green < cubes.green {
                fewest.green = cubes.green;
            }
            if fewest.blue < cubes.blue {
                fewest.blue = cubes.blue;
            }
        }

        fewest
    }
}

fn solve(input: &Input) -> (i32, i32) {
    let loaded_p1 = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };

    input.iter().fold((0, 0), |(mut p1, mut p2), game| {
        if game.possible(&loaded_p1) {
            p1 += game.id;
        }

        p2 += game.fewest_possible().values().product::<i32>();

        (p1, p2)
    })
}

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        let (part1, part2) = solve(&input);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
        Ok(())
    })
}

impl FromStr for Game {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.splitn(2, ':');
        let mut id_part = split.next().context("no id part")?.split(' ');
        let reveal_part = split.next().context("no reveal part")?.split(';');

        let id = id_part.nth(1).context("no id")?.parse()?;
        let mut revealed_cubes = vec![];

        for part in reveal_part {
            let mut cubes = Cubes::default();

            for part in part.split(',') {
                let mut split = part.trim().split(' ');
                let count = split.next().context("no count")?.parse()?;
                let color = split.next().context("no color")?;

                match color {
                    "red" => cubes.red = count,
                    "green" => cubes.green = count,
                    "blue" => cubes.blue = count,
                    _ => anyhow::bail!("No such color: {s}"),
                }
            }

            revealed_cubes.push(cubes);
        }

        Ok(Game { id, revealed_cubes })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.parse::<Game>().context("Unable to parse input line"))
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
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

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
        assert_eq!(solve(&as_input(INPUT)?).0, 8);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(solve(&as_input(INPUT)?).1, 2286);
        Ok(())
    }
}
