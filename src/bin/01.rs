use chumsky::prelude::*;
use tailsome::IntoOption;

advent_of_code::solution!(1);

fn parse(input: &str) -> Vec<(u32, u32)> {
    text::int::<char, Simple<_>>(10)
        .map(|s: String| s.parse::<u32>().unwrap())
        .then_ignore(text::whitespace())
        .then(text::int::<char, Simple<_>>(10).map(|s: String| s.parse::<u32>().unwrap()))
        .separated_by(text::newline())
        .parse(input)
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    let (mut a, mut b): (Vec<_>, Vec<_>) = input.into_iter().unzip();
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
    None
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
        assert_eq!(result, None);
    }
}
