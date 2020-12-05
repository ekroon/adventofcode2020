use aoc_runner_derive::aoc;
use itertools::Itertools;

type Seat = usize;

pub fn parse_ticket(line: &str) -> Seat {
    let (mut rstart, mut rend, mut cstart, mut cend) = (0, 128, 0, 8);
    for char in line.chars() {
        match char {
            'F' => rend -= (rend - rstart) / 2,
            'B' => rstart += (rend - rstart) / 2,
            'L' => cend -= (cend - cstart) / 2,
            'R' => cstart += (cend - cstart) / 2,
            _ => {}
        }
    }
    rstart * 8 + cstart
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> Option<usize> {
    input.lines().map(parse_ticket).max()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> Option<usize> {
    let mut tickets = input.lines().map(parse_ticket).collect_vec();
    tickets.sort_unstable();
    tickets.windows(2).find_map(|pair| {
        if pair[0] + 2 == pair[1] {
            Some(pair[0] + 1)
        } else {
            None
        }
    })
}
