use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day13, part1)]
pub fn part1(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    let earliest_depart = lines.next().unwrap().parse::<usize>().unwrap();
    let bus_ids = lines
        .next()
        .unwrap()
        .split(',')
        .flat_map(&str::parse)
        .collect::<Vec<usize>>();
    let mut wait = usize::MAX;
    let mut pick = 0usize;
    for id in bus_ids {
        let w = id - (earliest_depart % id);
        if w < earliest_depart {
            wait = w;
            pick = id;
        }
    }
    Some(pick * wait)
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    lines.next(); // drop first
    let mut bus_ids = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|s| {
            if let Ok(n) = s.1.parse::<usize>() {
                (s.0 + 1, n).into() // + 1 to prevent extra modulo on line 50
            } else {
                None
            }
        })
        .sorted_by_key(|v| v.1)
        .rev();

    let (index, id) = bus_ids.next().unwrap();
    let mut step_size = id;
    let mut start_at = id - (index % id);
    for (index, id) in bus_ids {
        for n in (start_at..).step_by(step_size) {
            if n % id == id - (index % id) {
                step_size *= id;
                start_at = n;
                break;
            }
        }
    }

    Some(start_at)
}
