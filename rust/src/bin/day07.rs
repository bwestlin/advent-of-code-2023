use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use anyhow::{Context, Result};

use utils::measure;

type Input = Vec<Hand>;

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];
const JOKER_IDX: usize = 3;

#[derive(Debug)]
struct Hand {
    cards: Vec<usize>,
    bid: usize,
}

impl Hand {
    fn get_type(&self, joker_pretend: bool) -> Type {
        let mut freq = HashMap::<usize, i32>::new();
        for &c in &self.cards {
            *freq.entry(c).or_default() += 1;
        }

        let j_freq = if joker_pretend {
            freq.remove(&JOKER_IDX)
        } else {
            None
        };

        let mut vals = freq.values().cloned().collect::<Vec<_>>();
        vals.sort();
        vals.reverse();

        if let Some(j) = j_freq {
            if vals.is_empty() {
                vals.push(j);
            } else {
                vals[0] += j;
            }
        }

        match vals[..] {
            [5] => Type::FiveOfAKind,
            [4, 1] => Type::FourOfAKind,
            [3, 2] => Type::FullHouse,
            [3, ..] => Type::ThreeOfAKind,
            [2, 2, ..] => Type::TwoPair,
            [2, ..] => Type::OnePair,
            _ => Type::HighCard,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Type {
    fn idx(&self) -> isize {
        *self as isize
    }
}

fn total_winnings(hands: &Vec<Hand>, joker_pretend: bool) -> usize {
    let mut hand_type = Vec::with_capacity(hands.len());
    for hand in hands {
        let typ = hand.get_type(joker_pretend);
        let cards = hand
            .cards
            .iter()
            .map(|&c| {
                if joker_pretend && c == JOKER_IDX {
                    CARDS.len()
                } else {
                    c
                }
            })
            .collect::<Vec<_>>();

        hand_type.push((hand, typ, cards));
    }

    hand_type.sort_by(|a, b| a.1.idx().cmp(&b.1.idx()).then(a.2.cmp(&b.2)));

    let num_hands = hand_type.len();
    let mut res = 0;
    for (i, (hand, _, _)) in hand_type.into_iter().enumerate() {
        let rank = num_hands - i;
        res += rank * hand.bid;
    }

    res
}

fn part1(input: &Input) -> usize {
    total_winnings(input, false)
}

fn part2(input: &Input) -> usize {
    total_winnings(input, true)
}

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
        Ok(())
    })
}

impl FromStr for Hand {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let cards = split
            .next()
            .context("No cards")?
            .chars()
            .map(card_idx)
            .collect::<Result<_>>()?;
        let bid = split.next().context("No bid")?.parse()?;

        Ok(Hand { cards, bid })
    }
}

fn card_idx(c: char) -> anyhow::Result<usize> {
    Ok(CARDS
        .iter()
        .enumerate()
        .find(|&(_, b)| c == *b)
        .context(format!("No card for {c}"))?
        .0)
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.parse::<Hand>().context("Unable to parse input line"))
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
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

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
        assert_eq!(part1(&as_input(INPUT)?), 6440);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 5905);
        Ok(())
    }
}
