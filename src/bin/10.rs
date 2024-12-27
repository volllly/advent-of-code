use chumsky::prelude::*;
use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::{Debug, Display},
    str::FromStr,
};
use tailsome::IntoOption;
use text::newline;
use velcro::hash_set;

use advent_of_code::{Coordinate, Direction, Map};

advent_of_code::solution!(10);

#[derive(Debug)]
struct Puzzle {
    map: Map<Cell>,
}

impl FromStr for Puzzle {
    type Err = Vec<chumsky::error::Simple<char>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        advent_of_code::digit()
            .map(Cell::new)
            .repeated()
            .at_least(1)
            .separated_by(newline())
            .at_least(1)
            .map(Map::from)
            .map(Puzzle::from)
            .parse(s)
    }
}

#[derive(Debug)]
struct Cell {
    height: u32,
    up: HashSet<Coordinate>,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.height))
    }
}

impl Cell {
    fn new(height: u32) -> Self {
        Self {
            height,
            up: Default::default(),
        }
    }
}

impl Puzzle {
    fn from(mut map: Map<Cell>) -> Self {
        for position in map.coordinates() {
            macro_rules! update_cell {
                ($map:expr) => {
                    update_cell!(map, Direction::North);
                    update_cell!(map, Direction::East);
                    update_cell!(map, Direction::South);
                    update_cell!(map, Direction::West);
                };
                ($map:expr, $direcion:expr) => {
                    let other = map.cell(position + $direcion).map(|c| c.height);
                    let cell = map.cell_mut(position).unwrap();
                    if let Some(other) = other {
                        if other == cell.height + 1 {
                            cell.up.insert(position + $direcion);
                        }
                    }
                };
            }

            update_cell!(map);
        }
        Puzzle { map }
    }

    fn trailheads(&self) -> HashSet<Coordinate> {
        self.map
            .cells()
            .filter(|(_, c)| c.height == 0)
            .map(|(p, _)| p)
            .collect()
    }

    fn score(&self, head: Coordinate) -> usize {
        fn recurse(puzzle: &Puzzle, head: Coordinate) -> HashSet<Coordinate> {
            let mut tails = HashSet::new();
            if puzzle.map[head].height == 9 {
                return hash_set!(head);
            }
            for up in &puzzle.map[head].up {
                tails.extend(recurse(puzzle, *up));
            }

            tails
        }

        recurse(self, head).len()
    }

    fn rate(&self, head: Coordinate) -> u32 {
        let mut rating = 0;
        if self.map[head].height == 9 {
            return 1;
        }
        for up in &self.map[head].up {
            rating += self.rate(*up);
        }

        rating
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let puzzle = Puzzle::from_str(input).unwrap();

    puzzle
        .trailheads()
        .into_iter()
        .map(|h| puzzle.score(h))
        .sum::<usize>()
        .into_some()
}

pub fn part_two(input: &str) -> Option<u32> {
    let puzzle = Puzzle::from_str(input).unwrap();

    puzzle
        .trailheads()
        .into_iter()
        .map(|h| puzzle.rate(h))
        .sum::<u32>()
        .into_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
