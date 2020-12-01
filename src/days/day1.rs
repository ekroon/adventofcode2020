use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day1)]
pub fn generate(input: &str) -> Vec<i32> {
    let mut parsed = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    parsed.sort_unstable();
    parsed
}

#[aoc(day1, part1, for_loop_split_at)]
pub fn part1(input: &[i32]) -> i32 {
    for i in 1..=input.len() {
        let (first, second) = input.split_at(i);
        let n1 = first.last().unwrap();
        for n2 in second {
            if n1 + n2 == 2020 {
                return n1 * n2;
            }
        }
    }
    0
}

#[aoc(day1, part2, for_loop_split_at)]
pub fn part2(input: &[i32]) -> i32 {
    for i in 1..=input.len() {
        let (first, second) = input.split_at(i);
        let n1 = first.last().unwrap();
        for j in 1..=second.len() {
            let (second, third) = second.split_at(j);
            let n2 = second.last().unwrap();
            if n1 + n2 > 2020 {
                continue;
            }
            for n3 in third {
                if n1 + n2 + n3 == 2020 {
                    return n1 * n2 * n3;
                }
            }
        }
    }
    0
}

#[aoc(day1, part2, for_loop_indexed)]
pub fn part2_for_loop(input: &[i32]) -> i32 {
    for i in 0..input.len() {
        for j in i..input.len() {
            if input[i] + input[j] > 2020 {
                continue;
            }
            for k in j..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    return input[i] * input[j] * input[k];
                }
            }
        }
    }
    0
}

#[aoc(day1, part2, itertools_combinations_copied)]
pub fn part2_itertools_copied(input: &[i32]) -> i32 {
    use itertools::Itertools;
    for combination in input.iter().combinations(3) {
        if combination.iter().copied().sum::<i32>() == 2020 {
            return combination.iter().copied().product();
        }
    }
    0
}

#[aoc(day1, part2, itertools_tuple_combinations)]
pub fn part2_itertools_tuple_combinations(input: &[i32]) -> i32 {
    use itertools::Itertools;
    for (a, b, c) in input.iter().tuple_combinations() {
        if a + b + c == 2020 {
            return a * b * c;
        }
    }
    0
}
