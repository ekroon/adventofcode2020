use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use Position::*;

#[derive(Clone, Default)]
struct Tile {
    id: usize,
    grid: Vec<Vec<char>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Position {
    Top,
    Left,
    Bottom,
    Right,
}

type Grid = Vec<Vec<Tile>>;

impl Tile {
    fn from_str(s: &str) -> Self {
        let mut lines = s.lines();
        let id = lines.next().unwrap()[5..9].parse().unwrap();
        Self {
            id,
            grid: lines.map(|s| s.chars().collect()).collect(),
        }
    }

    fn all_possible_edges(&self) -> Vec<Vec<char>> {
        let mut result = vec![];
        result.push(self.grid[0].clone());
        result.push(self.grid[9].clone());
        let (left, right) = self
            .grid
            .iter()
            .map(|vec| (vec[0], vec[vec.len() - 1]))
            .unzip();
        result.push(left);
        result.push(right);
        result
            .into_iter()
            .flat_map(|vec| [vec.clone(), vec.into_iter().rev().collect()].to_vec())
            .collect()
    }

    fn edge(&self, position: Position) -> Vec<char> {
        match position {
            Top => self.grid[0].clone(),
            Bottom => self.grid[self.grid.len() - 1].clone(),
            Left => self.grid.iter().map(|vec| vec[0]).collect_vec(),
            Right => self.grid.iter().map(|vec| vec[vec.len() - 1]).collect_vec(),
        }
    }

    fn edges(&self) -> Vec<(Position, Vec<char>)> {
        let mut result = vec![];
        for position in [Top, Bottom, Left, Right].iter() {
            result.push((*position, self.edge(*position)))
        }
        result
    }

    fn flip(&self) -> Self {
        let mut new_grid = self.grid.clone();
        new_grid.reverse();
        Self {
            id: self.id,
            grid: new_grid,
        }
    }

    fn rotate(&self) -> Self {
        let height = self.grid.len();
        let width = self.grid[0].len();
        let mut new_grid = self.grid.clone();
        for (i, j) in (0..height).cartesian_product(0..width) {
            new_grid[j][width - i - 1] = self.grid[i][j];
        }
        Self {
            id: self.id,
            grid: new_grid,
        }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Id: {}", self.id)?;
        for row in &self.grid {
            writeln!(
                f,
                "{}",
                row.iter().fold(String::new(), |mut acc, c| {
                    acc.push(*c);
                    acc
                })
            )?;
        }
        Ok(())
    }
}

fn parse(input: &str) -> impl Iterator<Item = Tile> + Clone + '_ {
    input.split("\n\n").map(Tile::from_str)
}

fn find_corner_ids(tiles: impl IntoIterator<Item = Tile>) -> impl Iterator<Item = usize> {
    border_map(tiles)
        .into_iter()
        .filter(|(_, vec)| vec.len() == 1) // ids with unique sides
        .fold(HashMap::new(), |mut acc, (_, ids)| {
            *acc.entry(ids[0]).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .filter(|(_, length)| *length == 4) // ids with 4 unique sides are on corner (reversed and original)
        .map(|(id, _)| id)
}

fn border_map(tiles: impl IntoIterator<Item = Tile>) -> HashMap<Vec<char>, Vec<usize>> {
    tiles.into_iter().fold(HashMap::new(), |mut acc, tile| {
        for edge in tile.all_possible_edges() {
            acc.entry(edge).or_insert(vec![]).push(tile.id)
        }
        acc
    })
}

fn all_border_positions(tiles: &[Tile]) -> HashMap<(Position, Vec<char>), Vec<Tile>> {
    tiles.iter().fold(HashMap::new(), |mut acc, tile| {
        let original_tile = tile.clone();
        for f in [|tile: &Tile| tile.clone(), Tile::flip].iter() {
            let mut tile = original_tile.clone();
            tile = f(&tile);
            for _ in 0..4 {
                let positions = tile.edges();
                for position in positions {
                    let entry = acc.entry(position).or_insert(vec![]);
                    entry.push(tile.clone());
                }
                tile = tile.rotate();
            }
        }
        acc
    })
}

fn find_monsters(image: &[Vec<char>], monster_coords: &HashSet<(usize, usize)>) -> usize {
    let hash_positions = image
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &c)| c == '#')
                .map(move |(j, _)| (i, j))
        })
        .collect::<HashSet<_>>();
    hash_positions
        .iter()
        .filter(|(i, j)| {
            monster_coords
                .iter()
                .map(|(a, b)| (i + a, j + b))
                .all(|pos| hash_positions.contains(&pos))
        })
        .count()
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> usize {
    let tiles = parse(input);
    find_corner_ids(tiles).product()
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> usize {
    let tiles = parse(input).collect_vec();
    let corner_id = find_corner_ids(tiles.clone()).next().unwrap();
    let dimension = (tiles.len() as f64).sqrt() as usize;
    let mut grid: Grid = vec![vec![Default::default(); dimension]; dimension];
    let all_possible_borders = all_border_positions(&tiles);

    let mut top_left = tiles.iter().find(|t| t.id == corner_id).unwrap().clone();
    loop {
        if top_left.edges().iter().all(|edge| match edge {
            (Top, _) | (Left, _) => all_possible_borders.get(edge).unwrap().len() == 1,
            _ => true,
        }) {
            break;
        } else {
            top_left = top_left.rotate();
        }
    }
    grid[0][0] = top_left;
    for row in 0..dimension {
        'inner: for col in 0..dimension {
            if row == 0 && col == 0 {
                continue 'inner; // skip top_left
            }
            let (neighbour, neighbour_position, position) = if col == 0 {
                (&grid[row - 1][col], Bottom, Top)
            } else {
                (&grid[row][col - 1], Right, Left)
            };
            let neighbour_edge = neighbour.edge(neighbour_position);
            let possible_tiles = all_possible_borders
                .get(&(position, neighbour_edge))
                .unwrap()
                .iter()
                .filter(|tile| tile.id != neighbour.id)
                .collect_vec();
            grid[row][col] = possible_tiles[0].clone();
        }
    }

    grid.iter_mut().for_each(|line| {
        line.iter_mut().for_each(|tile| {
            tile.grid.remove(9);
            tile.grid.remove(0);
            tile.grid.iter_mut().for_each(|line| {
                line.remove(9);
                line.remove(0);
            })
        })
    });

    let tile_dimension = grid[0][0].grid.len();
    let mut map: Vec<Vec<char>> = vec![vec![]; tile_dimension * dimension];
    for (i, j) in (0..dimension).cartesian_product(0..dimension) {
        let tile = &grid[i][j];
        for (index, line) in tile.grid.iter().enumerate() {
            map[i * tile_dimension + index].extend(line);
        }
    }

    let mut map = Tile { id: 0, grid: map };
    let total_hash = map.grid.iter().flatten().filter(|c| **c == '#').count();

    let monster = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ]
    .iter()
    .enumerate()
    .flat_map(|(i, row)| {
        row.chars()
            .enumerate()
            .filter(|&(_, c)| c == '#')
            .map(move |(j, _)| (i, j))
    })
    .collect::<HashSet<_>>();

    let result = loop {
        match find_monsters(&map.grid, &monster) {
            0 => map = map.rotate(),
            n => break n,
        }
    };
    total_hash - result * monster.len()
}
