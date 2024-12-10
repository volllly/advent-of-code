use std::{
    fmt::Debug,
    ops::{Add, Deref, DerefMut, Index, IndexMut},
    str::FromStr,
};

use tailsome::{IntoOption, IntoResult};

use advent_of_code::{Coordinate, Direction};

advent_of_code::solution!(6);

#[derive(Clone)]
enum Cell {
    Empty,
    Obscruction,
    Path,
    Guard(Direction),
}

impl TryFrom<char> for Cell {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Cell::Empty,
            '#' => Cell::Obscruction,
            'X' => Cell::Path,
            x if matches!(x, '^' | '<' | '>' | 'v') => Cell::Guard(Direction::try_from(x)?),
            s => Err(format!("invalid symbol {s}"))?,
        }
        .into_ok()
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Obscruction => write!(f, "#"),
            Cell::Path => write!(f, "X"),
            Cell::Guard(direction) => write!(f, "{:?}", direction),
        }
    }
}

impl Cell {
    pub fn get_direction(&self) -> Option<Direction> {
        match self {
            Cell::Empty => None,
            Cell::Obscruction => None,
            Cell::Path => None,
            Cell::Guard(driection) => driection.into_some().copied(),
        }
    }
}

struct Map {
    map: Vec<Vec<Cell>>,
    guard: Coordinate,
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.iter() {
            for cell in row.iter() {
                write!(f, "{:?}", cell)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Deref for Map {
    type Target = Vec<Vec<Cell>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl FromStr for Map {
    type Err = Vec<chumsky::error::Simple<char>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use chumsky::prelude::*;

        filter_map(|span, s| Cell::try_from(s).map_err(|msg| Simple::custom(span, msg)))
            .repeated()
            .at_least(1)
            .separated_by(text::newline())
            .at_least(1)
            .map(|map| {
                let guard = map
                    .iter()
                    .enumerate()
                    .find_map(|(y, row)| {
                        row.iter()
                            .enumerate()
                            .find(|(_, cell)| matches!(cell, Cell::Guard(_)))
                            .map(|c| (y, c))
                    })
                    .map(|(y, (x, _))| Coordinate { x, y })
                    .unwrap();

                Map { map, guard }
            })
            .parse(s)
    }
}

impl Index<Coordinate> for Map {
    type Output = Cell;

    fn index(&self, index: Coordinate) -> &Self::Output {
        &self.map[index.y][index.x]
    }
}

impl IndexMut<Coordinate> for Map {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        &mut self.map[index.y][index.x]
    }
}

impl Map {
    fn move_guard(&mut self) -> bool {
        let guard = self.guard;
        let direction = self[guard].get_direction().unwrap();
        let next = guard + direction;
        let Some(next_cell) = self.get(next.y).and_then(|row| row.get(next.x)) else {
            self[guard] = Cell::Path;
            return false;
        };

        match next_cell {
            Cell::Empty | Cell::Path => {
                self[guard] = Cell::Path;
                self.guard = next;
                self[next] = Cell::Guard(direction);
            }
            Cell::Obscruction => {
                self[guard] = Cell::Guard(direction.rotate(-1));
            }
            _ => panic!("multiple guards?"),
        }

        true
    }

    fn perform_moves(&mut self) {
        while self.move_guard() {}
    }

    fn count(&self, predicate: impl Fn(&Cell) -> bool) -> usize {
        self.iter()
            .map(|row| row.iter().filter(|cell| predicate(cell)).count())
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut map = Map::from_str(input).unwrap();

    map.perform_moves();

    map.count(|cell| matches!(cell, Cell::Path)).into_some()
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
