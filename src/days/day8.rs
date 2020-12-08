use aoc_runner_derive::aoc;
use std::collections::BTreeSet;

#[derive(Clone)]
pub enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .flat_map(|line| {
            let mut split = line.split_ascii_whitespace();
            let instruction = split.next();
            let num = split.next().unwrap_or("err").parse::<isize>();
            match (instruction, num) {
                (Some("acc"), Ok(num)) => Some(Instruction::Acc(num)),
                (Some("jmp"), Ok(num)) => Some(Instruction::Jmp(num)),
                (Some("nop"), Ok(num)) => Some(Instruction::Nop(num)),
                _ => None,
            }
        })
        .collect()
}

pub fn run_accumulator<F>(
    instructions: &[Instruction],
    mut complete_check: F,
) -> Option<(usize, isize)>
where
    F: FnMut(usize) -> bool,
{
    let mut instruction_counter = 0usize;
    let mut acc = 0isize;

    while complete_check(instruction_counter) {
        let instruction = instructions.get(instruction_counter)?;
        match instruction {
            Instruction::Acc(num) => {
                acc += *num;
                instruction_counter += 1;
            }
            Instruction::Jmp(num) => {
                instruction_counter += *num as usize;
            }
            Instruction::Nop(_) => {
                instruction_counter += 1;
            }
        }
    }
    Some((instruction_counter, acc))
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> Option<isize> {
    let instructions = parse(input);

    let mut seen_instruction_counter = BTreeSet::new();
    Some(
        run_accumulator(&instructions, |counter| {
            seen_instruction_counter.insert(counter)
        })?
        .1,
    )
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> Option<isize> {
    let instructions = parse(input);

    let mut seen_instruction_counter = BTreeSet::new();
    let final_counter = instructions.len();
    (0..instructions.len()).find_map(|i| {
        let mut instructions = instructions.clone();
        seen_instruction_counter.clear();

        match instructions.get(i) {
            Some(Instruction::Jmp(num)) => *instructions.get_mut(i)? = Instruction::Nop(*num),
            Some(Instruction::Nop(num)) => *instructions.get_mut(i)? = Instruction::Jmp(*num),
            _ => return None,
        }
        if let Some((counter, acc)) = run_accumulator(&instructions, |counter| {
            seen_instruction_counter.insert(counter) && counter < final_counter
        }) {
            if counter >= final_counter {
                Some(acc)
            } else {
                None
            }
        } else {
            None
        }
    })
}
