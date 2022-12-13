use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use crate::util;

static test_input: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

fn parse(input: &str) -> Vec<Vec<u8>> {
    util::read_lines(input)
        .map(|line| line.chars().map(|c| c as u8).collect())
        .collect()
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Loc(usize, usize);

impl Loc {
    fn next(&self) -> Vec<Self> {
        let mut next = vec![Loc(self.0, self.1 + 1), Loc(self.0 + 1, self.1)];
        if self.0 > 0 {
            next.push(Loc(self.0 - 1, self.1));
        }
        if self.1 > 0 {
            next.push(Loc(self.0, self.1 - 1));
        }
        next
    }
}

struct Path(HashSet<Loc>);

impl Path {
    fn new() -> Self {
        Self(HashSet::new())
    }

    fn add(&self, loc: &Loc) -> Self {
        let mut hs = self.0.clone();
        hs.insert(loc.clone());
        Self(hs)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

struct Map {
    start: Loc,
    end: Loc,
    height: HashMap<Loc, u8>,
}
impl Map {
    fn new(map: Vec<Vec<u8>>) -> Self {
        let mut start = Loc(0, 0);
        let mut end = Loc(0, 0);
        let mut height = HashMap::new();
        for (y, row) in map.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                let loc = Loc(x, y);
                let mut h = *v;
                if h == 'S' as u8 {
                    start = loc;
                    h = 'a' as u8;
                } else if h == 'E' as u8 {
                    end = loc;
                    h = 'z' as u8;
                }

                height.insert(loc, h);
            }
        }
        Self { height, start, end }
    }

    fn possible(&self, loc: &Loc) -> impl Iterator<Item = Loc> + '_ {
        // get the current height
        let h = self.height.get(loc).unwrap();
        loc.next()
            .into_iter()
            .filter(|l| self.height.get(l).filter(|lh| *lh - *h <= 1).is_some())
    }

    fn find_paths(&self, loc: &Loc, taken: &Path) -> Vec<Path> {
        self.possible(&loc)
            .filter(|l| !taken.0.contains(l))
            .flat_map(|l| {
                let t = taken.add(&l);
                self.find_paths(&l, &t)
            })
            .collect()
    }

    fn find_paths_from_start(&self) -> Vec<Path> {
        self.find_paths(&self.start, &Path::new())
    }
}

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let data = parse(test_input);
    let map = Map::new(data);
    let paths = map.find_paths_from_start();

    for p in paths {
        println!("len: {}", p.len())
    }
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
