use aoc_runner_derive::aoc;
use Square::{Empty, Floor, Full};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Square {
    Floor,
    Full,
    Empty,
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1isize, -1isize),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn parse(input: &str) -> Option<Vec<Vec<Square>>> {
    Some(
        input
            .lines()
            .map(|line| {
                line.chars().map(|c| match c {
                    'L' => Empty,
                    '#' => Full,
                    '.' => Floor,
                    _ => unreachable!(),
                })
            })
            .map(Iterator::collect)
            .collect(),
    )
}

fn count_full_neighbours(plan: &Vec<Vec<Square>>, x: usize, y: usize) -> usize {
    let mut count = 0;
    for (y1, x1) in &DIRECTIONS {
        if let Some(line) = plan.get((y as isize + y1) as usize) {
            if let Some(square) = line.get((x as isize + x1) as usize) {
                if *square == Full {
                    count += 1;
                }
            }
        }
    }
    count
}

fn count_full_neighbours_part2(plan: &Vec<Vec<Square>>, x: usize, y: usize) -> usize {
    let mut count: usize = 0;
    for (y1, x1) in &DIRECTIONS {
        let (mut x, mut y) = (x, y);
        'inner: loop {
            y = (y as isize + y1) as usize;
            x = (x as isize + x1) as usize;
            if let Some(line) = plan.get(y) {
                if let Some(square) = line.get(x) {
                    if *square == Full {
                        count += 1;
                        break 'inner;
                    } else if *square == Empty {
                        break 'inner;
                    }
                } else {
                    break 'inner;
                }
            } else {
                break 'inner;
            }
        }
    }
    count
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> Option<usize> {
    let mut plan = parse(input)?;

    let mut changed = 1;
    while changed > 0 {
        changed = 0;
        let mut new_plan = plan.clone();
        for (y, row) in plan.iter().enumerate() {
            for (x, square) in row.iter().enumerate() {
                match square {
                    Floor => continue,
                    Empty => {
                        if count_full_neighbours(&plan, x, y) == 0 {
                            new_plan[y][x] = Full;
                            changed += 1;
                        }
                    }
                    Full => {
                        if count_full_neighbours(&plan, x, y) >= 4 {
                            new_plan[y][x] = Empty;
                            changed += 1;
                        }
                    }
                }
            }
        }
        plan = new_plan;
    }

    Some(plan.iter().flatten().filter(|s| **s == Full).count())
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> Option<usize> {
    let mut plan = parse(input)?;

    let mut changed = 1;
    while changed > 0 {
        changed = 0;
        let mut new_plan = plan.clone();
        for (y, row) in plan.iter().enumerate() {
            for (x, square) in row.iter().enumerate() {
                match square {
                    Floor => continue,
                    Empty => {
                        if count_full_neighbours_part2(&plan, x, y) == 0 {
                            new_plan[y][x] = Full;
                            changed += 1;
                        }
                    }
                    Full => {
                        if count_full_neighbours_part2(&plan, x, y) >= 5 {
                            new_plan[y][x] = Empty;
                            changed += 1;
                        }
                    }
                }
            }
        }
        plan = new_plan;
    }

    Some(plan.iter().flatten().filter(|s| **s == Full).count())
}
