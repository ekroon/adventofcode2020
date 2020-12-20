use aoc_runner_derive::aoc;
use itertools::Itertools;
use regex_macro::regex;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Rule<'a> {
    Char(&'a str),
    Single(Vec<usize>),
    Multi(Vec<usize>, Vec<usize>),
}

fn execute_rule(
    rules: &HashMap<usize, Rule>,
    line: &str,
    rule: &Vec<usize>,
    start_at: usize,
    followed: Vec<usize>,
) -> (bool, usize) {
    let mut new_start_at = start_at;
    for n in rule {
        let (result, index) = is_match(rules, line, *n, new_start_at, followed.clone());
        if !result {
            return (false, start_at);
        }
        new_start_at = index;
    }
    (true, new_start_at)
}

fn is_match(
    rules: &HashMap<usize, Rule>,
    line: &str,
    index: usize,
    start_at: usize,
    followed: Vec<usize>,
) -> (bool, usize) {
    let rule = &rules.get(&index).unwrap();
    if start_at >= line.len() {
        return (false, start_at);
    }
    let mut followed = followed;
    followed.push(index);
    match rule {
        Rule::Char(char) => {
            if line[start_at..=start_at] == **char {
                println!(
                    "Matched char {} at position {} via {:?}",
                    char, start_at, followed
                );
                (true, start_at + 1)
            } else {
                println!(
                    "Didn't match char {} at position {} via {:?}",
                    char, start_at, followed
                );
                (false, start_at)
            }
        }
        Rule::Single(vec) => execute_rule(rules, line, vec, start_at, followed),
        Rule::Multi(v1, v2) => {
            if let (true, new_start_at) = execute_rule(rules, line, v1, start_at, followed.clone())
            {
                (true, new_start_at)
            } else if let (true, new_start_at) =
                execute_rule(rules, line, v2, start_at, followed.clone())
            {
                (true, new_start_at)
            } else {
                (false, start_at)
            }
        }
    }
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
    let (rules, input) = input.split("\n\n").collect_tuple().unwrap();

    let re = regex!(
        r#"^(?P<index>\d+): ("(?P<char>[a-z]{1})"$|(?P<single>[0-9 ]+)$|(?P<first>[0-9 ]+) \| (?P<second>[0-9 ]+))"#
    );

    let rules = rules
        .lines()
        .fold(HashMap::<usize, Rule>::new(), |mut rules, line| {
            if let Some(captures) = re.captures(line) {
                let index = captures.name("index").unwrap().as_str().parse().unwrap();
                rules.insert(
                    index,
                    if let Some(char) = captures.name("char") {
                        Rule::Char(char.as_str())
                    } else if let Some(single) = captures.name("single") {
                        Rule::Single(
                            single
                                .as_str()
                                .split_whitespace()
                                .map(|s| s.parse().unwrap())
                                .collect(),
                        )
                    } else {
                        let first = captures
                            .name("first")
                            .unwrap()
                            .as_str()
                            .split_whitespace()
                            .map(|s| s.parse().unwrap())
                            .collect();
                        let second = captures
                            .name("second")
                            .unwrap()
                            .as_str()
                            .split_whitespace()
                            .map(|s| s.parse().unwrap())
                            .collect();
                        Rule::Multi(first, second)
                    },
                );
            }

            rules
        });

    let mut matching = 0;
    for line in input.lines() {
        if let (true, index) = is_match(&rules, line, 0, 0, vec![]) {
            if index == line.len() {
                matching += 1;
            }
        }
    }

    matching
}
