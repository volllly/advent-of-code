use chumsky::{
    prelude::{any, just, take_until},
    Parser,
};
use tailsome::IntoOption;

advent_of_code::solution!(3);

fn parse(input: &str) -> Vec<(u32, u32)> {
    take_until(
        just("mul")
            .ignore_then(just('('))
            .ignore_then(advent_of_code::int::<u32>())
            .then_ignore(just(','))
            .then(advent_of_code::int::<u32>())
            .then_ignore(just(')')),
    )
    .map(|(_, m)| m)
    .repeated()
    .parse(input)
    .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    dbg!(&input);

    input
        .into_iter()
        .map(|(a, b)| a * b)
        .sum::<u32>()
        .into_some()
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
