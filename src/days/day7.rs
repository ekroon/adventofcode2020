use aoc_runner_derive::aoc;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd, Default)]
pub struct Bag<'a> {
    prefix: &'a str,
    color: &'a str,
}

const shiny_gold: Bag = Bag {
    prefix: "shiny",
    color: "gold",
};

pub fn parse(input: &str) -> impl Iterator<Item = (Bag, Vec<(i32, Bag)>)> {
    input.lines().map(|line| {
        let mut words = line.split_ascii_whitespace();
        let bag = Bag {
            prefix: words.next().unwrap(),
            color: words.next().unwrap(),
        };
        words.next();
        words.next();
        let mut bags = vec![];
        while let Some(word) = words.next() {
            if word == "no" {
                break;
            }
            bags.push((
                word.parse().unwrap(),
                Bag {
                    prefix: words.next().unwrap(),
                    color: words.next().unwrap(),
                },
            ));
            words.next();
        }
        (bag, bags)
    })
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> Option<usize> {
    let parsed = parse(input).fold(
        HashMap::new(),
        |mut acc: HashMap<Bag, Vec<Bag>>, (from, to)| {
            for (_, to) in to {
                if let Some(bags) = acc.get_mut::<Bag>(&to) {
                    bags.push(from);
                } else {
                    let mut bags = vec![];
                    bags.push(from);
                    acc.insert(to, bags);
                }
            }
            acc
        },
    );

    let mut queue = VecDeque::new();
    let mut seen_bags = HashSet::new();
    queue.push_back(shiny_gold);
    while let Some(bag) = queue.pop_front() {
        if seen_bags.insert(bag) {
            if let Some(bags) = parsed.get(&bag) {
                for bag in bags {
                    queue.push_back(*bag)
                }
            }
        }
    }
    Some(seen_bags.len() - 1)
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> Option<i32> {
    let parsed = parse(input).fold(HashMap::new(), |mut acc, (from, to)| {
        acc.insert(from, to);
        acc
    });

    let mut queue = VecDeque::new();
    let mut count = 0;
    queue.push_back((shiny_gold, 1));
    while let Some((bag, num)) = queue.pop_front() {
        count += num;
        if let Some(to) = parsed.get(&bag) {
            to.iter().for_each(|i| queue.push_back((i.1, i.0 * num)));
        }
    }
    Some(count - 1)
}
