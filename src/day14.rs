use std::{collections::HashMap, error::Error};

use crate::util;

static TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Rock,
    Sand,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Loc(usize, usize);

impl From<&str> for Loc {
    fn from(value: &str) -> Self {
        let bits: Vec<_> = value.split(',').collect();
        Self(bits[0].parse().unwrap(), bits[1].parse().unwrap())
    }
}

#[derive(Debug)]
struct Map {
    spaces: HashMap<Loc, Space>,
    maxy: usize,
    floor: bool,
}

impl Map {
    fn parse(s: &str) -> Self {
        let mut spaces = HashMap::new();

        for line in util::read_lines(s) {
            let pairs: Vec<_> = line.split(" -> ").map(|s| Loc::from(s)).collect();
            for pair in pairs.windows(2) {
                let l0 = pair[0];
                let l1 = pair[1];

                if l0.0 == l1.0 {
                    // y value changing
                    let (miny, maxy) = if l0.1 < l1.1 {
                        (l0.1, l1.1)
                    } else {
                        (l1.1, l0.1)
                    };
                    for y in miny..=maxy {
                        spaces.insert(Loc(l0.0, y), Space::Rock);
                    }
                } else if l0.1 == l1.1 {
                    // x value changing
                    let (minx, maxx) = if l0.0 < l1.0 {
                        (l0.0, l1.0)
                    } else {
                        (l1.0, l0.0)
                    };
                    for x in minx..=maxx {
                        spaces.insert(Loc(x, l0.1), Space::Rock);
                    }
                } else {
                    panic!("bad pair: {:?}, {:?}", l0, l1);
                }
            }
        }

        let maxy = spaces.keys().map(|l| l.1).max().unwrap();

        Self {
            spaces,
            maxy,
            floor: false,
        }
    }

    fn occupied(&self, loc: Loc) -> bool {
        self.spaces.contains_key(&loc) || (self.floor && loc.1 == self.maxy + 2)
    }

    fn is_empty(&self, loc: Loc) -> bool {
        !self.occupied(loc)
    }

    fn stopper(&self, start: Loc) -> Option<Loc> {
        for y in start.1..=self.maxy + 2 {
            let l = Loc(start.0, y);
            if !self.is_empty(l) {
                return Some(l);
            }
        }
        None
    }

    fn find_rest(&self, start: Loc) -> Option<Loc> {
        if let Some(s) = self.stopper(start) {
            let left = Loc(s.0 - 1, s.1);
            let right = Loc(s.0 + 1, s.1);

            if self.is_empty(left) {
                return self.find_rest(left);
            } else if self.is_empty(right) {
                return self.find_rest(right);
            } else {
                // we come to a stop above the stopper
                return Some(Loc(s.0, s.1 - 1));
            }
        }
        None
    }

    fn sand(&mut self, loc: Loc) {
        self.spaces.insert(loc, Space::Sand);
    }
}

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut map = Map::parse(input);
    println!("{:?}", map);
    let sand_start = Loc(500, 0);

    let mut units = 0;
    loop {
        match map.find_rest(sand_start) {
            Some(l) => {
                map.sand(l);
                units += 1;
                println!("{:?}, {}", l, units);
            }
            None => break,
        }
    }
    Ok(())
}

pub fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let mut map = Map::parse(input);
    map.floor = true;
    println!("{:?}", map);
    let sand_start = Loc(500, 0);

    let mut units = 0;
    loop {
        match map.find_rest(sand_start) {
            Some(l) => {
                map.sand(l);
                units += 1;
                println!("{:?}, {}", l, units);
                if l == sand_start {
                    break;
                }
            }
            None => panic!("shouldn't happen!"),
        }
    }
    Ok(())
}
