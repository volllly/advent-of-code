use std::{fmt::Debug, ops::Add, str::FromStr};

use chumsky::prelude::*;
use tailsome::IntoResult;

pub mod template;

pub fn int<T>() -> impl Parser<char, T, Error = Simple<char>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    text::int::<char, Simple<_>>(10).try_map(|s: String, span| {
        s.parse::<T>()
            .map_err(|e| Simple::custom(span, format!("{:?}", e)))
    })
}

#[derive(Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "^"),
            Direction::East => write!(f, ">"),
            Direction::South => write!(f, "v"),
            Direction::West => write!(f, ">"),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            d => Err(format!("unknown direction {d}"))?,
        }
        .into_ok()
    }
}

impl Direction {
    pub fn rotate(self, direction: i8) -> Direction {
        match self {
            Direction::North => match direction.cmp(&0) {
                std::cmp::Ordering::Greater => Direction::West,
                std::cmp::Ordering::Less => Direction::East,
                std::cmp::Ordering::Equal => Direction::North,
            },
            Direction::East => match direction.cmp(&0) {
                std::cmp::Ordering::Greater => Direction::North,
                std::cmp::Ordering::Less => Direction::South,
                std::cmp::Ordering::Equal => Direction::East,
            },
            Direction::South => match direction.cmp(&0) {
                std::cmp::Ordering::Greater => Direction::East,
                std::cmp::Ordering::Less => Direction::West,
                std::cmp::Ordering::Equal => Direction::South,
            },
            Direction::West => match direction.cmp(&0) {
                std::cmp::Ordering::Greater => Direction::South,
                std::cmp::Ordering::Less => Direction::North,
                std::cmp::Ordering::Equal => Direction::West,
            },
        }
    }
}

#[derive(Clone, Copy)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Add<Direction> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::North => Coordinate {
                x: self.x,
                y: self.y - 1,
            },
            Direction::East => Coordinate {
                x: self.x + 1,
                y: self.y,
            },
            Direction::South => Coordinate {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Coordinate {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}
