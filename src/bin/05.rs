use chumsky::prelude::*;
use std::{
    collections::{hash_set, HashMap, HashSet},
    path::MAIN_SEPARATOR,
    str::FromStr,
    vec,
};
use tailsome::IntoOption;

advent_of_code::solution!(5);

#[derive(Debug, Default)]
struct Order {
    after: HashMap<u32, HashSet<u32>>,
}

impl From<Vec<(u32, u32)>> for Order {
    fn from(value: Vec<(u32, u32)>) -> Self {
        let mut order = Order::default();

        for (b, a) in value {
            order
                .after
                .entry(b)
                .and_modify(|e| {
                    e.insert(a);
                })
                .or_insert_with(|| HashSet::from_iter([a].into_iter()));
        }

        order
    }
}

impl Order {
    fn get_valid_update(&self, update: &[u32]) -> bool {
        for page in 0..update.len() {
            if let Some(after) = self.after.get(&update[page]) {
                for before in &update[..(page)] {
                    if after.contains(before) {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn correct_update(&mut self, update: &mut Vec<u32>) -> bool {
        let mut valid = true;
        for page in 0..update.len() {
            if let Some(after) = self.after.get(&update[page]) {
                for before in 0..(page) {
                    if after.contains(&update[before]) {
                        update.swap(page, before);
                        valid = false;
                    }
                }
            }
        }

        valid
    }
}

#[derive(Debug)]
struct Puzzle {
    order: Order,
    updates: Vec<Vec<u32>>,
}

impl FromStr for Puzzle {
    type Err = Vec<chumsky::error::Simple<char>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        advent_of_code::int::<u32>()
            .then_ignore(just('|'))
            .then(advent_of_code::int::<u32>())
            .separated_by(text::newline())
            .map(Order::from)
            .then_ignore(text::newline())
            .then_ignore(text::newline())
            .then(
                advent_of_code::int::<u32>()
                    .separated_by(just(','))
                    .at_least(1)
                    .separated_by(text::newline())
                    .at_least(1),
            )
            .map(|(order, updates)| Puzzle { order, updates })
            .parse(s)
    }
}

impl Puzzle {
    fn get_valid_updates(&self) -> Vec<&Vec<u32>> {
        self.updates
            .iter()
            .filter(|update| self.order.get_valid_update(update))
            .collect()
    }

    fn correct_updates(&mut self) -> Vec<&Vec<u32>> {
        let Self { order, updates } = self;
        let mut invalid = Vec::<usize>::new();

        for (i, update) in updates.iter_mut().enumerate() {
            if !order.correct_update(update) {
                invalid.push(i);
            }
        }

        invalid
            .into_iter()
            .map(|i| self.updates.get(i).unwrap())
            .collect()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let puzzle = Puzzle::from_str(input).unwrap();

    let valid = puzzle.get_valid_updates();

    valid
        .into_iter()
        .map(|update| update[update.len() / 2])
        .sum::<u32>()
        .into_some()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut puzzle = Puzzle::from_str(input).unwrap();

    let invalid = puzzle.correct_updates();

    dbg!(&invalid);
    invalid
        .into_iter()
        .map(|update| update[update.len() / 2])
        .sum::<u32>()
        .into_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
