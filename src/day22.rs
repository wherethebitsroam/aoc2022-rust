use std::{collections::HashMap, error::Error, fmt::Display};

static TEST_INPUT: &str = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Floor,
    Wall,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Op {
    Move(usize),
    Right,
    Left,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn facing(&self) -> i64 {
        match self {
            Self::Right => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Up => 3,
        }
    }

    fn opp(&self) -> Dir {
        match self {
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Up => Self::Down,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Loc(i64, i64);

impl Loc {
    fn next(&self, dir: Dir) -> Self {
        match dir {
            Dir::Left => Loc(self.0 - 1, self.1),
            Dir::Right => Loc(self.0 + 1, self.1),
            Dir::Up => Loc(self.0, self.1 - 1),
            Dir::Down => Loc(self.0, self.1 + 1),
        }
    }
}

#[derive(Debug)]
struct Map {
    tiles: HashMap<Loc, Tile>,
    ops: Vec<Op>,
    start: Loc,
    tops: HashMap<i64, Loc>,
    bottoms: HashMap<i64, Loc>,
    rights: HashMap<i64, Loc>,
    lefts: HashMap<i64, Loc>,
}

impl Map {
    fn parse(s: &str) -> Self {
        let mut tiles = HashMap::new();
        let mut iter = s.trim_end().split('\n');
        let mut start = None;

        let mut tops = HashMap::new();
        let mut bottoms = HashMap::new();
        let mut rights = HashMap::new();
        let mut lefts = HashMap::new();

        for (y, line) in (&mut iter).enumerate() {
            let y = y as i64;
            if line == "" {
                break;
            }
            for (x, c) in line.chars().enumerate() {
                let x = x as i64;
                let loc = Loc(x, y);

                match c {
                    ' ' => {}
                    '.' => {
                        tiles.insert(loc, Tile::Floor);
                        if start.is_none() {
                            start = Some(loc);
                        }
                    }
                    '#' => {
                        tiles.insert(loc, Tile::Wall);
                    }
                    _ => panic!("what is this? {}", c),
                }

                // work out limits
                if matches!(c, '.' | '#') {
                    lefts.entry(y).or_insert(loc);
                    tops.entry(x).or_insert(loc);
                    rights.insert(y, loc);
                    bottoms.insert(x, loc);
                }
            }
        }

        let ops = Self::parse_ops(iter.next().unwrap());

        Self {
            tiles,
            ops,
            start: start.unwrap(),
            tops,
            bottoms,
            rights,
            lefts,
        }
    }

    fn parse_ops(s: &str) -> Vec<Op> {
        let mut ops = Vec::new();
        let mut num: usize = 0;

        for c in s.chars() {
            match c {
                '0'..='9' => {
                    let x = c as usize - '0' as usize;
                    num = num * 10 + x;
                }
                'R' => {
                    if num > 0 {
                        ops.push(Op::Move(num));
                        num = 0;
                    }
                    ops.push(Op::Right);
                }
                'L' => {
                    if num > 0 {
                        ops.push(Op::Move(num));
                        num = 0;
                    }
                    ops.push(Op::Left);
                }
                _ => panic!("what is this? {}", c),
            }
        }

        // check for the final number
        if num > 0 {
            ops.push(Op::Move(num));
        }

        ops
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let maxx = self.tiles.keys().map(|l| l.0).max().unwrap();
        let maxy = self.tiles.keys().map(|l| l.1).max().unwrap();

        for y in 0..=maxy {
            for x in 0..=maxx {
                match self.tiles.get(&Loc(x, y)) {
                    None => {
                        write!(f, " ")?;
                    }
                    Some(Tile::Floor) => {
                        write!(f, ".")?;
                    }
                    Some(Tile::Wall) => {
                        write!(f, "#")?;
                    }
                }
            }

            write!(f, " {:?} -> {:?}\n", self.lefts[&y], self.rights[&y])?;
        }

        write!(f, "\n")?;

        for op in self.ops.iter() {
            match op {
                Op::Move(x) => {
                    write!(f, "{}", x)?;
                }
                Op::Left => {
                    write!(f, "L")?;
                }
                Op::Right => {
                    write!(f, "R")?;
                }
            }
        }

        Ok(())
    }
}

struct MapWalker<'a> {
    map: &'a Map,
    dir: Dir,
    current: Loc,
}

impl<'a> MapWalker<'a> {
    fn new(map: &'a Map) -> Self {
        Self {
            map,
            current: map.start,
            dir: Dir::Right,
        }
    }
    fn wrap(&self) -> Loc {
        match self.dir {
            Dir::Left => self.map.rights[&self.current.1],
            Dir::Right => self.map.lefts[&self.current.1],
            Dir::Up => self.map.bottoms[&self.current.0],
            Dir::Down => self.map.tops[&self.current.0],
        }
    }

    fn mov(&mut self) {
        let next = self.current.next(self.dir);
        let next = match self.map.tiles.get(&next) {
            Some(Tile::Floor) => next,
            Some(Tile::Wall) => self.current,
            None => self.wrap(),
        };
        self.current = next;
    }

    fn op(&mut self, op: Op) {
        match op {
            Op::Left => {
                self.dir = match self.dir {
                    Dir::Left => Dir::Down,
                    Dir::Right => Dir::Up,
                    Dir::Up => Dir::Left,
                    Dir::Down => Dir::Right,
                }
            }
            Op::Right => {
                self.dir = match self.dir {
                    Dir::Left => Dir::Up,
                    Dir::Right => Dir::Down,
                    Dir::Up => Dir::Right,
                    Dir::Down => Dir::Left,
                }
            }
            Op::Move(x) => {
                for _ in 0..x {
                    self.mov();
                }
            }
        }
    }

    fn run(&mut self) {
        for op in self.map.ops.iter() {
            self.op(*op);
        }
    }

    fn password(&self) -> i64 {
        1000 * (self.current.1 + 1) + 4 * (self.current.0 + 1) + self.dir.facing()
    }
}

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let map = Map::parse(input);
    let mut mw = MapWalker::new(&map);
    mw.run();

    println!("password: {:?}", mw.password());

    Ok(())
}

struct Edge {
    x: i64,
    y: i64,
    dir: Dir,
}

impl Edge {
    fn new(l: Loc, dir: Dir) -> Self {
        Self {
            x: l.0,
            y: l.1,
            dir,
        }
    }

    fn map(
        e1: Edge,
        e2: Edge,
        edge_size: i64,
        rev: bool,
        wraps: &mut HashMap<(Loc, Dir), (Loc, Dir)>,
    ) {
        let l1 = e1.locs(edge_size);
        let mut l2 = e2.locs(edge_size);

        if rev {
            l2 = l2.into_iter().rev().collect();
        }

        for (l1, l2) in l1.into_iter().zip(l2.into_iter()) {
            wraps.insert((l1, e1.dir), (l2, e2.dir.opp()));
            wraps.insert((l2, e2.dir), (l1, e1.dir.opp()));
        }
    }

    fn locs(&self, edge_size: i64) -> Vec<Loc> {
        match self.dir {
            Dir::Up => {
                let y = self.y * edge_size;
                (self.x * edge_size..(self.x + 1) * edge_size)
                    .map(|x| Loc(x, y))
                    .collect()
            }
            Dir::Down => {
                let y = (self.y + 1) * edge_size - 1;
                (self.x * edge_size..(self.x + 1) * edge_size)
                    .map(|x| Loc(x, y))
                    .collect()
            }
            Dir::Left => {
                let x = self.x * edge_size;
                (self.y * edge_size..(self.y + 1) * edge_size)
                    .map(|y| Loc(x, y))
                    .collect()
            }
            Dir::Right => {
                let x = (self.x + 1) * edge_size - 1;
                (self.y * edge_size..(self.y + 1) * edge_size)
                    .map(|y| Loc(x, y))
                    .collect()
            }
        }
    }
}

struct CubeWalker<'a> {
    map: &'a Map,
    dir: Dir,
    current: Loc,
    wraps: HashMap<(Loc, Dir), (Loc, Dir)>,
}

