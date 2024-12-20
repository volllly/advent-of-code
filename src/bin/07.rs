use chumsky::prelude::*;
use std::{iter, ops::Deref, str::FromStr};
use tailsome::IntoOption;

use advent_of_code::arena::Arena;
advent_of_code::solution!(7);

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl From<u32> for Operator {
    fn from(value: u32) -> Self {
        match value {
            0 => Operator::Add,
            1 => Operator::Multiply,
            2 => Operator::Concatenate,
            _ => panic!("invalid number for operator"),
        }
    }
}

impl Operator {
    fn eval(self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concatenate => format!("{}{}", a, b).parse().unwrap(),
        }
    }
}

struct Permutation(u32, Vec<u32>);

impl Deref for Permutation {
    type Target = Vec<u32>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl Permutation {
    fn from(n: u32, length: usize) -> Self {
        Self(n, Vec::from_iter(iter::repeat_n(0, length)))
    }

    fn next(mut self) -> Self {
        let mut carry = 1;
        for n in 0..self.1.len() {
            self.1[n] += carry;
            carry = 0;
            if self.1[n] >= self.0 {
                self.1[n] = 0;
                carry = 1;
            }
        }

        self
    }

    fn is_zero(&self) -> bool {
        self.1.iter().all(|p| *p == 0)
    }
}

struct Equation {
    result: u64,
    args: Vec<u64>,
}

impl Equation {
    fn is_correct(&self, operators: u32) -> bool {
        let operator_count = self.args.len() - 1;
        let mut permutation = Permutation::from(operators, operator_count);
        loop {
            let operators = permutation.iter().map(|o| Operator::from(*o));

            let mut sum = self.args[0];

            for (n, o) in operators.enumerate() {
                sum = o.eval(sum, self.args[n + 1]);
            }

            if sum == self.result {
                return true;
            }

            permutation = permutation.next();

            if permutation.is_zero() {
                break;
            }
        }
        false
    }
}

struct Puzzle(Arena<Equation>);

impl Deref for Puzzle {
    type Target = Arena<Equation>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Arena<Equation>> for Puzzle {
    fn as_ref(&self) -> &Arena<Equation> {
        &self.0
    }
}

impl FromStr for Puzzle {
    type Err = Vec<chumsky::error::Simple<char>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        advent_of_code::int::<u64>()
            .then_ignore(just(": "))
            .then(
                advent_of_code::int::<u64>()
                    .separated_by(just(' '))
                    .at_least(1),
            )
            .map(|(result, args)| Equation { result, args })
            .separated_by(text::newline())
            .at_least(1)
            .map(|e| Puzzle(e.into()))
            .parse(s)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let puzzle = Puzzle::from_str(input).unwrap();

    puzzle
        .ids()
        .map(|id| id.get(&puzzle))
        .filter(|eq| eq.is_correct(2))
        .map(|eq| eq.result)
        .sum::<u64>()
        .into_some()
}

pub fn part_two(input: &str) -> Option<u64> {
    let puzzle = Puzzle::from_str(input).unwrap();

    puzzle
        .ids()
        .map(|id| id.get(&puzzle))
        .filter(|eq| eq.is_correct(3))
        .map(|eq| eq.result)
        .sum::<u64>()
        .into_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
