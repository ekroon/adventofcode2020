use aoc_runner_derive::aoc;

type Parsed = Vec<usize>;

pub fn parse(input: &str) -> Parsed {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> Option<usize> {
    let mut parsed = parse(input);
    parsed.push(0);
    parsed.sort_unstable();
    let max = { parsed.last()? + 3 };
    parsed.push(max);
    let result = parsed.windows(2).fold((0, 0, 0), |mut acc, pair| {
        match pair[1] - pair[0] {
            1 => acc.0 += 1,
            2 => acc.1 += 1,
            3 => acc.2 += 1,
            _ => unreachable!(),
        }
        acc
    });

    println!("{:?}", result);
    Some(result.0 * result.2)
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> Option<usize> {
    let mut parsed = parse(input);
    parsed.push(0);
    parsed.sort_unstable();
    let max = { parsed.last()? + 3 };
    parsed.push(max);
    let result = parsed.windows(3).fold((1, 0), |(result, current), pair| {
        match (pair[1] - pair[0], pair[2] - pair[1]) {
            (1, 1) => (result, current + 1),
            _ => match current {
                0 => (result, 0),
                1 => (result * 2, 0),
                2 => (result * 4, 0),
                3 => (result * 7, 0),
                _ => unreachable!(),
            },
        }
    });

    Some(result.0)
}
