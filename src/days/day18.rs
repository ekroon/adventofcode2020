use aoc_runner_derive::aoc;
use std::collections::VecDeque;
use Op::*;

#[derive(Copy, Clone)]
enum Op {
    Add,
    Mul,
}

fn calculate_expression(line: &str) -> usize {
    let mut stack = VecDeque::new();
    let mut current_op = None;
    let mut value = 0usize;

    for char in line.chars() {
        match char {
            '(' => {
                stack.push_front((current_op.take(), value));
                value = 0;
            }
            ')' => {
                if let Some(op) = stack.pop_front() {
                    match op {
                        (Some(Add), val) => value += val,
                        (Some(Mul), val) => value *= val,
                        (None, _) => {}
                    }
                }
            }
            '0'..='9' => {
                let val = char.to_digit(10).unwrap() as usize;
                if let Some(op) = current_op.take() {
                    match op {
                        Add => value += val,
                        Mul => value *= val,
                    }
                } else if current_op.is_none() {
                    value = val;
                } else {
                    unreachable!();
                }
            }
            '+' => current_op = Add.into(),
            '*' => current_op = Mul.into(),
            _ => {}
        }
    }
    value
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        total += calculate_expression(line);
    }
    total
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        let mut expression = String::new();
        expression.push('(');
        line.chars().for_each(|c| match c {
            '*' => {
                expression.push_str(") * (");
            }
            ')' | '(' => {
                expression.push(c);
                expression.push(c);
            }
            _ => expression.push(c),
        });
        expression.push(')');
        let value = calculate_expression(&expression);
        total += value;
    }
    total
}
