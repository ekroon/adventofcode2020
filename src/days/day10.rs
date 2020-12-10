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
                0 => (result, 0),     // nothing to pick
                1 => (result * 2, 0), // (1 pick 0) + (1 pick 1)
                2 => (result * 4, 0), // (2 pick 0) + (2 pick 1) + (2 pick 2)
                3 => (result * 7, 0), // (3 pick 1) + (3 pick 2) + (3 pick 3)
                _ => unreachable!(),
            },
        }
    });

    Some(result.0)
}

#[aoc(day10, part2, raymond)]
pub fn part2_raymond(input: &str) -> Option<usize> {
    let mut parsed = parse(input);
    parsed.push(0);
    parsed.sort_unstable();
    let max = { parsed.last()? + 3 };
    parsed.push(max);

    let mut paths = vec![0usize; parsed.len()];
    paths[0] = 1;
    let result = parsed
        .iter()
        .enumerate()
        .fold(paths, |mut acc, (index, &jolt)| {
            parsed[index + 1..]
                .iter()
                .enumerate()
                .take_while(|(_, &jolt2)| jolt2 <= jolt + 3)
                .for_each(|(index2, _)| acc[index + 1 + index2] += acc[index]);
            acc
        });

    Some(*result.last().unwrap())
}
