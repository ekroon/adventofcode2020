use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[aoc(day14, part1)]
pub fn part1(input: &str) -> Option<isize> {
    let mut or_mask = 0isize;
    let mut and_mask = 0isize;
    let mut mem = HashMap::new();
    for line in input.lines() {
        let mut split = line.split_ascii_whitespace();
        let first = split.next().unwrap();
        match first {
            "mask" => {
                split.next();
                let str_mask = split.next().unwrap();
                or_mask = isize::from_str_radix(&str_mask.replace('X', "0"), 2).unwrap();
                and_mask = isize::from_str_radix(
                    &str_mask
                        .chars()
                        .map(|c| match c {
                            'X' => '1',
                            _ => '0',
                        })
                        .collect::<String>(),
                    2,
                )
                .unwrap();
            }
            _ => {
                let address = first
                    .strip_prefix("mem[")
                    .unwrap()
                    .strip_suffix("]")
                    .unwrap()
                    .parse::<isize>()
                    .unwrap();
                split.next();
                let value = split.next().unwrap().parse::<isize>().unwrap();
                mem.insert(address, (value & and_mask) | or_mask);
            }
        }
    }
    Some(mem.values().sum())
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> Option<isize> {
    let mut or_mask = 0isize;
    let mut x_positions = vec![];
    let mut mem = HashMap::new();
    for line in input.lines() {
        let mut split = line.split_ascii_whitespace();
        let first = split.next().unwrap();
        match first {
            "mask" => {
                split.next();
                let str_mask = split.next().unwrap();
                x_positions = str_mask
                    .chars()
                    .rev()
                    .enumerate()
                    .filter_map(|(i, c)| if c == 'X' { (i as isize).into() } else { None })
                    .collect();
                or_mask = isize::from_str_radix(
                    &str_mask
                        .chars()
                        .map(|c| match c {
                            'X' => '0',
                            _ => c,
                        })
                        .collect::<String>(),
                    2,
                )
                .unwrap();
            }
            _ => {
                let address = first
                    .strip_prefix("mem[")
                    .unwrap()
                    .strip_suffix("]")
                    .unwrap()
                    .parse::<isize>()
                    .unwrap()
                    | or_mask;

                split.next();
                let value = split.next().unwrap().parse::<isize>().unwrap();
                for ones in 0..2isize.pow(x_positions.len() as u32) {
                    // ones = 000 001 010 011 etc in binary
                    let mut address = address;
                    for (i, position) in x_positions.iter().enumerate() {
                        // check 1 at position i in ones
                        if 1 << i & ones != 0 {
                            address |= 1 << position;
                        } else {
                            address &= !(1 << position);
                        }
                    }
                    mem.insert(address, value);
                }
            }
        }
    }
    Some(mem.values().sum())
}
