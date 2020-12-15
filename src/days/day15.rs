use aoc_runner_derive::aoc;

const PART1_SIZE: u32 = 2020;
const PART2_SIZE: u32 = 30000000;

pub fn solve(input: &str, seen: &mut [u32]) -> u32 {
    let mut turn = 0;
    let mut last_num = 0;
    for (i, num) in input.split(',').enumerate() {
        if i > 0 {
            seen[last_num] = turn;
        }
        last_num = num.parse().unwrap();
        turn += 1;
    }

    let until = seen.len() as u32;
    while turn < until as u32 {
        match seen[last_num] {
            0 => {
                seen[last_num] = turn;
                last_num = 0;
            }
            last_turn => {
                seen[last_num] = turn;
                last_num = (turn - last_turn) as usize;
            }
        }
        turn += 1;
    }

    last_num as u32
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> u32 {
    let mut seen = [0u32; PART1_SIZE as usize];
    solve(input, &mut seen)
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> u32 {
    let mut seen = vec![0u32; PART2_SIZE as usize];
    solve(input, &mut seen)
}
