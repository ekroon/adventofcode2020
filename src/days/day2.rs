use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use scan_fmt::scan_fmt;

// 8-10 t: ptckdjtsptlmzrktwcw
const SCANLINE: &str = "{}-{} {}: {}";

#[aoc_generator(day2)]
pub fn generate(input: &str) -> Vec<(usize, usize, char, String)> {
    input
        .lines()
        .map(|l| {
            let result = scan_fmt!(l, SCANLINE, usize, usize, char, String);
            result.unwrap()
        })
        .collect::<Vec<_>>()
}

#[aoc(day2, part1)]
pub fn part1(input: &[(usize, usize, char, String)]) -> Option<i32> {
    let mut correct = 0;
    for (from, to, letter, password) in input {
        let count = password.chars().filter(|c| c == letter).count();
        if *from <= count && *to >= count {
            correct += 1;
        }
    }
    Some(correct)
}

#[aoc(day2, part2)]
pub fn part2(input: &[(usize, usize, char, String)]) -> Option<i32> {
    let mut correct = 0;
    for (from, to, letter, password) in input {
        let password = password.get(0..).unwrap();
        if password.chars().nth(from - 1).unwrap() == *letter
            && password.chars().nth(to - 1).unwrap() != *letter
            || password.chars().nth(from - 1).unwrap() != *letter
                && password.chars().nth(to - 1).unwrap() == *letter
        {
            correct += 1;
        }
    }
    Some(correct)
}
