use crate::utils;
use aoc_runner_derive::aoc;
use itertools::Itertools;
use serde_scan::scan;
use utils::clamp;

type Parsed<'a> = (
    Vec<&'a str>,
    Vec<(usize, usize, usize, usize)>,
    Vec<usize>,
    Vec<Vec<usize>>,
);

pub fn parse(input: &str) -> Parsed {
    let mut names = vec![];
    let mut ranges = vec![];
    let mut ticket = vec![];
    let mut tickets = vec![];
    let mut part = 0;
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            lines.next(); //skip header
            part += 1;
            continue;
        }
        match part {
            0 => {
                let (name, a, b, c, d): (&str, usize, usize, usize, usize) = scan!(
                    "{}: {}-{} or {}-{}" <- line
                )
                .unwrap();
                names.push(name);
                ranges.push((a, b, c, d))
            }
            1 => ticket = line.split(',').map(|s| s.parse().unwrap()).collect(),
            2 => {
                tickets.push(line.split(',').map(|s| s.parse().unwrap()).collect());
            }
            _ => {}
        }
    }
    (names, ranges, ticket, tickets)
}

fn get_invalid_sum(ranges: &[(usize, usize, usize, usize)], ticket: &[usize]) -> Option<usize> {
    let sum: usize = ticket
        .iter()
        .flat_map(|v| {
            if !ranges
                .iter()
                .any(|(a, b, c, d)| clamp(v, a, b) == v || clamp(v, c, d) == v)
            {
                Some(v)
            } else {
                None
            }
        })
        .sum();

    if sum > 0 {
        Some(sum)
    } else {
        None
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> usize {
    let (_, ranges, _, tickets) = parse(input);
    tickets
        .iter()
        .flat_map(|t| get_invalid_sum(&ranges, t))
        .sum()
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> usize {
    let (names, ranges, ticket, mut tickets) = parse(input);
    tickets.retain(|t| get_invalid_sum(&ranges, t).is_none());

    let mut valid_ticket_pos_to_range = vec![vec![true; names.len()]; tickets[0].len()];

    for ticket in tickets {
        for (i, v) in ticket.iter().enumerate() {
            for (j, (a, b, c, d)) in ranges.iter().enumerate() {
                if !(clamp(v, a, b) == v || clamp(v, c, d) == v) {
                    valid_ticket_pos_to_range[i][j] = false;
                }
            }
        }
    }

    let mut found = 0;
    let mut sum = 1;
    loop {
        let (i, j) = valid_ticket_pos_to_range
            .iter()
            .enumerate()
            .find_map(|(i, v)| {
                let true_values = v.iter().enumerate().filter(|(_, v)| **v).collect_vec();
                if true_values.len() == 1 {
                    Some((i, true_values[0].0))
                } else {
                    None
                }
            })
            .unwrap();

        for column in &mut valid_ticket_pos_to_range {
            column[j] = false;
        }

        found += 1;
        if names[j].starts_with("departure") {
            sum *= ticket[i]
        }
        if found == names.len() {
            break;
        }
    }
    sum
}
