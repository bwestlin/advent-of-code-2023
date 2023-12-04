use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{Context, Result};

use utils::measure;

type Input = Vec<String>;

fn extract_digits(s: &str) -> (Vec<i32>, Vec<i32>) {
    const SPELLED_DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut digits1 = vec![];
    let mut digits2 = vec![];

    let chrs = s.chars().collect::<Vec<_>>();
    for idx in 0..s.len() {
        let c = chrs[idx];
        if c.is_ascii_digit() {
            let digit = (c as u8 - b'0') as i32;
            digits1.push(digit);
            digits2.push(digit);
        }

        for (si, sl) in SPELLED_DIGITS.iter().enumerate() {
            if s[idx..].starts_with(sl) {
                digits2.push(si as i32 + 1)
            }
        }
    }
    (digits1, digits2)
}

fn calibration_from_digits(digits: &[i32]) -> i32 {
    digits.iter().next().unwrap_or(&0) * 10 + digits.iter().last().unwrap_or(&0)
}

fn both_parts(input: &Input) -> (i32, i32) {
    input.iter().fold((0, 0), |(r1, r2), s| {
        let (digits1, digits2) = extract_digits(s);
        (
            r1 + calibration_from_digits(&digits1),
            r2 + calibration_from_digits(&digits2),
        )
    })
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
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.parse::<String>().context("Unable to parse input line"))
        .collect()
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

    const INPUT2: &str = "
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

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
        assert_eq!(both_parts(&as_input(INPUT1)?).0, 142);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(both_parts(&as_input(INPUT2)?).1, 281);
        Ok(())
    }
}