impl<'a> CubeWalker<'a> {
    fn new(map: &'a Map) -> Self {
        let size = map.tiles.keys().map(|l| l.0).max().unwrap();

        let real = size > 20;
        let edge_size = if real { (size + 1) / 3 } else { (size + 1) / 4 };
        let mut wraps = HashMap::new();

        println!("size: {}, edge_size: {}", size, edge_size);

        let edges = if !real {
            // the numbered sides
            let s1 = Loc(2, 0);
            let s2 = Loc(0, 1);
            let s3 = Loc(1, 1);
            let s4 = Loc(2, 1);
            let s5 = Loc(2, 2);
            let s6 = Loc(3, 2);

            vec![
                (Edge::new(s1, Dir::Left), Edge::new(s3, Dir::Up), false),
                (Edge::new(s1, Dir::Up), Edge::new(s2, Dir::Up), true),
                (Edge::new(s1, Dir::Right), Edge::new(s6, Dir::Right), true),
                (Edge::new(s2, Dir::Left), Edge::new(s6, Dir::Down), true),
                (Edge::new(s2, Dir::Down), Edge::new(s5, Dir::Down), true),
                (Edge::new(s3, Dir::Down), Edge::new(s5, Dir::Left), true),
                (Edge::new(s6, Dir::Up), Edge::new(s4, Dir::Right), true),
            ]
        } else {
            // _12
            // _3_
            // 45_
            // 6__

            // the numbered sides
            let s1 = Loc(1, 0);
            let s2 = Loc(2, 0);
            let s3 = Loc(1, 1);
            let s4 = Loc(0, 2);
            let s5 = Loc(1, 2);
            let s6 = Loc(0, 3);

            vec![
                (Edge::new(s1, Dir::Up), Edge::new(s6, Dir::Left), false),
                (Edge::new(s1, Dir::Left), Edge::new(s4, Dir::Left), true),
                (Edge::new(s2, Dir::Up), Edge::new(s6, Dir::Down), false),
                (Edge::new(s2, Dir::Right), Edge::new(s5, Dir::Right), true),
                (Edge::new(s2, Dir::Down), Edge::new(s3, Dir::Right), false),
                (Edge::new(s3, Dir::Left), Edge::new(s4, Dir::Up), false),
                (Edge::new(s5, Dir::Down), Edge::new(s6, Dir::Right), false),
            ]
        };

        for (e1, e2, rev) in edges {
            Edge::map(e1, e2, edge_size, rev, &mut wraps);
        }

        Self {
            map,
            current: map.start,
            dir: Dir::Right,
            wraps,
        }
    }

