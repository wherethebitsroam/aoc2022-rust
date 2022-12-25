use std::{collections::HashSet, error::Error, fmt::Display};

use crate::util;

static TEST_INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "^"),
            Self::Down => write!(f, "v"),
            Self::Left => write!(f, "<"),
            Self::Right => write!(f, ">"),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Loc {
    x: i64,
    y: i64,
}

impl Loc {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    fn nu(x: usize, y: usize) -> Self {
        Self {
            x: x as i64,
            y: y as i64,
        }
    }
}

impl Default for Loc {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Debug)]
struct Blizzard {
    loc: Loc,
    dir: Dir,
}

#[derive(Debug)]
struct Valley {
    start: Loc,
    end: Loc,
    blizzards: Vec<Blizzard>,
    // the area inside the wall
    // (top left, bottom right)
    min: Loc,
    max: Loc,
}

impl Valley {
    fn parse(s: &str) -> Self {
        let lines: Vec<_> = util::read_lines(s).collect();

        let min = Loc { x: 1, y: 1 };
        let mut max = Loc::default();
        let mut start = Loc::default();
        let mut end = Loc::default();
        let mut blizzards = Vec::new();
        let last_line = lines.len() - 1;

        for (y, line) in lines.into_iter().enumerate() {
            let chars: Vec<_> = line.chars().collect();
            if y == 0 {
                max = Loc::nu(chars.len() - 2, last_line - 1);
                // find the start
                let x = chars.iter().position(|x| *x == '.').unwrap();
                start = Loc::nu(x, y);
            } else if y == last_line {
                // find the end
                let x = chars.iter().position(|x| *x == '.').unwrap();
                end = Loc::nu(x, y);
            } else {
                // find blizzards
                for (x, c) in chars.into_iter().enumerate() {
                    let dir = match c {
                        '^' => Dir::Up,
                        'v' => Dir::Down,
                        '<' => Dir::Left,
                        '>' => Dir::Right,
                        _ => continue,
                    };
                    let loc = Loc::nu(x, y);
                    blizzards.push(Blizzard { loc, dir });
                }
            }
        }

        Self {
            start,
            end,
            blizzards,
            min,
            max,
        }
    }

    fn step(&mut self) {
        for b in self.blizzards.iter_mut() {
            match b.dir {
                Dir::Up => {
                    if b.loc.y == self.min.y {
                        b.loc.y = self.max.y;
                    } else {
                        b.loc.y -= 1;
                    }
                }
                Dir::Down => {
                    if b.loc.y == self.max.y {
                        b.loc.y = self.min.y;
                    } else {
                        b.loc.y += 1;
                    }
                }
                Dir::Left => {
                    if b.loc.x == self.min.x {
                        b.loc.x = self.max.x;
                    } else {
                        b.loc.x -= 1;
                    }
                }
                Dir::Right => {
                    if b.loc.x == self.max.x {
                        b.loc.x = self.min.x;
                    } else {
                        b.loc.x += 1;
                    }
                }
            }
        }
    }

    fn dump(&self) {
        for y in self.min.y..=self.max.y {
            for x in self.min.x..=self.max.x {
                let bs: Vec<_> = self
                    .blizzards
                    .iter()
                    .filter(|b| b.loc.x == x && b.loc.y == y)
                    .collect();

                match bs.len() {
                    0 => print!("."),
                    1 => print!("{}", bs[0].dir),
                    x => print!("{}", x),
                };
            }
            println!();
        }
    }

    fn contains(&self, l: &Loc) -> bool {
        *l == self.start
            || *l == self.end
            || (l.x >= self.min.x && l.x <= self.max.x && l.y >= self.min.y && l.y <= self.max.y)
    }
}

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut valley = Valley::parse(input);

    // the set of possible positions
    let mut pos = HashSet::new();
    pos.insert(valley.start);

    'label: for i in 0..1000 {
        valley.step();

        // all of the occupied spaces after the step
        let occ: HashSet<Loc> = valley.blizzards.iter().map(|b| b.loc).collect();

        let mut new = HashSet::new();

        for p in pos.iter() {
            let next = [
                Loc::new(p.x, p.y),
                Loc::new(p.x, p.y + 1),
                Loc::new(p.x, p.y - 1),
                Loc::new(p.x + 1, p.y),
                Loc::new(p.x - 1, p.y),
            ];

            // check if we can get to the end
            if next.contains(&valley.end) {
                println!("done: {}", i + 1);
                break 'label;
            }

            for n in next {
                if valley.contains(&n) && !occ.contains(&n) {
                    new.insert(n);
                }
            }
        }

        pos = new;
        // println!("{:?}", pos);
    }

    Ok(())
}

pub fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let mut valley = Valley::parse(input);

    // the set of possible positions
    let mut pos = HashSet::new();
    pos.insert(valley.start);
    let mut goal = valley.end;
    // this is reverse order because we pop from the end
    let mut goals_remaining = vec![valley.end, valley.start];

    'label: for i in 0..1000 {
        valley.step();

        // all of the occupied spaces after the step
        let occ: HashSet<Loc> = valley.blizzards.iter().map(|b| b.loc).collect();

        let mut new = HashSet::new();

        for p in pos.iter() {
            let next = [
                Loc::new(p.x, p.y),
                Loc::new(p.x, p.y + 1),
                Loc::new(p.x, p.y - 1),
                Loc::new(p.x + 1, p.y),
                Loc::new(p.x - 1, p.y),
            ];

            // check if we can get to the goal
            if next.contains(&goal) {
                if goals_remaining.is_empty() {
                    println!("done: {}", i + 1);
                    break 'label;
                }

                // we got to the goal, so that is the only starting position
                new = HashSet::new();
                new.insert(goal);

                // get the new goal
                goal = goals_remaining.pop().unwrap();

                break;
            } else {
                for n in next {
                    if valley.contains(&n) && !occ.contains(&n) {
                        new.insert(n);
                    }
                }
            }
        }

        pos = new;
        // println!("{:?}", pos);
    }
    Ok(())
}
