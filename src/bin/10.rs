use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    io::stdin,
    iter::repeat,
    ops::{Add, AddAssign, Index},
    str::FromStr,
};

use chumsky::{prelude::*, text::newline};
use itertools::Itertools;
use strum::{EnumCount, EnumIs, FromRepr};
use tailsome::{IntoOption, IntoResult};

advent_of_code::solution!(10);

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIs)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn cw(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn ccw(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn rev(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug)]
struct Pipe([Direction; 2]);

impl TryFrom<char> for Pipe {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Pipe([Direction::North, Direction::East]),
            'F' => Pipe([Direction::South, Direction::East]),
            'J' => Pipe([Direction::North, Direction::West]),
            '7' => Pipe([Direction::South, Direction::West]),
            '|' => Pipe([Direction::North, Direction::South]),
            '-' => Pipe([Direction::East, Direction::West]),
            _ => ().into_err()?,
        }
        .into_ok()
    }
}

impl Pipe {
    fn is_north(&self) -> bool {
        self.0.iter().any(|d| d.is_north())
    }
    fn is_east(&self) -> bool {
        self.0.iter().any(|d| d.is_east())
    }
    fn is_south(&self) -> bool {
        self.0.iter().any(|d| d.is_south())
    }
    fn is_west(&self) -> bool {
        self.0.iter().any(|d| d.is_west())
    }

    fn directions(&self) -> &[Direction; 2] {
        &self.0
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_north() && self.is_east() {
            f.write_str("└")
        } else if self.is_east() && self.is_south() {
            f.write_str("┌")
        } else if self.is_south() && self.is_west() {
            f.write_str("┐")
        } else if self.is_west() && self.is_north() {
            f.write_str("┘")
        } else if self.is_north() && self.is_south() {
            f.write_str("│")
        } else if self.is_west() && self.is_east() {
            f.write_str("─")
        } else {
            unreachable!();
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIs)]
enum Area {
    Inner,
    Outer,
}

#[derive(Debug, EnumIs)]
enum RawCell {
    Empty(Option<Area>),
    Start,
    Pipe(Pipe),
}

#[derive(Debug, EnumIs)]
enum Cell {
    Empty(Option<Area>),
    Start,
    Pipe(Pipe),
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty(None) => f.write_str("."),
            Cell::Empty(Some(Area::Inner)) => f.write_str("█"),
            Cell::Empty(Some(Area::Outer)) => f.write_str("░"),
            Cell::Start => f.write_str("S"),
            Cell::Pipe(pipe) => f.write_str(&pipe.to_string()),
        }
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        if value == 'S' {
            Cell::Start
        } else {
            Pipe::try_from(value)
                .map(Cell::Pipe)
                .unwrap_or(Cell::Empty(None))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coordinates {
    pub x: isize,
    pub y: isize,
}

impl Add<Direction> for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::North => Coordinates {
                y: self.y - 1,
                x: self.x,
            },
            Direction::East => Coordinates {
                y: self.y,
                x: self.x + 1,
            },
            Direction::South => Coordinates {
                y: self.y + 1,
                x: self.x,
            },
            Direction::West => Coordinates {
                y: self.y,
                x: self.x - 1,
            },
        }
    }
}