    fn wrap(&mut self) -> (Loc, Dir) {
        if let Some((loc, dir)) = self.wraps.get(&(self.current, self.dir)) {
            (*loc, *dir)
        } else {
            panic!("missing wrap for {:?}, {:?}", self.current, self.dir);
        }
    }

    fn mov(&mut self) {
        let mut next = (self.current.next(self.dir), self.dir);
        if self.map.tiles.get(&next.0).is_none() {
            next = self.wrap();
        }
        if *self.map.tiles.get(&next.0).unwrap() == Tile::Wall {
            // no move to make!
            return;
        };
        self.current = next.0;
        self.dir = next.1;
    }

    fn op(&mut self, op: Op) {
        match op {
            Op::Left => {
                self.dir = match self.dir {
                    Dir::Left => Dir::Down,
                    Dir::Right => Dir::Up,
                    Dir::Up => Dir::Left,
                    Dir::Down => Dir::Right,
                }
            }
            Op::Right => {
                self.dir = match self.dir {
                    Dir::Left => Dir::Up,
                    Dir::Right => Dir::Down,
                    Dir::Up => Dir::Right,
                    Dir::Down => Dir::Left,
                }
            }
            Op::Move(x) => {
                for _ in 0..x {
                    self.mov();
                }
            }
        }
    }

    fn run(&mut self) {
        for op in self.map.ops.iter() {
            self.op(*op);
        }
    }

    fn password(&self) -> i64 {
        1000 * (self.current.1 + 1) + 4 * (self.current.0 + 1) + self.dir.facing()
    }
}

pub fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let map = Map::parse(input);
    let mut cw = CubeWalker::new(&map);

    cw.run();

    println!("password: {:?}", cw.password());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ops() {
        let v = Map::parse_ops("10R12L20");
        assert_eq!(
            v,
            vec![
                Op::Move(10),
                Op::Right,
                Op::Move(12),
                Op::Left,
                Op::Move(20)
            ]
        );
    }
}
