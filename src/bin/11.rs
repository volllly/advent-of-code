use std::{str::FromStr, sync::Arc};

use chumsky::prelude::*;
use dashmap::DashMap;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use tailsome::IntoOption;

advent_of_code::solution!(11);

#[derive(Default)]
struct SolutionCache(Arc<DashMap<u64, Vec<u64>>>);

impl Clone for SolutionCache {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[derive(Default)]
struct CycleCache(Arc<DashMap<(u64, usize), u64>>);

impl Clone for CycleCache {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

struct Puzzle {
    stones: Vec<u64>,
    solution_cache: SolutionCache,
    cycle_cache: CycleCache,
}

impl Puzzle {
    fn from(from: Vec<u64>) -> Self {
        Self {
            stones: from,
            solution_cache: Default::default(),
            cycle_cache: Default::default(),
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
    fn blink(&mut self, times: usize) -> u64 {
        let sum = self
            .stones
            .par_iter()
            .map(|stone| {
                Self::evaluate(
                    *stone,
                    self.solution_cache.clone(),
                    self.cycle_cache.clone(),
                    times,
                )
            })
            .sum();

        sum
    }

    fn evaluate(
        stone: u64,
        solution_cache: SolutionCache,
        cycle_cache: CycleCache,
        times: usize,
    ) -> u64 {
        if times == 0 {
            return 1;
        }
        if let Some(cached) = cycle_cache.0.get(&(stone, times)) {
            return *cached;
        }

        let new = if let Some(cached) = solution_cache.0.get(&stone) {
            cached.clone()
        } else {
            let new = if stone == 0 {
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
            };

            solution_cache.0.insert(stone, new.clone());
            new
        };

        new.into_par_iter()
            .map(|stone| {
                let cycle = Self::evaluate(
                    stone,
                    solution_cache.clone(),
                    cycle_cache.clone(),
                    times - 1,
                );

                cycle_cache.0.entry((stone, times - 1)).or_insert(cycle);

                cycle
            })
            .sum::<u64>()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut puzzle = Puzzle::from_str(input).unwrap();
    puzzle.blink(25).into_some()
}

pub fn part_two(input: &str) -> Option<u64> {
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
        assert_eq!(result, Some(65601038650482));
    }
}
