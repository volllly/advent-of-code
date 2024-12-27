use std::{collections::HashMap, str::FromStr};

use chumsky::{chain::Chain, prelude::*};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use itertools::Itertools;
use tailsome::IntoOption;

advent_of_code::solution!(11);

struct Puzzle {
    stones: Stones,
    cache: HashMap<u64, Vec<u64>>,
}

impl Puzzle {
    fn from(from: Vec<u64>) -> Self {
        Self {
            stones: Stones(from),
            cache: Default::default(),
        }
    }
}

impl FromStr for Puzzle {
    type Err = Vec<chumsky::error::Simple<char>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        advent_of_code::int()
            .separated_by(just(' '))
            .at_least(1)
            .map(Puzzle::from)
            .parse(s)
    }
}

impl Puzzle {
    fn blink(&mut self, times: usize) -> usize {
        let progress = MultiProgress::new();
        let blink_progress = ProgressBar::new(times as u64);
        blink_progress.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] [{wide_bar}] ({pos}/{len}, ETA {eta}) {msg}",
            )
            .unwrap(),
        );
        progress.add(blink_progress.clone());

        let mut stones = self.stones.clone();
        for _ in 0..times {
            let stone_progress = ProgressBar::new(stones.0.len() as u64);
            stone_progress.set_style(
                ProgressStyle::with_template(
                    "[{elapsed_precise}] [{wide_bar}] ({pos}/{len}, ETA {eta})",
                )
                .unwrap(),
            );
            progress.add(stone_progress.clone());

            stones = stones.next(stone_progress.clone(), &mut self.cache);

            progress.remove(&stone_progress);
            blink_progress.inc(1);
            blink_progress.set_message(format!("[cached: {}]", self.cache.len()));
        }
        blink_progress.finish();
        stones.0.len()
    }
}

#[derive(Clone)]
struct Stones(Vec<u64>);

impl Stones {
    fn next(self, progress: ProgressBar, cache: &mut HashMap<u64, Vec<u64>>) -> Stones {
        let mut next = Vec::<u64>::with_capacity(self.len() * 2);

        for stone in self.0 {
            let new = cache.entry(stone).or_insert_with(|| {
                if stone == 0 {
                    vec![1]
                } else {
                    let digits = stone.ilog10();
                    if digits % 2 == 1 {
                        let cutoff = 10i32.pow((digits + 1) / 2) as u64;
                        let left = stone / cutoff;
                        let right = stone - left * cutoff;
                        vec![left, right]
                    } else {
                        vec![stone * 2024]
                    }
                }
            });
            next.extend_from_slice(new);
            progress.inc(1);
        }
        progress.finish();
        Stones(next)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut puzzle = Puzzle::from_str(input).unwrap();
    puzzle.blink(25).into_some()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut puzzle = Puzzle::from_str(input).unwrap();
    puzzle.blink(75).into_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
