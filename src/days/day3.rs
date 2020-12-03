use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::ops::Add;

#[aoc_generator(day3)]
pub fn generate(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|s| s.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect()
}

fn solve(input: &[Vec<bool>], (right, down): (usize, usize)) -> Option<usize> {
    let line_length = input.first()?.len();
    let mut x = 0;
    let mut trees = 0;
    for line in input.iter().step_by(down) {
        if *line.get(x)? {
            trees = trees.add(1);
        }
        x = (x + right) % line_length
    }
    Some(trees)
}

#[aoc(day3, part1)]
pub fn part1(input: &[Vec<bool>]) -> Option<usize> {
    solve(input, (3, 1))
}

#[aoc(day3, part2)]
pub fn part2(input: &[Vec<bool>]) -> Option<usize> {
    let mut result = 1;
    for step in &[(1usize, 1usize), (3, 1), (5, 1), (7, 1), (1, 2)] {
        result *= solve(input, *step)?;
    }
    Some(result)
}

#[aoc(day3, part2, wrong)]
pub fn part2_wrong(input: &[Vec<bool>]) -> Option<i32> {
    let mut result = 1i32;
    for step in &[(1usize, 1usize), (3, 1), (5, 1), (7, 1), (1, 2)] {
        result *= solve(input, *step)? as i32;
    }
    Some(result)
}
