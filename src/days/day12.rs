use aoc_runner_derive::aoc;

type Point = (isize, isize);

pub fn rotate<F>(next: F, coord: Point, times: isize) -> Point
where
    F: Fn(Point) -> Point,
{
    (0..times).fold(coord, |coord, _| next(coord))
}

pub fn rotate_left(coord: Point, amount: isize) -> Point {
    rotate(|point| (-point.1, point.0), coord, amount / 90)
}

pub fn rotate_right(coord: Point, amount: isize) -> Point {
    rotate(|point| (point.1, -point.0), coord, amount / 90)
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> Option<isize> {
    let ((x, y), _) = input
        .lines()
        .fold(((0, 0), (1, 0)), |(coord, direction), step| {
            let action = &step[0..1];
            let amount = step[1..].parse::<isize>().unwrap();
            let (x, y) = coord;
            let (x1, y1) = direction;
            match action {
                "N" => ((x, y + amount), direction),
                "S" => ((x, y - amount), direction),
                "W" => ((x - amount, y), direction),
                "E" => ((x + amount, y), direction),
                "F" => ((x + x1 * amount, y + y1 * amount), direction),
                "L" => (coord, rotate_left(direction, amount)),
                "R" => (coord, rotate_right(direction, amount)),
                _ => unreachable!(),
            }
        });

    Some(x.abs() + y.abs())
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> Option<isize> {
    let ((x, y), _) = input
        .lines()
        .fold(((0, 0), (10, 1)), |(coord, waypoint), step| {
            let action = &step[0..1];
            let amount = step[1..].parse::<isize>().unwrap();
            let (x, y) = coord;
            let (x1, y1) = waypoint;
            match action {
                "N" => (coord, (x1, y1 + amount)),
                "S" => (coord, (x1, y1 - amount)),
                "W" => (coord, (x1 - amount, y1)),
                "E" => (coord, (x1 + amount, y1)),
                "F" => ((x + x1 * amount, y + y1 * amount), waypoint),
                "L" => (coord, rotate_left(waypoint, amount)),
                "R" => (coord, rotate_right(waypoint, amount)),
                _ => unreachable!(),
            }
        });

    Some(x.abs() + y.abs())
}