impl AddAssign<Direction> for Coordinates {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

struct Cells(Vec<Vec<Cell>>);

impl Debug for Cells {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\n")?;
        for row in self.rows() {
            for cell in row.iter() {
                f.write_str(&cell.to_string())?
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}

impl From<Vec<Vec<Cell>>> for Cells {
    fn from(value: Vec<Vec<Cell>>) -> Self {
        Self(value)
    }
}

impl Index<Coordinates> for Cells {
    type Output = Cell;

    fn index(&self, index: Coordinates) -> &Self::Output {
        if index.x < 0 || index.y < 0 {
            return &Cell::Empty(Some(Area::Outer));
        }

        self.0
            .get(index.y as usize)
            .and_then(|row| row.get(index.x as usize))
            .unwrap_or(&Cell::Empty(Some(Area::Outer)))
    }
}

impl Cells {
    fn get_mut(&mut self, index: Coordinates) -> Option<&mut Cell> {
        if index.x < 0 || index.y < 0 {
            return None;
        }

        self.0
            .get_mut(index.y as usize)
            .and_then(|row| row.get_mut(index.x as usize))
    }

    fn rows(&self) -> impl Iterator<Item = &Vec<Cell>> {
        self.0.iter()
    }

    fn rows_mut(&mut self) -> impl Iterator<Item = &mut Vec<Cell>> {
        self.0.iter_mut()
    }

    fn onto(&self, from: Coordinates, through: Coordinates) -> Option<Coordinates> {
        let Cell::Pipe(cell) = &self[through] else {
            return None;
        };
        cell.directions()
            .map(|d| through + d)
            .iter()
            .find(|c| **c != from)
            .copied()
    }

    fn entry_direction(&self, from: Coordinates, through: Coordinates) -> Option<Direction> {
        let Cell::Pipe(cell) = &self[through] else {
            return None;
        };

        cell.directions()
            .iter()
            .find(|d| through + **d == from)
            .copied()
            .unwrap()
            .into_some()
    }

    fn exit_direction(&self, from: Coordinates, through: Coordinates) -> Option<Direction> {
        let Cell::Pipe(cell) = &self[through] else {
            return None;
        };

        cell.directions()
            .iter()
            .find(|d| through + **d != from)
            .copied()
            .unwrap()
            .into_some()
    }

    fn curvature(&self, from: Coordinates, through: Coordinates) -> Option<i8> {
        let entry_direction = self.entry_direction(from, through)?;

        let exit_direction = self.exit_direction(from, through)?;

        if entry_direction.ccw() == exit_direction {
            return 1.into_some();
        }

        if entry_direction.cw() == exit_direction {
            return (-1).into_some();
        }

        0.into_some()
    }

    fn orthogonal(through: Coordinates, direction: Direction, curvature: i8) -> Coordinates {
        match direction {
            Direction::North => {
                if curvature.is_positive() {
                    through + Direction::East
                } else {
                    through + Direction::West
                }
            }
            Direction::East => {
                if curvature.is_positive() {
                    through + Direction::South
                } else {
                    through + Direction::North
                }
            }
            Direction::South => {
                if curvature.is_positive() {
                    through + Direction::West
                } else {
                    through + Direction::East
                }
            }
            Direction::West => {
                if curvature.is_positive() {
                    through + Direction::North
                } else {
                    through + Direction::South
                }
            }
        }
    }

    fn get_orthogonal(
        &self,
        from: Coordinates,
        through: Coordinates,
        curvature: i8,
    ) -> HashSet<Coordinates> {
        let mut orthogonal = HashSet::<Coordinates>::new();

        let Some(exit_direction) = self.exit_direction(from, through) else {
            return orthogonal;
        };

        if self
            .curvature(from, through)
            .map(|c| c == 0)
            .unwrap_or_default()
        {
            orthogonal.insert(Self::orthogonal(through, exit_direction, curvature));
        } else {
            #[allow(clippy::collapsible_else_if)]
            if self
                .curvature(from, through)
                .map(|c| c.signum() == curvature.signum())
                .unwrap_or_default()
            {
                orthogonal
                    .insert(Self::orthogonal(through, exit_direction, curvature) + exit_direction);
            } else {
                orthogonal.insert(Self::orthogonal(through, exit_direction, curvature));

                let Some(entry_direction) = self.entry_direction(from, through) else {
                    return HashSet::new();
                };
                let entry_orthogonal = Self::orthogonal(through, entry_direction.rev(), curvature);

                orthogonal.insert(entry_orthogonal);

                orthogonal.insert(entry_orthogonal + entry_direction.rev());
            }
        }

        orthogonal
    }

    fn flood_fill(&mut self, with: Area, start: Coordinates) {
        let mut flood = HashSet::<Coordinates>::new();
        flood.insert(start);
        loop {
            let old_flood = flood.clone();
            flood.clear();
            for wave in old_flood {
                if matches!(self[wave], Cell::Empty(None)) {
                    if let Some(cell) = self.get_mut(wave) {
                        *cell = Cell::Empty(Some(with));
                    }
                }

                if matches!(self[wave + Direction::North], Cell::Empty(None)) {
                    flood.insert(wave + Direction::North);
                }

                if matches!(self[wave + Direction::East], Cell::Empty(None)) {
                    flood.insert(wave + Direction::East);
                }

                if matches!(self[wave + Direction::West], Cell::Empty(None)) {
                    flood.insert(wave + Direction::West);
                }

                if matches!(self[wave + Direction::South], Cell::Empty(None)) {
                    flood.insert(wave + Direction::South);
                }
            }
            if flood.is_empty() {
                break;
            }
        }
    }
}

#[derive(Debug)]
struct Network {
    cells: Cells,
    start: (Coordinates, Pipe),
}

impl From<Cells> for Network {
    fn from(mut cells: Cells) -> Self {
        let start = cells
            .rows()
            .enumerate()
            .find_map(|(y, r)| {
                if let Some(cell) = r.iter().enumerate().find_map(|(x, c)| {
                    if c.is_start() {
                        (x, c).into_some()
                    } else {
                        None
                    }
                }) {
                    (y, cell).into_some()
                } else {
                    None
                }
            })
            .map(|(y, (x, _))| Coordinates {
                x: x as isize,
                y: y as isize,
            })
            .unwrap();

        let mut directions = Vec::<Direction>::new();
        if let Cell::Pipe(pipe) = &cells[start + Direction::North] {
            if pipe.is_south() {
                directions.push(Direction::North)
            }
        }
        if let Cell::Pipe(pipe) = &cells[start + Direction::East] {
            if pipe.is_west() {
                directions.push(Direction::East)
            }
        }
        if let Cell::Pipe(pipe) = &cells[start + Direction::South] {
            if pipe.is_north() {
                directions.push(Direction::South)
            }
        }
        if let Cell::Pipe(pipe) = &cells[start + Direction::West] {
            if pipe.is_east() {
                directions.push(Direction::West)
            }
        }

        *cells.get_mut(start).unwrap() = Cell::Pipe(Pipe(directions.clone().try_into().unwrap()));
        Network {
            start: (start, Pipe(directions.try_into().unwrap())),
            cells,
        }
    }
}

impl FromStr for Network {
    type Err = Vec<Simple<char>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row = newline().not().map(|c: char| Cell::from(c)).repeated();

        row.separated_by(newline())
            .map(Cells::from)
            .map(Network::from)
            .parse(s)
    }
}

impl Network {
    fn get_loop(&self) -> Vec<Coordinates> {
        let mut coordinates = vec![self.start.0];

        let mut last = self.start.0;
        let mut current = last + self.start.1.directions()[0];

        loop {
            coordinates.push(current);
            (last, current) = (current, self.cells.onto(last, current).unwrap());

            if current == self.start.0 {
                return coordinates;
            }
        }
    }

