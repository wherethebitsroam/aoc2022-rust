use std::{collections::HashMap, error::Error};

use crate::util;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Rock,
    Sand,
    Air,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Loc(usize, usize);

struct Map {
    spaces: HashMap<Loc, Space>,
}

impl Map {
    fn parse(s: &str) -> Self {
        let mut spaces = HashMap::new();
    }
}

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, 1);
    }
}
