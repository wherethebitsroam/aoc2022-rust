use std::{
    collections::{HashMap, HashSet},
    error::Error,
    iter::{Cycle, Enumerate},
    str::Chars,
};

static TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Loc(usize, usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

struct Pattern<'a> {
    chars: Cycle<Enumerate<Chars<'a>>>,
}

impl<'a> Pattern<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            chars: s.chars().enumerate().cycle(),
        }
    }

    fn next(&mut self) -> (usize, Dir) {
        if let Some((i, c)) = self.chars.next() {
            let dir = match c {
                '>' => Dir::Right,
                '<' => Dir::Left,
                c => panic!("bad direction: {:?}", c),
            };
            return (i, dir);
        }
        unreachable!()
    }
}

#[derive(Debug, Clone, Copy)]
enum ShapeKind {
    Dash,
    Plus,
    L,
    Column,
    Box,
}

static SHAPE_KINDS: [ShapeKind; 5] = [
    ShapeKind::Dash,
    ShapeKind::Plus,
    ShapeKind::L,
    ShapeKind::Column,
    ShapeKind::Box,
];

struct ShapeGenerator {
    shape_kinds: Cycle<std::slice::Iter<'static, ShapeKind>>,
}

impl ShapeGenerator {
    fn new() -> Self {
        Self {
            shape_kinds: SHAPE_KINDS.iter().cycle(),
        }
    }

    fn next_shape(&mut self, height: usize) -> Shape {
        let kind = *self.shape_kinds.next().unwrap();
        match kind {
            ShapeKind::Dash => Shape::new(
                kind,
                vec![
                    Loc(2, height),
                    Loc(3, height),
                    Loc(4, height),
                    Loc(5, height),
                ],
            ),
            ShapeKind::Plus => Shape::new(
                kind,
                vec![
                    Loc(3, height),
                    Loc(2, height + 1),
                    Loc(3, height + 1),
                    Loc(4, height + 1),
                    Loc(3, height + 2),
                ],
            ),
            ShapeKind::L => Shape::new(
                kind,
                vec![
                    Loc(2, height),
                    Loc(3, height),
                    Loc(4, height),
                    Loc(4, height + 1),
                    Loc(4, height + 2),
                ],
            ),
            ShapeKind::Column => Shape::new(
                kind,
                vec![
                    Loc(2, height),
                    Loc(2, height + 1),
                    Loc(2, height + 2),
                    Loc(2, height + 3),
                ],
            ),
            ShapeKind::Box => Shape::new(
                kind,
                vec![
                    Loc(2, height),
                    Loc(3, height),
                    Loc(2, height + 1),
                    Loc(3, height + 1),
                ],
            ),
        }
    }
}

#[derive(Debug)]
struct Shape {
    kind: ShapeKind,
    locs: Vec<Loc>,
}

impl Shape {
    fn new(kind: ShapeKind, locs: Vec<Loc>) -> Self {
        Self { kind, locs }
    }

