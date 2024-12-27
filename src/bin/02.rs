use chumsky::prelude::*;
use itertools::Itertools;
use tailsome::IntoOption;

advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<Vec<u32>> {
    advent_of_code::ints::<u32>()
        .separated_by(just(' '))
        .at_least(1)
        .separated_by(text::newline())
        .parse(input)
        .unwrap()
}

fn valid(level: &[u32]) -> bool {
    let level = level
        .iter()
        .tuple_windows()
        .map(|(a, b)| *b as i64 - *a as i64)
        .collect::<Vec<_>>();
    level.iter().all(|r| (1..=3).contains(r)) || level.iter().all(|r| (-3..=-1).contains(r))
}

fn valid_dampened(level: &[u32]) -> bool {
    valid(level)
        || (0..level.len()).any(|i| {
            valid(
                level
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| i != *j)
                    .map(|(_, r)| *r)
                    .collect::<Vec<_>>()
                    .as_slice(),
            )
        })
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);

    let input: Vec<Vec<u32>> = input.into_iter().filter(|l| valid(l)).collect();

    input.len().into_some()
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);

    let input: Vec<Vec<u32>> = input.into_iter().filter(|l| valid_dampened(l)).collect();

    input.len().into_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