    fn discard_junk(&mut self) -> &mut Self {
        let start_loop = self.get_loop();

        for (y, row) in self.cells.rows_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                if !start_loop.contains(&Coordinates {
                    x: x as isize,
                    y: y as isize,
                }) {
                    *cell = Cell::Empty(None)
                }
            }
        }

        self
    }

    fn fill_areas(&mut self) -> &mut Self {
        let start_loop = self.get_loop();

        let mut curvature = 0;

        for (last, current) in start_loop.iter().tuple_windows() {
            curvature += self.cells.curvature(*last, *current).unwrap();
        }

        for (last, curent) in start_loop.iter().tuple_windows() {
            for inner in self.cells.get_orthogonal(*last, *curent, curvature) {
                if let Cell::Empty(None) = self.cells[inner] {
                    self.cells.flood_fill(Area::Inner, inner);
                }
            }
            for outer in self.cells.get_orthogonal(*last, *curent, -curvature) {
                if let Cell::Empty(None) = self.cells[outer] {
                    self.cells.flood_fill(Area::Outer, outer);
                }
            }
        }

        self
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    (Network::from_str(input).unwrap().get_loop().len() / 2).into_some()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut network = Network::from_str(input).unwrap();

    network.discard_junk().fill_areas();

    network
        .cells
        .rows()
        .flat_map(|row| row.iter())
        .filter(|c| matches!(c, Cell::Empty(Some(Area::Inner))))
        .count()
        .into_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, 8.into_some());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, 10.into_some());
    }
}
