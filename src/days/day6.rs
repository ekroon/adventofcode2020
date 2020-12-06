use aoc_runner_derive::aoc;
use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> Option<usize> {
    let mut sum = 0;
    let mut questions: HashSet<char> = HashSet::new();
    for group in input.split("\n\n") {
        for char in group.chars() {
            if char != '\n' {
                questions.insert(char);
            }
        }
        sum += questions.len();
        questions.clear();
    }
    Some(sum)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> Option<usize> {
    let mut sum = 0;
    let mut questions: HashSet<char> = HashSet::new();
    let mut person_questions: HashSet<char> = HashSet::new();
    for group in input.split("\n\n") {
        for (i, person) in group.lines().enumerate() {
            for char in person.chars() {
                if char != '\n' {
                    person_questions.insert(char);
                }
            }
            if i == 0 {
                questions.extend(&person_questions);
            } else {
                questions = questions.intersection(&person_questions).copied().collect();
            }
            person_questions.clear();
        }
        sum += questions.len();
        questions.clear();
    }
    Some(sum)
}
