use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::Add;

#[aoc(day16, part1)]
pub fn part1(input: &str) -> usize {
    let lines = input.lines().collect_vec();
    let mut parts = lines.split(|s| s.trim().is_empty());

    let ranges = parts.next().unwrap();
    let mut valid_nums = HashSet::<usize>::new();
    for line in ranges {
        line.split(':')
            .dropping(1)
            .flat_map(|l| l.trim().split(" or "))
            .for_each(|range| {
                let mut range = range.split('-');
                let from = range.next().unwrap().parse::<usize>().unwrap();
                let to = range.next().unwrap().parse::<usize>().unwrap();
                for i in from..=to {
                    valid_nums.insert(i);
                }
            })
    }

    parts.next(); // drop own ticket

    let invalid_nums = parts.next().unwrap().iter().dropping(1).flat_map(|ticket| {
        ticket.trim().split(',').filter_map(|s| {
            let parsed = s.parse::<usize>().unwrap();
            if !valid_nums.contains(&parsed) {
                Some(parsed)
            } else {
                None
            }
        })
    });

    invalid_nums.fold(0, Add::add)
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> usize {
    let lines = input.lines().collect_vec();
    let mut parts = lines.split(|s| s.trim().is_empty());

    let mut all_field_names = Vec::new();
    let ranges = parts.next().unwrap();
    let mut valid_nums = HashMap::<_, Vec<&str>>::new();
    for line in ranges {
        let mut split = line.split(':');

        let name = split.next().unwrap();
        all_field_names.push(name);

        split
            .flat_map(|l| l.trim().split(" or "))
            .for_each(|range| {
                let mut range = range.split('-');
                let from = range.next().unwrap().parse::<usize>().unwrap();
                let to = range.next().unwrap().parse::<usize>().unwrap();

                for i in from..=to {
                    valid_nums.entry(i).or_default().push(name);
                }
            })
    }

    let ticket = parts
        .next()
        .unwrap()
        .iter()
        .dropping(1) // header
        .flat_map(|line| line.split(',').map(|n| n.parse().unwrap()))
        .collect::<Vec<usize>>();

    let acc = vec![all_field_names.clone(); ticket.len()];
    let mut possible_field_names = parts
        .next()
        .unwrap()
        .iter()
        .dropping(1) // header
        .filter_map(|t| {
            let mut result = vec![];
            for n in t.split(',').map(|s| s.parse::<usize>().unwrap()) {
                if !valid_nums.contains_key(&n) {
                    return None;
                }
                result.push(valid_nums.get(&n).unwrap());
            }
            Some(result)
        })
        .fold(acc, |mut acc, current_ticket| {
            for (i, vec) in current_ticket.iter().enumerate() {
                acc[i].retain(|v| vec.contains(v));
            }
            acc
        });

    let mut sum = 1;
    for _ in 0..possible_field_names.len() {
        let (index, current_one) = possible_field_names
            .iter()
            .enumerate()
            .find(|s| s.1.len() == 1)
            .unwrap();
        let current_one = current_one.clone();
        if current_one[0].starts_with("departure") {
            sum *= ticket[index];
        }
        for set in possible_field_names.iter_mut() {
            set.retain(|v| !current_one.contains(v));
        }
    }

    sum
}
