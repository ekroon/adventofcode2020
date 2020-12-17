use aoc_runner_derive::aoc;
use itertools::Itertools;

type Pocket<'a> = &'a [Vec<Vec<bool>>];

fn parse(input: &str) -> Vec<Vec<bool>> {
    let parsed_state = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => {
                        unreachable!()
                    }
                })
                .collect_vec()
        })
        .collect_vec();
    parsed_state
}

fn count_neighbours(grid: Pocket, z: usize, y: usize, x: usize) -> usize {
    let mut neighbours = 0;
    for (z_, ys) in grid[z - 1..=z + 1].iter().enumerate() {
        for (y_, xs) in ys[y - 1..=y + 1].iter().enumerate() {
            for (x_, b) in xs[x - 1..=x + 1].iter().enumerate() {
                if !(x_ == 1 && y_ == 1 && z_ == 1) && *b {
                    neighbours += 1;
                }
            }
        }
    }

    neighbours
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> usize {
    let size = 32usize;
    let mut pocket = vec![vec![vec![false; size]; size]; size];
    let parsed_state = parse(input);
    let dim_pocket = parsed_state.len();

    let mut start = size / 2 - dim_pocket / 2;
    let mut end = size / 2 + dim_pocket / 2;

    parsed_state.into_iter().enumerate().for_each(|(y, vec)| {
        vec.into_iter().enumerate().for_each(|(x, b)| {
            pocket[start + dim_pocket / 2][y + start][x + start] = b;
        })
    });

    let mut cycles = 0;
    let mut active = 0;

    let mut mutations = vec![];

    while cycles < 6 {
        mutations.clear();
        active = 0;
        cycles += 1;
        start -= 1;
        end += 1;
        for z in start..=end {
            for y in start..=end {
                for x in start..=end {
                    let current = pocket[z][y][x];
                    let neighbours = count_neighbours(&pocket, z, y, x);
                    if current && (neighbours == 3 || neighbours == 2) {
                        active += 1;
                    } else if !current && neighbours == 3 {
                        active += 1;
                        mutations.push((z, y, x, true));
                    } else if current {
                        mutations.push((z, y, x, false));
                    }
                }
            }
        }
        for (z, y, x, b) in &mutations {
            pocket[*z][*y][*x] = *b;
        }
    }

    active
}

type Pocket4<'a> = &'a [Vec<Vec<Vec<bool>>>];

fn count_neighbours4(grid: Pocket4, w: usize, z: usize, y: usize, x: usize) -> usize {
    let mut neighbours = 0;
    for (w_, zs) in grid[w - 1..=w + 1].iter().enumerate() {
        for (z_, ys) in zs[z - 1..=z + 1].iter().enumerate() {
            for (y_, xs) in ys[y - 1..=y + 1].iter().enumerate() {
                for (x_, b) in xs[x - 1..=x + 1].iter().enumerate() {
                    if !(x_ == 1 && y_ == 1 && z_ == 1 && w_ == 1) && *b {
                        neighbours += 1;
                    }
                }
            }
        }
    }

    neighbours
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> usize {
    let size = 32usize;
    let mut pocket = vec![vec![vec![vec![false; size]; size]; size]; size];
    let parsed_state = parse(input);
    let dim_pocket = parsed_state.len();

    let mut start = size / 2 - dim_pocket / 2;
    let mut end = size / 2 + dim_pocket / 2;

    parsed_state.into_iter().enumerate().for_each(|(y, vec)| {
        vec.into_iter().enumerate().for_each(|(x, b)| {
            pocket[start + dim_pocket / 2][start + dim_pocket / 2][y + start][x + start] = b;
        })
    });

    let mut cycles = 0;
    let mut active = 0;

    let mut mutations = vec![];

    while cycles < 6 {
        mutations.clear();
        active = 0;
        cycles += 1;
        start -= 1;
        end += 1;
        for w in start..=end {
            for z in start..=end {
                for y in start..=end {
                    for x in start..=end {
                        let current = pocket[w][z][y][x];
                        let neighbours = count_neighbours4(&pocket, w, z, y, x);
                        if current && (neighbours == 3 || neighbours == 2) {
                            active += 1;
                        } else if !current && neighbours == 3 {
                            active += 1;
                            mutations.push((w, z, y, x, true));
                        } else if current {
                            mutations.push((w, z, y, x, false));
                        }
                    }
                }
            }
        }
        for (w, z, y, x, b) in &mutations {
            pocket[*w][*z][*y][*x] = *b;
        }
    }

    active
}
