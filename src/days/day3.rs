use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

type ParsedOutput = Vec<Vec<bool>>;
type ParsedInput<'a> = &'a [Vec<bool>];

#[aoc_generator(day3)]
pub fn generate(input: &str) -> ParsedOutput {
    input
        .lines()
        .map(|s| s.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn solve(input: ParsedInput, (right, down): (usize, usize)) -> Option<usize> {
    let line_length = input.first()?.len();
    let mut x = 0;
    let mut trees = 0;
    for line in input.iter().step_by(down) {
        if *line.get(x)? {
            trees += 1;
        }
        x = (x + right) % line_length
    }
    Some(trees)
}

#[aoc(day3, part1)]
pub fn part1(input: ParsedInput) -> Option<usize> {
    solve(input, (3, 1))
}

#[aoc(day3, part2)]
pub fn part2(input: ParsedInput) -> Option<usize> {
    let mut result = 1;
    for step in &[(1usize, 1usize), (3, 1), (5, 1), (7, 1), (1, 2)] {
        result *= solve(input, *step)?;
    }
    Some(result)
}

#[aoc(day3, part2, wrong)]
pub fn part2_wrong(input: ParsedInput) -> Option<u32> {
    let mut result = 1u32;
    for step in &[(1usize, 1usize), (3, 1), (5, 1), (7, 1), (1, 2)] {
        result *= solve(input, *step)? as u32;
    }
    Some(result)
}
