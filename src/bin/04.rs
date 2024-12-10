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

#[derive(Debug)]
struct Coordinate {
    y: i32,
    x: i32,
}

impl Coordinate {
    pub fn offset(&self, offset: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + offset.x,
            y: self.y + offset.y,
        }
    }
}

impl Puzzle {
    fn direction(&self, start: &Coordinate, x: i32, y: i32, find: &[u8]) -> bool {
        for (n, char) in find.iter().enumerate() {
            let y = start.y + y * n as i32;
            let x = start.x + x * n as i32;
            if x < 0 || y < 0 {
                return false;
            }
            let Some(Cell::XMAS(i)) = self.get(y as usize).and_then(|row| row.get(x as usize))
            else {
                return false;
            };
            if *i != *char {
                return false;
            }
        }
        true
    }
    fn search_xmas_on(&self, start: Coordinate) -> u32 {
        let find: [u8; 4] = [0, 1, 2, 3];
        {
            let Cell::XMAS(i) = (*self)[start.y as usize][start.x as usize] else {
                return 0;
            };
            if i != 0 {
                return find[0] as u32;
            }
        }

        self.direction(&start, 1, 0, &find) as u32
            + self.direction(&start, 1, 1, &find) as u32
            + self.direction(&start, 0, 1, &find) as u32
            + self.direction(&start, -1, 1, &find) as u32
            + self.direction(&start, -1, 0, &find) as u32
            + self.direction(&start, -1, -1, &find) as u32
            + self.direction(&start, 0, -1, &find) as u32
            + self.direction(&start, 1, -1, &find) as u32
    }
    pub fn search_xmas(&self) -> u32 {
        let mut count = 0;
        for (y, row) in self.grid.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                count += self.search_xmas_on(Coordinate {
                    y: y as i32,
                    x: x as i32,
                })
            }
        }
        count
    }
    fn search_masx_on(&self, start: Coordinate) -> bool {
        if start.x < 1
            || start.y < 1
            || start.y >= self.len() as i32 - 1
            || start.x >= self.first().unwrap().len() as i32 - 1
        {
            return false;
        }
        let find: [u8; 3] = [1, 2, 3];
        {
            let Cell::XMAS(i) = (*self)[start.y as usize][start.x as usize] else {
                return false;
            };
            if i != find[1] {
                return false;
            }
        }

        (self.direction(&start.offset(Coordinate { x: -1, y: -1 }), 1, 1, &find)
            || self.direction(&start.offset(Coordinate { x: 1, y: 1 }), -1, -1, &find))
            && (self.direction(&start.offset(Coordinate { x: 1, y: -1 }), -1, 1, &find)
                || self.direction(&start.offset(Coordinate { x: -1, y: 1 }), 1, -1, &find))
    }
    pub fn search_masx(&self) -> u32 {
        let mut count = 0;
        for (y, row) in self.grid.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                count += self.search_masx_on(Coordinate {
                    y: y as i32,
                    x: x as i32,
                }) as u32
            }
        }
        count
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let puzzle = Puzzle::from_str(input).unwrap();

    puzzle.search_xmas().into_some()
}

pub fn part_two(input: &str) -> Option<u32> {
    let puzzle = Puzzle::from_str(input).unwrap();

    puzzle.search_masx().into_some()
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
        assert_eq!(result, Some(9));
    }
}
