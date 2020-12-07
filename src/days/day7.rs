use aoc_runner_derive::aoc;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct Bag<'a> {
    prefix: &'a str,
    color: &'a str,
}

type FromTo<'a> = Vec<(Bag<'a>, Vec<(Bag<'a>, i32)>)>;

pub fn parse(input: &str) -> Option<Vec<(Bag, Vec<(Bag, i32)>)>> {
    let mut parsed: Vec<(Bag, Vec<(Bag, i32)>)> = vec![];
    for line in input.lines() {
        let mut words = line.split_ascii_whitespace();
        let bag = Bag {
            prefix: words.next()?,
            color: words.next()?,
        };
        words.next();
        words.next();
        let mut bags = vec![];
        while let Some(word) = words.next() {
            if word == "no" {
                break;
            }
            bags.push((
                Bag {
                    prefix: words.next()?,
                    color: words.next()?,
                },
                word.parse().unwrap(),
            ));
            words.next();
        }
        parsed.push((bag, bags));
    }

    Some(parsed)
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> Option<usize> {
    let parsed = parse(input)?;
    let shiny_gold = Bag {
        prefix: "shiny",
        color: "gold",
    };

    let mut queue = VecDeque::new();
    let mut seen_bags = HashSet::new();
    queue.push_back(shiny_gold);
    while let Some(bag) = queue.pop_front() {
        if seen_bags.insert(bag) {
            for (from, to) in &parsed {
                for (b, _) in to {
                    if b == &bag {
                        queue.push_back(*from);
                    }
                }
            }
        }
    }
    Some(seen_bags.len() - 1)
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> Option<i32> {
    let parsed = parse(input)?;
    let shiny_gold = Bag {
        prefix: "shiny",
        color: "gold",
    };

    let mut queue = VecDeque::new();
    let mut count = 0;
    queue.push_back((shiny_gold, 1));
    while let Some((bag, num)) = queue.pop_front() {
        count += num;
        for (from, to) in &parsed {
            if *from == bag {
                to.iter().for_each(|i| queue.push_back((i.0, i.1 * num)));
            }
        }
    }
    Some(count - 1)
}