    fn can_jet(&self, dir: Dir, chamber: &Chamber) -> bool {
        for l in self.locs.iter() {
            match dir {
                Dir::Left => {
                    if l.0 == 0 || chamber.occ.contains(&Loc(l.0 - 1, l.1)) {
                        return false;
                    }
                }
                Dir::Right => {
                    if l.0 == 6 || chamber.occ.contains(&Loc(l.0 + 1, l.1)) {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn jet(&mut self, dir: Dir, chamber: &Chamber) {
        if self.can_jet(dir, chamber) {
            for l in self.locs.iter_mut() {
                l.0 = match dir {
                    Dir::Left => l.0 - 1,
                    Dir::Right => l.0 + 1,
                }
            }
        }
    }

    fn can_fall(&self, chamber: &Chamber) -> bool {
        for l in self.locs.iter() {
            if l.1 == 0 || chamber.occ.contains(&Loc(l.0, l.1 - 1)) {
                return false;
            }
        }
        true
    }

    // Returns true if the shape fell
    fn fall(&mut self, chamber: &Chamber) -> bool {
        if self.can_fall(chamber) {
            for l in self.locs.iter_mut() {
                l.1 -= 1;
            }
            true
        } else {
            false
        }
    }
}

struct Chamber {
    occ: HashSet<Loc>,
    height: usize,
    tops: [usize; 7],
}

impl Chamber {
    fn new() -> Self {
        Self {
            occ: HashSet::new(),
            height: 0,
            tops: [0; 7],
        }
    }

    fn drops(&self) -> Vec<usize> {
        self.tops.iter().map(|x| self.height - x).collect()
    }

    fn add_shape(&mut self, shape: Shape) {
        for l in shape.locs.iter() {
            let y = l.1 + 1;
            if y > self.tops[l.0] {
                self.tops[l.0] = y;
            }
            if y > self.height {
                self.height = y;
            }
        }
        for l in shape.locs {
            self.occ.insert(l);
        }
    }

    fn dump(&self) {
        for r in 0..self.height + 2 {
            let row = self.height + 1 - r;
            print!("|");
            for col in 0..=6 {
                let c = if self.occ.contains(&Loc(col, row)) {
                    "#"
                } else {
                    "."
                };
                print!("{}", c);
            }
            println!("| {}", row);
        }
        println!("+-------+")
    }
}

fn rounds(input: &str, r: usize, inc: usize) -> Chamber {
    let mut pattern = Pattern::new(input);
    let mut shape_generator = ShapeGenerator::new();
    let mut chamber = Chamber::new();

    for i in 0..r {
        let mut shape = shape_generator.next_shape(chamber.height + 3);
        loop {
            let (_, dir) = pattern.next();
            shape.jet(dir, &chamber);
            if !shape.fall(&chamber) {
                break;
            }
        }
        chamber.add_shape(shape);

        if i % inc == 0 {
            println!("done: {}", i);
        }
    }

    chamber
}

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let chamber = rounds(input, 2022, 100000);
    println!("height: {}", chamber.height);
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Values {
    shapes: usize,
    height: usize,
}

pub fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    // let input = TEST_INPUT;

    let mut pattern_idx;
    let mut pattern = Pattern::new(input);
    let mut shape_generator = ShapeGenerator::new();
    let mut chamber = Chamber::new();

    let mut seen: HashMap<(usize, Vec<usize>), Values> = HashMap::new();

    let first: Values;
    let second: Values;

    let mut shapes = 0;
    loop {
        let mut shape = shape_generator.next_shape(chamber.height + 3);
        loop {
            let (idx, dir) = pattern.next();
            pattern_idx = idx;

            shape.jet(dir, &chamber);
            if !shape.fall(&chamber) {
                break;
            }
        }
        chamber.add_shape(shape);
        shapes += 1;

        let values = Values {
            shapes,
            height: chamber.height,
        };
        let state = (pattern_idx, chamber.drops());
        if let Some(prev) = seen.get(&state) {
            first = prev.clone();
            second = values;
            break;
        }
        seen.insert(state, values);
    }

    let repeat_shapes = second.shapes - first.shapes;
    let repeat_height = second.height - first.height;

    let rounds: usize = 1000000000000;
    let remain = rounds - first.shapes;

    let repeats = remain / repeat_shapes;
    let last = remain % repeat_shapes;

    // finish the last pieces
    for _ in 0..last {
        let mut shape = shape_generator.next_shape(chamber.height + 3);
        loop {
            let (_, dir) = pattern.next();
            shape.jet(dir, &chamber);
            if !shape.fall(&chamber) {
                break;
            }
        }
        chamber.add_shape(shape);
    }

    let last_height = chamber.height - second.height;

    let final_height = first.height + repeats * repeat_height + last_height;

    println!("height: {}", final_height);

    Ok(())
}
