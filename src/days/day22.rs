use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};

fn parse(input: &str) -> (VecDeque<u32>, VecDeque<u32>) {
    let mut players = input.split("\n\n");
    let player1 = players
        .next()
        .unwrap()
        .lines()
        .dropping(1)
        .map(&str::parse)
        .map(Result::unwrap)
        .collect();
    let player2 = players
        .next()
        .unwrap()
        .lines()
        .dropping(1)
        .map(&str::parse)
        .map(Result::unwrap)
        .collect();
    (player1, player2)
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> u32 {
    let (mut player1, mut player2) = parse(input);

    loop {
        match (player1.pop_front(), player2.pop_front()) {
            (Some(card1), Some(card2)) => match card1.cmp(&card2) {
                Ordering::Greater => {
                    player1.push_back(card1);
                    player1.push_back(card2);
                }
                Ordering::Less => {
                    player2.push_back(card2);
                    player2.push_back(card1);
                }
                _ => unreachable!(),
            },
            (Some(card1), None) => {
                player1.push_front(card1);
                break;
            }
            (None, Some(card2)) => {
                player2.push_front(card2);
                break;
            }
            _ => unreachable!(),
        }
    }
    [player1, player2]
        .iter()
        .map(|player| {
            player
                .iter()
                .rev()
                .enumerate()
                .fold(0, |mut acc, (index, card)| {
                    acc += card * (index + 1) as u32;
                    acc
                })
        })
        .max()
        .unwrap()
}

fn player1_won(player1: VecDeque<u32>, player2: VecDeque<u32>) -> bool {
    let (score1, score2) = [player1, player2]
        .iter()
        .map(|player| {
            player
                .iter()
                .rev()
                .enumerate()
                .fold(0, |mut acc, (index, card)| {
                    acc += card * (index + 1) as u32;
                    acc
                })
        })
        .collect_tuple()
        .unwrap();
    score1 > score2
}

fn play_game(
    mut player1: VecDeque<u32>,
    mut player2: VecDeque<u32>,
    mut seen: HashSet<(VecDeque<u32>, VecDeque<u32>)>,
) -> (VecDeque<u32>, VecDeque<u32>) {
    loop {
        if !seen.insert((player1.clone(), player2.clone())) {
            return (vec![1].into_iter().collect(), vec![].into_iter().collect());
        }
        match (player1.pop_front(), player2.pop_front()) {
            (Some(card1), Some(card2)) => {
                if player1.len() as u32 >= card1 && player2.len() as u32 >= card2 {
                    let mut player1_sub = player1.clone();
                    player1_sub.truncate(card1 as usize);
                    let mut player2_sub = player2.clone();
                    player2_sub.truncate(card2 as usize);
                    let (deck1, deck2) = play_game(player1_sub, player2_sub, HashSet::new());
                    if player1_won(deck1, deck2) {
                        player1.push_back(card1);
                        player1.push_back(card2);
                    } else {
                        player2.push_back(card2);
                        player2.push_back(card1);
                    }
                } else {
                    match card1.cmp(&card2) {
                        Ordering::Greater => {
                            player1.push_back(card1);
                            player1.push_back(card2);
                        }
                        Ordering::Less => {
                            player2.push_back(card2);
                            player2.push_back(card1);
                        }
                        _ => unreachable!(),
                    }
                }
            }
            (Some(card1), None) => {
                player1.push_front(card1);
                break;
            }
            (None, Some(card2)) => {
                player2.push_front(card2);
                break;
            }
            _ => unreachable!(),
        }
    }
    (player1, player2)
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> u32 {
    let (player1, player2) = parse(input);
    let (deck1, deck2) = play_game(player1, player2, HashSet::new());

    [deck1, deck2]
        .iter()
        .map(|player| {
            player
                .iter()
                .rev()
                .enumerate()
                .fold(0, |mut acc, (index, card)| {
                    acc += card * (index + 1) as u32;
                    acc
                })
        })
        .max()
        .unwrap()
}
