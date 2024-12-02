use chumsky::prelude::*;
use itertools::Itertools;
use tailsome::IntoOption;

advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<Vec<u32>> {
    advent_of_code::int::<u32>()
        .separated_by(just(' '))
        .at_least(1)
        .separated_by(text::newline())
        .parse(input)
        .unwrap()
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);

    let input: Vec<Vec<i64>> = input
        .into_iter()
        .map(|l| {
            l.into_iter()
                .tuple_windows()
                .map(|(a, b)| b as i64 - a as i64)
                .collect()
        })
        .filter(|l: &Vec<i64>| {
            l.iter().copied().all(|r| (1..=3).contains(&r))
                || l.iter().copied().all(|r| (-3..=-1).contains(&r))
        })
        .collect();

    input.len().into_some()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
