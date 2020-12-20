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
    rule: &[usize],
    start_at: usize,
) -> (bool, usize) {
    let mut new_start_at = start_at;
    for n in rule {
        let (result, index) = is_match(rules, line, *n, new_start_at);
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
) -> (bool, usize) {
    let rule = &rules.get(&index).unwrap();
    if start_at >= line.len() {
        return (false, start_at);
    }
    match rule {
        Rule::Char(char) => {
            if line[start_at..=start_at] == **char {
                (true, start_at + 1)
            } else {
                (false, start_at)
            }
        }
        Rule::Single(vec) => execute_rule(rules, line, vec, start_at),
        Rule::Multi(v1, v2) => {
            if let (true, new_start_at) = execute_rule(rules, line, v1, start_at) {
                (true, new_start_at)
            } else if let (true, new_start_at) = execute_rule(rules, line, v2, start_at) {
                (true, new_start_at)
            } else {
                (false, start_at)
            }
        }
    }
}

fn parse(rules_input: &str) -> HashMap<usize, Rule> {
    let re = regex!(
        r#"^(?P<index>\d+): ("(?P<char>[a-z]{1})"$|(?P<single>[0-9 ]+)$|(?P<first>[0-9 ]+) \| (?P<second>[0-9 ]+))"#
    );

    rules_input
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
        })
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
    let (rules, input) = input.split("\n\n").collect_tuple().unwrap();

    let rules = parse(rules);

    let mut matching = 0;
    for line in input.lines() {
        if let (true, index) = is_match(&rules, line, 0, 0) {
            if index == line.len() {
                matching += 1;
            }
        }
    }

    matching
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> usize {
    let (rules, input) = input.split("\n\n").collect_tuple().unwrap();

    let rules = parse(rules);

    let mut matching = 0;
    for line in input.lines() {
        // result of modification is two or more 42 (n) followed by max n - 1 31s
        // 0: 8 11 -> (42)+ (42){n} (31){n}
        let mut x42s = 0;
        let mut start_at = 0;
        while let (true, index) = is_match(&rules, line, 42, start_at) {
            x42s += 1;
            start_at = index;
        }
        for _ in 0..x42s - 1 {
            if let (true, index) = is_match(&rules, line, 31, start_at) {
                if index == line.len() {
                    matching += 1;
                    break;
                }
                start_at = index;
            }
        }
    }

    matching
}
