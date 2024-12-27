use std::{
    fmt::{Debug, Display, Write},
    ops::{Add, Deref, DerefMut, Index, IndexMut},
    str::FromStr,
};

use chumsky::prelude::*;
use tailsome::IntoResult;

pub mod template;

pub mod arena;

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

pub fn digit() -> impl Parser<char, u32, Error = Simple<char>> {
    filter(|c: &char| c.is_ascii_digit()).try_map(|c: char, span| {
        c.to_digit(10)
            .ok_or_else(|| Simple::custom(span, format!("cloud not parse digit {}", c)))
    })
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: i64,
    pub y: i64,
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

#[derive(Debug)]
pub struct Map<T>(Vec<Vec<T>>);

impl<T> Display for Map<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;
        for row in self.iter() {
            for cell in row {
                f.write_fmt(format_args!("{}", cell))?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl<T> Map<T> {
    pub fn dimensions(&self) -> Coordinate {
        Coordinate {
            y: self.0.len() as i64,
            x: self.0[0].len() as i64,
        }
    }

    pub fn coordinates(&self) -> impl Iterator<Item = Coordinate> {
        let cols = self.0[0].len();

        (0..self.0.len()).flat_map(move |y| {
            (0..cols).map(move |x| Coordinate {
                x: x as i64,
                y: y as i64,
            })
        })
    }

    pub fn cells(&self) -> impl Iterator<Item = (Coordinate, &T)> {
        self.0.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, cell)| {
                (
                    Coordinate {
                        x: x as i64,
                        y: y as i64,
                    },
                    cell,
                )
            })
        })
    }

    pub fn cells_mut(&mut self) -> impl Iterator<Item = (Coordinate, &mut T)> {
        self.0.iter_mut().enumerate().flat_map(|(y, row)| {
            row.iter_mut().enumerate().map(move |(x, cell)| {
                (
                    Coordinate {
                        x: x as i64,
                        y: y as i64,
                    },
                    cell,
                )
            })
        })
    }

    pub fn cell(&self, position: Coordinate) -> Option<&T> {
        self.0
            .get(position.y as usize)
            .and_then(|row| row.get(position.x as usize))
    }

    pub fn cell_mut(&mut self, position: Coordinate) -> Option<&mut T> {
        self.0
            .get_mut(position.y as usize)
            .and_then(|row| row.get_mut(position.x as usize))
    }
}

impl<T> From<Vec<Vec<T>>> for Map<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        Map(value)
    }
}

impl<T> Deref for Map<T> {
    type Target = Vec<Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Map<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Index<Coordinate> for Map<T> {
    type Output = T;

    fn index(&self, index: Coordinate) -> &Self::Output {
        &self.0[index.y as usize][index.x as usize]
    }
}

impl<T> IndexMut<Coordinate> for Map<T> {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        &mut self.0[index.y as usize][index.x as usize]
    }
}
