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

#[derive(Debug)]
struct Input {
    workflows: Vec<Workflow>,
    part_ratings: Vec<PartRating>,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}
impl Workflow {
    fn eval(&self, ratings: &HashMap<char, i32>) -> &str {
        for r in &self.rules {
            if r.matches(ratings) {
                return r.target();
            }
        }
        unreachable!()
    }

    fn eval_one(&self, rating: char, v: i32) -> &str {
        for r in &self.rules {
            if r.matches_part(rating, v) {
                return r.target();
            }
        }
        unreachable!()
    }
}

#[derive(Debug)]
enum Rule {
    Gt {
        part: char,
        value: i32,
        target: String,
    },
    Lt {
        part: char,
        value: i32,
        target: String,
    },
    Send {
        target: String,
    },
}
impl Rule {
    fn matches(&self, ratings: &HashMap<char, i32>) -> bool {
        match self {
            Rule::Gt {
                part,
                value,
                target,
            } => ratings[part] > *value,
            Rule::Lt {
                part,
                value,
                target,
            } => ratings[part] < *value,
            Rule::Send { .. } => true,
        }
    }

    fn matches_part(&self, p: char, v: i32) -> bool {
        match self {
            Rule::Gt {
                part,
                value,
                target,
            } => {
                if *part != p {
                    false
                } else {
                    v > *value
                }
            }
            Rule::Lt {
                part,
                value,
                target,
            } => {
                if *part != p {
                    false
                } else {
                    v < *value
                }
            }
            Rule::Send { .. } => true,
        }
    }

    fn target(&self) -> &str {
        match self {
            Rule::Gt { target, .. } => target,
            Rule::Lt { target, .. } => target,
            Rule::Send { target } => target,
        }
    }
}

#[derive(Debug)]
struct PartRating {
    ratings: HashMap<char, i32>,
}

fn part1(input: &Input) -> i32 {
    // dbg!(input);

    let Input {
        workflows,
        part_ratings,
    } = input;

    let workflows = workflows
        .iter()
        .map(|wf| (wf.name.as_str(), wf))
        .collect::<HashMap<_, _>>();

    let mut ret = 0;
    for PartRating { ratings } in part_ratings {
        let mut curr_wf = "in";

        for _ in 0.. {
            let wf = workflows[curr_wf];

            let next_wf = wf.eval(ratings);

            if next_wf == "A" {
                println!("ratings={ratings:?} Accepted");
                ret += ratings.values().sum::<i32>();
                break;
            } else if next_wf == "R" {
                println!("ratings={ratings:?} Rejected");
                break;
            }

            curr_wf = next_wf;
        }
    }

    ret
}

