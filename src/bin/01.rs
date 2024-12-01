use chumsky::prelude::*;
use itertools::Itertools;
use tailsome::IntoOption;

advent_of_code::solution!(1);

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    text::int::<char, Simple<_>>(10)
        .map(|s: String| s.parse::<u32>().unwrap())
        .then_ignore(text::whitespace())
        .then(text::int::<char, Simple<_>>(10).map(|s: String| s.parse::<u32>().unwrap()))
        .separated_by(text::newline())
        .parse(input)
        .unwrap()
        .into_iter()
        .unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut a, mut b) = parse(input);
    a.sort();
    b.sort();

    let input = a.into_iter().zip(b).collect::<Vec<_>>();
    println!("{:?}", input);

    input
        .into_iter()
        .map(|(a, b)| b.abs_diff(a))
        .sum::<u32>()
        .into_some()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (a, b) = parse(input);

    a.into_iter()
        .map(|a| b.iter().filter(|b| **b == a).count() as u32 * a)
        .sum::<u32>()
        .into_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
