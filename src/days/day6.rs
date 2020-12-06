use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> Option<usize> {
    let mut sum = 0;
    let mut questions = 0u32;
    for group in input.split("\n\n") {
        for char in group.as_bytes() {
            if *char != b'\n' {
                questions |= 1 << (char - b'a')
            }
        }
        sum += questions.count_ones();
        questions = 0;
    }
    Some(sum as usize)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> Option<usize> {
    let mut sum = 0;
    let mut questions = 0u32;
    let mut person_questions = 0u32;
    for group in input.split("\n\n") {
        for (i, person) in group.lines().enumerate() {
            for char in person.as_bytes() {
                if *char != b'\n' {
                    person_questions |= 1 << (char - b'a');
                }
            }
            if i == 0 {
                questions = person_questions;
            } else {
                questions &= person_questions;
            }
            person_questions = 0;
        }
        sum += questions.count_ones();
        questions = 0;
    }
    Some(sum as usize)
}