fn part2(input: &Input) -> i64 {
    let Input {
        workflows,
        part_ratings,
    } = input;

    let workflows = workflows
        .iter()
        .map(|wf| (wf.name.as_str(), wf))
        .collect::<HashMap<_, _>>();

    let mut accepted = 0;

    let ratings_range = [
        ('x', (1, 4000)),
        ('m', (1, 4000)),
        ('a', (1, 4000)),
        ('s', (1, 4000)),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    let mut queue = VecDeque::new();
    queue.push_back(("in", 0, ratings_range));

    while let Some((curr_wf, idx, ratings_range)) = queue.pop_front() {
        if curr_wf == "A" {
            println!("curr_wf={curr_wf}, idx={idx}, ratings_range={ratings_range:?}");

            let mut tmp = vec![];
            for (_, (a, b)) in ratings_range {
                tmp.push((b - a + 1) as i64);
            }
            accepted += tmp.into_iter().product::<i64>();

            continue;
        } else if curr_wf == "R" {
            continue;
        }

        let wf = workflows[curr_wf];

        let rule = &wf.rules[idx];

        match rule {
            Rule::Gt {
                part,
                value,
                target,
            } => {
                let (a, b) = ratings_range[part];

                // Lower part
                if a <= *value {
                    let new_range = (a, *value);
                    let mut ratings_range = ratings_range.clone();
                    ratings_range.insert(*part, new_range);
                    queue.push_back((curr_wf, idx + 1, ratings_range));
                }
                // Higher part
                if b > *value {
                    let new_range = (*value + 1, b);
                    let mut ratings_range = ratings_range.clone();
                    ratings_range.insert(*part, new_range);
                    queue.push_back((target, 0, ratings_range));
                }
            }
            Rule::Lt {
                part,
                value,
                target,
            } => {
                let (a, b) = ratings_range[part];

                // Lower part
                if a < *value {
                    let new_range = (a, *value - 1);
                    let mut ratings_range = ratings_range.clone();
                    ratings_range.insert(*part, new_range);
                    queue.push_back((target, 0, ratings_range));
                }
                // Higher part
                if b >= *value {
                    let new_range = (*value, b);
                    let mut ratings_range = ratings_range.clone();
                    ratings_range.insert(*part, new_range);
                    queue.push_back((curr_wf, idx + 1, ratings_range));
                }
            }
            Rule::Send { target } => {
                queue.push_back((target, 0, ratings_range));
            }
        }
    }

    accepted
}

fn part2b(input: &Input) -> i64 {
    let Input {
        workflows,
        part_ratings,
    } = input;

    let workflows = workflows
        .iter()
        .map(|wf| (wf.name.as_str(), wf))
        .collect::<HashMap<_, _>>();

    let mut accepted = 0;
    let mut ratings = HashMap::new();
    for x in 1..=4000 {
        println!("x={x}");
        ratings.insert('x', x);
        for m in 1..=4000 {
            println!("m={m}");
            ratings.insert('m', x);
            for a in 1..=4000 {
                ratings.insert('a', x);
                for s in 1..=4000 {
                    ratings.insert('s', x);
                    let mut curr_wf = "in";

                    for _ in 0.. {
                        let wf = workflows[curr_wf];

                        let next_wf = wf.eval(&ratings);

                        if next_wf == "A" {
                            // println!("={ratings:?} Accepted");
                            accepted += 1;
                            break;
                        } else if next_wf == "R" {
                            break;
                        }

                        curr_wf = next_wf;
                    }
                }
            }
        }
    }

    accepted
}

fn part2a(input: &Input) -> i64 {
    let Input {
        workflows,
        part_ratings,
    } = input;

    let workflows = workflows
        .iter()
        .map(|wf| (wf.name.as_str(), wf))
        .collect::<HashMap<_, _>>();

    let mut acc = vec![];
    for part in ['x', 'm', 'a', 's'] {
        let mut accepted = 0;
        for v in 1..=4000 {
            let mut curr_wf = "in";

            for _ in 0.. {
                let wf = workflows[curr_wf];

                let next_wf = wf.eval_one(part, v);

                if next_wf == "A" {
                    // println!("={ratings:?} Accepted");
                    accepted += 1;
                    break;
                } else if next_wf == "R" {
                    break;
                }

                curr_wf = next_wf;
            }
        }

        println!("{part} accepted={accepted}");
        acc.push(accepted);
    }

    acc.into_iter().product()
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

impl FromStr for Workflow {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('{');
        let name = split.next().unwrap().to_owned();
        let rest = split.next().unwrap();
        let rest = &rest[0..(rest.len() - 1)];

        let mut split = rest.split(',');
        let mut rules = vec![];
        for s in split {
            let rule = s.parse().unwrap();
            rules.push(rule);
        }

        Ok(Self { name, rules })
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');

        let rule = match (split.next(), split.next()) {
            (Some(target), None) => Rule::Send {
                target: target.to_owned(),
            },
            (Some(s), Some(target)) => {
                let target = target.to_owned();

                if s.contains('>') {
                    let mut split = s.split('>');
                    let part = split.next().unwrap().chars().next().unwrap();
                    let value = split.next().unwrap().parse().unwrap();
                    Rule::Gt {
                        part,
                        value,
                        target,
                    }
                } else {
                    let mut split = s.split('<');
                    let part = split.next().unwrap().chars().next().unwrap();
                    let value = split.next().unwrap().parse().unwrap();
                    Rule::Lt {
                        part,
                        value,
                        target,
                    }
                }
            }
            _ => unreachable!(),
        };

        Ok(rule)
    }
}

impl FromStr for PartRating {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = &s[1..(s.len() - 1)];
        let mut split = s.split(',');

        let mut ratings = HashMap::new();
        for s in split {
            let mut split = s.split('=');
            let part = split.next().unwrap().chars().next().unwrap();
            let rating = split.next().unwrap().parse().unwrap();
            ratings.insert(part, rating);
        }

        Ok(Self { ratings })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    let mut lines = reader.lines().map_while(Result::ok);

    let mut workflows = vec![];
    let mut part_ratings = vec![];

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let wf = line.parse()?;
        workflows.push(wf);
    }

    for line in lines {
        let pr = line.parse()?;
        part_ratings.push(pr);
    }

    Ok(Input {
        workflows,
        part_ratings,
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
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}";

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
    //     assert_eq!(part1(&as_input(INPUT)?), 19114);
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 167409079868000);
        Ok(())
    }
}
