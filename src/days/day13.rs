use aoc_runner_derive::aoc;
use itertools::Itertools;
use lexical::parse;

#[aoc(day13, part1)]
pub fn part1(input: &str) -> Option<usize> {
    let mut bytes = input.as_bytes().split(|b| *b == b'\n');

    let earliest_depart = parse::<usize, _>(bytes.next().unwrap()).unwrap();
    let bus_ids = bytes
        .next()
        .unwrap()
        .split(|b| *b == b',')
        .flat_map(parse)
        .collect::<Vec<usize>>();
    let mut min_wait = usize::MAX;
    let mut pick = 0usize;
    for id in bus_ids {
        let wait = id - (earliest_depart % id);
        if wait < min_wait {
            min_wait = wait;
            pick = id;
        }
    }
    Some(pick * min_wait)
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> Option<usize> {
    let mut bytes = input.as_bytes().split(|b| *b == b'\n');
    bytes.next(); // drop first
    let mut bus_ids = bytes
        .next()
        .unwrap()
        .split(|b| *b == b',')
        .enumerate()
        .filter_map(|s| {
            if let Ok(n) = parse(s.1) {
                (s.0, n).into()
            } else {
                None
            }
        })
        .sorted_by(|v1, v2| Ord::cmp(&v2.1, &v1.1));

    let (index, id) = bus_ids.next().unwrap();
    let mut step_size = id;
    let mut start_at = id - index % id;
    for (index, id) in bus_ids {
        for n in (start_at..).step_by(step_size) {
            if (n + index) % id == 0 {
                step_size *= id;
                start_at = n;
                break;
            }
        }
    }

    Some(start_at)
}
