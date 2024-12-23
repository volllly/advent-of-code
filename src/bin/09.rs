use chumsky::prelude::*;
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
    str::FromStr,
};
use tailsome::IntoOption;

advent_of_code::solution!(9);

struct Input(Vec<u8>);

impl FromStr for Input {
    type Err = Vec<chumsky::error::Simple<char>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        filter(|c: &char| c.is_ascii_digit())
            .map(|c| u8::from_str(&format!("{}", c)).unwrap())
            .repeated()
            .map(Input)
            .parse(s)
    }
}

enum Block {
    Empty,
    File(usize),
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::File(id) => write!(f, "{}", format!("{}", id).chars().last().unwrap()),
        }
    }
}

struct Filesystem(Vec<Block>);

impl Deref for Filesystem {
    type Target = Vec<Block>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Filesystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Debug for Filesystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in &self.0 {
            f.write_fmt(format_args!("{:?}", b))?;
        }

        Ok(())
    }
}

impl From<Input> for Filesystem {
    fn from(value: Input) -> Self {
        let mut filesystem = vec![];
        for (id, (block, empty)) in value.0.chunks(2).map(|c| (c[0], c.get(1))).enumerate() {
            for _ in 0..block {
                filesystem.push(Block::File(id));
            }
            if let Some(empty) = empty {
                for _ in 0..*empty {
                    filesystem.push(Block::Empty);
                }
            }
        }

        Self(filesystem)
    }
}

impl Filesystem {
    fn compress_v1(&mut self) {
        let mut empty: Option<usize> = None;
        let max = self.len();
        for pointer in (0..self.len()).rev() {
            let Block::File(id) = self[pointer] else {
                continue;
            };

            if empty.is_none() || matches!(self.get(empty.unwrap() + 1), Some(Block::File(_))) {
                empty = self.0.iter().position(|b| matches!(b, Block::Empty));
            } else if empty.unwrap() + 1 < max {
                empty = Some(empty.unwrap() + 1)
            }
            let Some(empty) = empty else {
                break;
            };
            if empty > pointer {
                break;
            }

            self[empty] = Block::File(id);
            self[pointer] = Block::Empty;
        }
    }

    fn compress_v2(&mut self) {
        let mut last_id: Option<usize> = None;
        for file_end in (0..self.len()).rev() {
            let Block::File(id) = self[file_end] else {
                continue;
            };
            if let Some(last_id) = last_id {
                if last_id == id {
                    continue;
                }
            }
            last_id = Some(id);

            let file_start = (0..file_end)
                .rev()
                .find(|i| match self[*i] {
                    Block::Empty => true,
                    Block::File(start_id) => start_id != id,
                })
                .map(|i| i + 1)
                .unwrap_or(1);
            let file_size = file_end - file_start + 1;

            let mut space_start = Some(0usize);
            while let Some(start) = space_start {
                if let Some(size) = self.space_size(start) {
                    if size >= file_size {
                        break;
                    } else {
                        space_start = Some(start + size);
                    }
                } else {
                    space_start = Some(start + 1);
                }
                if let Some(start) = space_start {
                    if start >= self.len() {
                        space_start = None;
                    }
                }
            }

            let Some(space_start) = space_start else {
                continue;
            };

            if space_start > file_end {
                continue;
            }

            for i in 0..file_size {
                self[space_start + i] = Block::File(id);
                self[file_start + i] = Block::Empty;
            }
        }
    }

    fn space_size(&self, start: usize) -> Option<usize> {
        let mut size = None;

        while let Some(Block::Empty) = self.get(start + size.unwrap_or_default()) {
            size = Some(size.unwrap_or_default() + 1);
        }

        size
    }

    fn checksum(&self) -> u64 {
        let mut checksum = 0u64;
        for (pointer, block) in self.iter().enumerate() {
            if let Block::File(id) = block {
                checksum += *id as u64 * pointer as u64;
            }
        }

        checksum
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut filesystem = Filesystem::from(Input::from_str(input).unwrap());
    filesystem.compress_v1();
    filesystem.checksum().into_some()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut filesystem = Filesystem::from(Input::from_str(input).unwrap());
    filesystem.compress_v2();
    filesystem.checksum().into_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
