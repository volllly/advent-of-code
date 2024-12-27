use chumsky::{
    prelude::{just, take_until},
    Parser,
};
use tailsome::IntoOption;

advent_of_code::solution!(3);

#[derive(Debug)]
enum Operation {
    Mul(u32, u32),
    Do,
    Dont,
}

fn parse(input: &str) -> Vec<Operation> {
    take_until(
        just("mul")
            .ignore_then(just('('))
            .ignore_then(advent_of_code::ints::<u32>())
            .then_ignore(just(','))
            .then(advent_of_code::ints::<u32>())
            .then_ignore(just(')'))
            .map(|(a, b)| Operation::Mul(a, b))
            .or(just("do()").map(|_| Operation::Do))
            .or(just("don't()").map(|_| Operation::Dont)),
    )
    .map(|(_, o)| o)
    .repeated()
    .parse(input)
    .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);

    input
        .into_iter()
        .filter_map(|o| {
            if let Operation::Mul(a, b) = o {
                Some(a * b)
            } else {
                None
            }
        })
        .sum::<u32>()
        .into_some()
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);

    let mut enabled = true;
    let mut sum = 0;
    for operation in input.into_iter() {
        match operation {
            Operation::Mul(a, b) => {
                if enabled {
                    sum += a * b
                }
            }
            Operation::Do => enabled = true,
            Operation::Dont => enabled = false,
        }
    }

    sum.into_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
