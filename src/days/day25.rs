use aoc_runner_derive::aoc;

const KEYS: [usize; 2] = [12232269, 19452773];
// const KEYS: [usize; 2] = [5764801, 17807724];
const MOD: usize = 20201227;
const SUBJECT_NUMBER: usize = 7;

#[aoc(day25, part1)]
pub fn part1(_input: &str) -> usize {
    let first = (&KEYS)
        .iter()
        .take(1)
        .map(|key| {
            let mut num = 1;
            let mut loop_num = 0usize;
            while num != *key {
                loop_num += 1;
                num = (num * SUBJECT_NUMBER) % MOD;
            }
            loop_num
        })
        .next()
        .unwrap();
    let mut num = 1;
    for _ in 0..first {
        num = (num * KEYS[1]) % MOD;
    }
    num
}
