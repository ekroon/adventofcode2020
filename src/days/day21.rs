use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Allergen<'a> = &'a str;
type Ingredient<'a> = &'a str;

fn parse(input: &str) -> Vec<(HashSet<Ingredient>, HashSet<Allergen>)> {
    let mut result = vec![];
    input.lines().for_each(|line| {
        let mut parts = line.split(" (contains ");
        let ingredients = parts
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<HashSet<_>>();
        let mut allergens_part = parts.next().unwrap();
        allergens_part = &allergens_part[0..allergens_part.len() - 1];
        let allergens = allergens_part.split(", ").collect();
        result.push((ingredients, allergens));
    });
    result
}

fn allergen_map<'a>(
    list: &[(HashSet<Ingredient<'a>>, HashSet<Allergen<'a>>)],
) -> HashMap<Allergen<'a>, HashSet<Ingredient<'a>>> {
    list.iter().fold(
        HashMap::<Allergen, HashSet<Ingredient>>::new(),
        |mut acc, (ingredients, allergens)| {
            for allergen in allergens.iter() {
                let ingredients_set = ingredients.iter().copied().collect::<HashSet<_>>();
                acc.entry(&allergen)
                    .or_insert_with(|| ingredients_set.clone())
                    .retain(|a| ingredients_set.contains(a))
            }
            acc
        },
    )
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> usize {
    let list = parse(input);
    let allergen_map = allergen_map(&list);
    let possible_allergens = allergen_map
        .values()
        .flatten()
        .copied()
        .collect::<HashSet<Ingredient>>();
    let mut count = 0;
    for (ingredients, _) in list.iter() {
        for ingredient in ingredients.iter() {
            if !possible_allergens.contains(ingredient) {
                count += 1;
            }
        }
    }
    count
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> String {
    let list = parse(input);
    let mut allergen_map = allergen_map(&list);
    let mut mapping = vec![];

    while let Some((&allergen, ingredients)) = allergen_map.iter().find(|(_, v)| v.len() == 1) {
        let ingredient = *ingredients.iter().next().unwrap();
        mapping.push((allergen, ingredient));
        allergen_map
            .values_mut()
            .for_each(|value| value.retain(|&i| i != ingredient));
    }

    mapping.sort_by_key(|(allergen, _)| allergen.to_string());
    mapping
        .iter()
        .map(|(_, ingredient)| ingredient)
        .copied()
        .collect_vec()
        .join(",")
}
