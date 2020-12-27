use aoc_runner_derive::aoc;
use itertools::Itertools;

fn run(nums: &[usize], steps: usize) -> Vec<usize> {
    // 1 2 3 4 5 =>  2 3 4 5 1 index lists next number
    let mut next = vec![0; nums.len() + 1];
    for (&n1, &n2) in nums.iter().tuple_windows() {
        next[n1] = n2;
    }
    next[nums[nums.len() - 1]] = nums[0];

    let max = nums.len();
    let mut current = nums[0];
    for _ in 0..steps {
        let pickup_1 = next[current];
        let pickup_2 = next[pickup_1];
        let pickup_3 = next[pickup_2];
        let next_current = next[pickup_3];

        let mut insert_location = if current >= 2 { current - 1 } else { max };
        while [pickup_1, pickup_2, pickup_3].contains(&insert_location) {
            insert_location = if insert_location >= 2 {
                insert_location - 1
            } else {
                max
            };
        }

        next[pickup_3] = next[insert_location];
        next[insert_location] = pickup_1;
        next[current] = next_current;

        current = next_current;
    }

    next
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> String {
    let nums = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    let next = run(&nums, 100);

    let mut current = 1;
    let mut result = String::new();
    for _ in 0..next.len() - 2 {
        result.push_str(&*next[current].to_string());
        current = next[current];
    }
    result
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> usize {
    let mut nums = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    nums.extend(10..=1_000_000);

    let next = run(&nums, 10_000_000);

    next[1] * next[next[1]]
}
