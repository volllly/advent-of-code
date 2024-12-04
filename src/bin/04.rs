use chumsky::prelude::*;
use std::{ops::Deref, str::FromStr};
use tailsome::IntoOption;
advent_of_code::solution!(4);

#[derive(Debug)]
enum Cell {
    Ignore,
    XMAS(u8),
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            'X' => Cell::XMAS(0),
            'M' => Cell::XMAS(1),
            'A' => Cell::XMAS(2),
            'S' => Cell::XMAS(3),
            _ => Cell::Ignore,
        }
    }
}

#[derive(Debug)]
struct Puzzle {
    grid: Vec<Vec<Cell>>,
}

impl Deref for Puzzle {
    type Target = Vec<Vec<Cell>>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl FromStr for Puzzle {
    type Err = Vec<chumsky::error::Simple<char>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        filter_map(|span, c: char| match c.is_ascii_uppercase() {
            true => Ok(Cell::from(c)),
            false => Err(Simple::custom(span, format!("'{}' is not ascii", c))),
        })
        .repeated()
        .at_least(1)
        .separated_by(text::newline())
        .map(|grid| Puzzle { grid })
        .parse(s)
    }
}

struct Coordinate {
    y: usize,
    x: usize,
}

impl Puzzle {
    fn direction(&self, start: &Coordinate, x: i32, y: i32) -> u32 {
        for n in 1..4 {
            let y = start.y as i32 + y * n;
            let x = start.x as i32 + x * n;
            if x < 0 || y < 0 {
                return 0;
            }
            let Some(Cell::XMAS(i)) = self.get(y as usize).and_then(|row| row.get(x as usize))
            else {
                return 0;
            };
            if *i as i32 != n {
                return 0;
            }
        }

        1
    }
    fn search_one(&self, start: Coordinate) -> u32 {
        {
            let Cell::XMAS(i) = (*self)[start.y][start.x] else {
                return 0;
            };
            if i != 0 {
                return 0;
            }
        }

        self.direction(&start, 1, 0)
            + self.direction(&start, 1, 1)
            + self.direction(&start, 0, 1)
            + self.direction(&start, -1, 1)
            + self.direction(&start, -1, 0)
            + self.direction(&start, -1, -1)
            + self.direction(&start, 0, -1)
            + self.direction(&start, 1, -1)
    }
    pub fn search(&self) -> u32 {
        let mut count = 0;
        for (y, row) in self.grid.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                count += self.search_one(Coordinate { y, x })
            }
        }
        count
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let puzzle = Puzzle::from_str(input).unwrap();
    dbg!(&puzzle);

    puzzle.search().into_some()
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
