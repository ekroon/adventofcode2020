use aoc_runner_derive::aoc;
use std::collections::{HashMap, HashSet};
use InstructionStep::*;

type Coord = (isize, isize);

enum InstructionStep {
    East,      // e
    SouthEast, // se
    SouthWest, // sw
    West,      // w
    NorthWest, // nw
    NorthEast, // ne
}

impl InstructionStep {
    fn new_coord(&self, coord: Coord) -> Coord {
        let (x, y) = coord;
        match self {
            East => (x + 2, y),
            SouthEast => (x + 1, y + 1),
            SouthWest => (x - 1, y + 1),
            West => (x - 2, y),
            NorthWest => (x - 1, y - 1),
            NorthEast => (x + 1, y - 1),
        }
    }
}

fn parse(input: &str) -> Vec<Vec<InstructionStep>> {
    let mut result = vec![];
    for line in input.lines() {
        let mut instruction_line = vec![];
        let mut chars = line.chars();
        while let Some(char) = chars.next() {
            match char {
                'e' => instruction_line.push(East),
                's' => match chars.next().unwrap() {
                    'e' => instruction_line.push(SouthEast),
                    'w' => instruction_line.push(SouthWest),
                    _ => unreachable!(),
                },
                'w' => instruction_line.push(West),
                'n' => match chars.next().unwrap() {
                    'w' => instruction_line.push(NorthWest),
                    'e' => instruction_line.push(NorthEast),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }

        result.push(instruction_line);
    }
    result
}

fn execute_instructions(instructions: Vec<Vec<InstructionStep>>) -> HashSet<(isize, isize)> {
    let mut tiles = HashSet::<(isize, isize)>::new();
    for instruction in instructions.iter() {
        let coord = instruction
            .iter()
            .fold((0, 0), |acc, step| step.new_coord(acc));
        if let Some(coord) = tiles.get(&coord).copied() {
            tiles.remove(&coord);
        } else {
            tiles.insert(coord);
        }
    }
    tiles
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> usize {
    let instructions = parse(input);
    let tiles = execute_instructions(instructions);
    tiles.len()
}

fn coord_neighbours((x, y): (isize, isize)) -> [(isize, isize); 6] {
    [
        (x + 2, y),
        (x + 1, y + 1),
        (x - 1, y + 1),
        (x - 2, y),
        (x - 1, y - 1),
        (x + 1, y - 1),
    ]
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> usize {
    let instructions = parse(input);
    let mut black_tiles = execute_instructions(instructions);
    let mut black_neighbours_white = HashMap::<Coord, usize>::new();
    let mut to_white = HashSet::new();
    for _ in 0..100 {
        black_neighbours_white.clear();
        to_white.clear();

        {
            for coord in &black_tiles {
                let mut count = 0;
                for neighbour in &coord_neighbours(*coord) {
                    if black_tiles.contains(neighbour) {
                        count += 1;
                    } else {
                        *black_neighbours_white.entry(*neighbour).or_insert(0) += 1;
                    }
                }
                if count == 0 || count > 2 {
                    to_white.insert(*coord);
                }
            }
        }

        for (&coord, &num) in &black_neighbours_white {
            if num == 2 {
                black_tiles.insert(coord);
            }
        }
        for coord in &to_white {
            black_tiles.remove(coord);
        }
    }
    black_tiles.len()
}
