use aoc_runner_derive::aoc;
use itertools::{Itertools, MinMaxResult};
use std::cmp::Ordering;
use std::collections::VecDeque;
use MinMaxResult::MinMax;

const PREAMBLE: usize = 25;

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = usize> + 'a {
    input.lines().map(|line| line.parse().unwrap())
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> Option<usize> {
    let numbers = parse(input);

    numbers
        .enumerate()
        .try_fold(VecDeque::with_capacity(PREAMBLE), |mut acc, (i, num)| {
            if i < PREAMBLE {
                acc.push_back(num);
                Ok(acc)
            } else if acc.iter().any(|&v| acc.contains(&(num - v))) {
                acc.pop_front();
                acc.push_back(num);
                Ok(acc)
            } else {
                Err(num)
            }
        })
        .err()
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> Option<usize> {
    let part1 = part1(input)?;

    let numbers = parse(input).collect::<Vec<_>>();
    let mut from = 0;
    let mut to = 1;
    loop {
        let result: usize = numbers[from..=to].iter().sum();
        match result.cmp(&part1) {
            Ordering::Less => to += 1,
            Ordering::Greater => from += 1,
            Ordering::Equal => break,
        }
    }
    if let MinMax(min, max) = numbers[from..=to].iter().minmax() {
        Some(min + max)
    } else {
        None
    }
}
