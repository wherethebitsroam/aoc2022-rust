use std::{
    collections::{hash_map::Entry, HashMap},
    error::Error,
};

use crate::util;

static TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

fn parse(input: &str) -> Vec<Vec<u8>> {
    util::read_lines(input)
        .map(|line| line.chars().map(|c| c as u8).collect())
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
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

    fn possible(&self, loc: &Loc, dir: Direction) -> Vec<Loc> {
        // get the current height
        let h = self.height.get(loc).unwrap();
        loc.next()
            .into_iter()
            .filter(|l| match self.height.get(l) {
                None => false,
                Some(lh) => match dir {
                    Direction::Down => *h - 1 <= *lh,
                    Direction::Up => *h + 1 >= *lh,
                },
            })
            .collect()
    }

    fn find_path_from_start(&self) -> usize {
        Hike::hike(&self, &self.start, Direction::Up, |l| l == &self.end).unwrap()
    }

    fn find_path_from_end(&self) -> usize {
        Hike::hike(&self, &self.end, Direction::Down, |l| l == &self.start).unwrap()
    }
}

struct Hike<'a, F>
where
    F: Fn(&Loc) -> bool,
{
    map: &'a Map,
    // map of points that have been reached and how many steps it took
    seen: HashMap<Loc, usize>,
    goal: F,
}

impl<'a, F> Hike<'a, F>
where
    F: Fn(&Loc) -> bool,
{
    fn hike(map: &'a Map, start: &Loc, dir: Direction, goal: F) -> Option<usize> {
        let mut seen = HashMap::new();
        seen.insert(start.to_owned(), 0);
        let mut hiker = Self { seen, map, goal };
        hiker.find_path(start, 0, dir)
    }

    fn find_path(&mut self, loc: &Loc, steps: usize, dir: Direction) -> Option<usize> {
        let steps = steps + 1;
        let mut remaining = Vec::new();

        for next in self.map.possible(&loc, dir) {
            if (self.goal)(&next) {
                return Some(steps);
            }
            let cont = match self.seen.entry(next) {
                Entry::Occupied(mut e) => {
                    let x = e.get_mut();
                    if steps < *x {
                        // we found a shorter route
                        *x = steps;
                        true
                    } else {
                        // we already got to this point quicker another way
                        false
                    }
                }
                Entry::Vacant(e) => {
                    // first time we got here!
                    e.insert(steps);
                    true
                }
            };

            if cont {
                remaining.push(self.find_path(&next, steps, dir));
            }
        }

        remaining.into_iter().flatten().min()
    }
}

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let data = parse(input);
    let map = Map::new(data);
    let len = map.find_path_from_end();

    println!("len: {}", len);
    Ok(())
}

pub fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let data = parse(input);
    let map = Map::new(data);

    let len = Hike::hike(&map, &map.end, Direction::Down, |l| {
        match map.height.get(l) {
            Some(h) => *h == 'a' as u8,
            None => false,
        }
    })
    .unwrap();

    println!("len: {}", len);

    Ok(())
}
