use std::{
    collections::{HashMap, HashSet},
    error::Error,
    ops::Add,
};

static TEST_INPUT: &str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Dir {
    N,
    S,
    W,
    E,
    NE,
    NW,
    SE,
    SW,
}

impl Dir {
    const ALL: [Self; 8] = [
        Self::N,
        Self::S,
        Self::W,
        Self::E,
        Self::NE,
        Self::NW,
        Self::SE,
        Self::SW,
    ];
    const NORTH: [Self; 3] = [Self::N, Self::NE, Self::NW];
    const SOUTH: [Self; 3] = [Self::S, Self::SE, Self::SW];
    const EAST: [Self; 3] = [Self::E, Self::NE, Self::SE];
    const WEST: [Self; 3] = [Self::W, Self::NW, Self::SW];

    fn order(&self) -> [Self; 4] {
        match self {
            Self::N => [Self::N, Self::S, Self::W, Self::E],
            Self::S => [Self::S, Self::W, Self::E, Self::N],
            Self::W => [Self::W, Self::E, Self::N, Self::S],
            Self::E => [Self::E, Self::N, Self::S, Self::W],
            _ => panic!("bad direction"),
        }
    }

    fn next(&self) -> Dir {
        match self {
            Self::N => Self::S,
            Self::S => Self::W,
            Self::W => Self::E,
            Self::E => Self::N,
            _ => panic!("bad direction"),
        }
    }

    fn check(&self) -> &[Dir] {
        match self {
            Self::N => &Dir::NORTH,
            Self::S => &Dir::SOUTH,
            Self::W => &Dir::WEST,
            Self::E => &Dir::EAST,
            _ => panic!("bad direction"),
        }
    }
    fn loc(&self) -> Loc {
        match self {
            Self::N => Loc(0, -1),
            Self::S => Loc(0, 1),
            Self::W => Loc(-1, 0),
            Self::E => Loc(1, 0),
            Self::NE => Loc(1, -1),
            Self::NW => Loc(-1, -1),
            Self::SE => Loc(1, 1),
            Self::SW => Loc(-1, 1),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Loc(i64, i64);

impl Add for Loc {
    type Output = Loc;

    fn add(self, rhs: Loc) -> Self::Output {
        Loc(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Loc {
    fn propose(&self, current: &HashSet<Loc>, dir: Dir) -> Option<Loc> {
        let occupied: HashSet<Dir> = Dir::ALL
            .into_iter()
            .filter(|c| current.contains(&self.add(c.loc())))
            .collect();

        // println!("loc: {:?}, occupied: {:?}", self, occupied);

        if occupied.is_empty() {
            return None;
        }

        for d in dir.order() {
            // println!("dir: {:?}, check: {:?}", d, d.check());
            if !d.check().iter().any(|d| occupied.contains(d)) {
                return Some(self.add(d.loc()));
            }
        }

        None
    }
}

fn parse(s: &str) -> HashSet<Loc> {
    let mut hs = HashSet::new();
    for (y, line) in s.trim().split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                hs.insert(Loc(x as i64, y as i64));
            }
        }
    }
    hs
}

fn print(map: &HashSet<Loc>) {
    let minx = map.iter().map(|x| x.0).min().unwrap();
    let maxx = map.iter().map(|x| x.0).max().unwrap();
    let miny = map.iter().map(|x| x.1).min().unwrap();
    let maxy = map.iter().map(|x| x.1).max().unwrap();

    for y in (miny - 3)..(maxy + 3) {
        for x in (minx - 3)..(maxx + 3) {
            print!("{}", if map.contains(&Loc(x, y)) { "#" } else { "." });
        }
        println!("  {}", y);
    }
}

fn rectangle(map: &HashSet<Loc>) -> i64 {
    let minx = map.iter().map(|x| x.0).min().unwrap();
    let maxx = map.iter().map(|x| x.0).max().unwrap();
    let miny = map.iter().map(|x| x.1).min().unwrap();
    let maxy = map.iter().map(|x| x.1).max().unwrap();

    (1 + maxx - minx) * (1 + maxy - miny)
}

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut occupied = parse(input);
    let mut dir = Dir::N;

    for _ in 0..10 {
        // dest => [src]
        let mut proposed: HashMap<Loc, Vec<Loc>> = HashMap::new();
        for loc in occupied.iter() {
            if let Some(next) = loc.propose(&occupied, dir) {
                proposed
                    .entry(next)
                    .and_modify(|v| v.push(*loc))
                    .or_insert(vec![*loc]);
            }
        }

        // updated
        for (dst, srcs) in proposed {
            if srcs.len() == 1 {
                occupied.remove(&srcs[0]);
                occupied.insert(dst);
            }
        }

        dir = dir.next();
    }

    let rectangle = rectangle(&occupied);
    println!("{}", rectangle - occupied.len() as i64);

    Ok(())
}

pub fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let mut occupied = parse(input);
    let mut dir = Dir::N;

    for i in 0..1000 {
        // dest => [src]
        let mut proposed: HashMap<Loc, Vec<Loc>> = HashMap::new();
        for loc in occupied.iter() {
            if let Some(next) = loc.propose(&occupied, dir) {
                proposed
                    .entry(next)
                    .and_modify(|v| v.push(*loc))
                    .or_insert(vec![*loc]);
            }
        }

        let moves: Vec<_> = proposed
            .into_iter()
            .filter_map(|(dst, srcs)| {
                if srcs.len() == 1 {
                    Some((dst, srcs[0]))
                } else {
                    None
                }
            })
            .collect();

        if moves.is_empty() {
            println!("{}", i + 1);
            break;
        }

        // updated
        for (dst, src) in moves {
            occupied.remove(&src);
            occupied.insert(dst);
        }

        dir = dir.next();
    }

    Ok(())
}
