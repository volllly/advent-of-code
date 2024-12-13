use std::{
    fmt::Debug,
    ops::{Deref, Index, IndexMut},
    str::FromStr,
};

use tailsome::{IntoOption, IntoResult};
use velcro::hash_set;

use advent_of_code::{Coordinate, Direction};

advent_of_code::solution!(6);

#[derive(Clone)]
enum InitCell {
    Empty,
    Obscruction,
    Guard(Direction),
}

#[derive(Clone)]
enum Cell {
    Empty,
    Obscruction,
    Path,
}
impl From<InitCell> for Cell {
    fn from(value: InitCell) -> Self {
        match value {
            InitCell::Obscruction => Cell::Obscruction,
            _ => Cell::Empty,
        }
    }
}

impl TryFrom<char> for InitCell {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => InitCell::Empty,
            '#' => InitCell::Obscruction,
            x if matches!(x, '^' | '<' | '>' | 'v') => InitCell::Guard(Direction::try_from(x)?),
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
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Guard {
    position: Coordinate,
    direction: Direction,
}

struct Map {
    map: Vec<Vec<Cell>>,
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

struct Puzzle {
    map: Map,
    guard: Guard,
}

impl FromStr for Puzzle {
    type Err = Vec<chumsky::error::Simple<char>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use chumsky::prelude::*;

        filter_map(|span, s| InitCell::try_from(s).map_err(|msg| Simple::custom(span, msg)))
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
                            .find_map(|(x, cell)| {
                                if let InitCell::Guard(direction) = cell {
                                    Some((x, direction))
                                } else {
                                    None
                                }
                            })
                            .map(|c| (y, c))
                    })
                    .map(|(y, (x, direction))| Guard {
                        position: Coordinate {
                            x: x as i64,
                            y: y as i64,
                        },
                        direction: *direction,
                    })
                    .unwrap();

                let map = map
                    .into_iter()
                    .map(|row| row.into_iter().map(Cell::from).collect())
                    .collect();
                Puzzle {
                    map: Map { map },
                    guard,
                }
            })
            .parse(s)
    }
}

impl Index<Coordinate> for Map {
    type Output = Cell;

    fn index(&self, index: Coordinate) -> &Self::Output {
        &self.map[index.y as usize][index.x as usize]
    }
}

impl IndexMut<Coordinate> for Map {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        &mut self.map[index.y as usize][index.x as usize]
    }
}

impl Map {
    fn move_guard(&mut self, guard: Guard) -> Option<Guard> {
        let next = guard.position + guard.direction;
        if next.x < 0 || next.y < 0 {
            return None;
        }
        let Some(next_cell) = self
            .get(next.y as usize)
            .and_then(|row| row.get(next.x as usize))
        else {
            self[guard.position] = Cell::Path;
            return None;
        };

        match next_cell {
            Cell::Empty | Cell::Path => {
                self[guard.position] = Cell::Path;
                Guard {
                    position: next,
                    direction: guard.direction,
                }
            }
            Cell::Obscruction => Guard {
                position: guard.position,
                direction: guard.direction.rotate(-1),
            },
        }
        .into_some()
    }

    fn perform_moves(&mut self, mut guard: Guard) {
        loop {
            guard = if let Some(guard) = self.move_guard(guard) {
                guard
            } else {
                break;
            }
        }
    }

    fn detect_loops(&mut self, guard: Guard) -> usize {
        let mut loops = 0usize;

        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                if matches!(self.map[y][x], Cell::Obscruction) {
                    continue;
                }
                let backup = self.map[y][x].clone();
                self.map[y][x] = Cell::Obscruction;
                if self.detect_loop(guard.clone()) {
                    loops += 1;
                }
                self.map[y][x] = backup
            }
        }

        loops
    }

    fn detect_loop(&mut self, mut guard: Guard) -> bool {
        let mut positions = hash_set! {guard.clone()};
        loop {
            guard = if let Some(guard) = self.move_guard(guard.clone()) {
                if positions.contains(&guard) {
                    return true;
                }
                guard
            } else {
                return false;
            };
            positions.insert(guard.clone());
        }
    }

    fn count(&self, predicate: impl Fn(&Cell) -> bool) -> usize {
        self.iter()
            .map(|row| row.iter().filter(|cell| predicate(cell)).count())
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut puzzle = Puzzle::from_str(input).unwrap();

    puzzle.map.perform_moves(puzzle.guard);

    puzzle
        .map
        .count(|cell| matches!(cell, Cell::Path))
        .into_some()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut puzzle = Puzzle::from_str(input).unwrap();

    puzzle.map.detect_loops(puzzle.guard).into_some()
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
        assert_eq!(result, Some(6));
    }
}
