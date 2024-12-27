use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Write},
    str::FromStr,
};

use advent_of_code::{Coordinate, Map};
use chumsky::prelude::*;
use itertools::Itertools;
use tailsome::IntoOption;
use velcro::hash_set;

advent_of_code::solution!(8);

struct Puzzle {
    dimensions: Coordinate,
    frequencies: HashMap<char, HashSet<Coordinate>>,
}

#[allow(dead_code)]
struct Solved<'a>(&'a Puzzle, &'a HashSet<Coordinate>);

impl Debug for Solved<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;
        for y in 0..self.0.dimensions.y {
            for x in 0..self.0.dimensions.x {
                if let Some((char, _)) = self
                    .0
                    .frequencies
                    .iter()
                    .find(|f| f.1.contains(&Coordinate { x, y }))
                {
                    f.write_char(*char)?;
                } else if self.1.contains(&Coordinate { x, y }) {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl FromStr for Puzzle {
    type Err = Vec<chumsky::error::Simple<char>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = filter(|c: &char| *c == '.' || c.is_ascii_alphanumeric())
            .repeated()
            .at_least(1)
            .separated_by(text::newline())
            .at_least(1)
            .map(Map::<char>::from)
            .parse(s)?;

        let mut frequencies = HashMap::<char, HashSet<Coordinate>>::new();
        for (coordinate, cell) in map.cells() {
            match cell {
                '.' => {}
                char => {
                    let entry = frequencies.entry(*char);
                    entry
                        .and_modify(|e| {
                            e.insert(coordinate);
                        })
                        .or_insert_with(|| hash_set![coordinate]);
                }
            }
        }

        Ok(Puzzle {
            dimensions: map.dimensions(),
            frequencies,
        })
    }
}

impl Puzzle {
    fn antinodes(&self, harmonics: bool) -> HashSet<Coordinate> {
        let mut antinodes = HashSet::<Coordinate>::new();

        for frequency in self.frequencies.values() {
            for (a, b) in frequency
                .iter()
                .tuple_combinations()
                .flat_map(|(a, b)| [(a, b), (b, a)])
            {
                let mut i = 1;
                if harmonics {
                    antinodes.insert(*a);
                    antinodes.insert(*b);
                }
                loop {
                    let n = Coordinate {
                        x: a.x - i * (b.x - a.x),
                        y: a.y - i * (b.y - a.y),
                    };

                    if n.x >= 0 && n.y >= 0 && n.x < self.dimensions.x && n.y < self.dimensions.y {
                        antinodes.insert(n);
                    } else {
                        break;
                    }
                    if harmonics {
                        i += 1;
                    } else {
                        break;
                    }
                }
            }
        }

        antinodes
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let puzzle = Puzzle::from_str(input).unwrap();

    puzzle.antinodes(false).len().into_some()
}

pub fn part_two(input: &str) -> Option<usize> {
    let puzzle = Puzzle::from_str(input).unwrap();

    puzzle.antinodes(true).len().into_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
