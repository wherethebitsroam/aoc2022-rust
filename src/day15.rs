use std::{collections::HashMap, error::Error};

use crate::util;

static TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Loc(i32, i32);

impl Loc {
    fn manhattan_distance(&self, other: Loc) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Space {
    Beacon,
    Sensor,
    Empty,
}

// e.g. y=1:
fn parse_assign(s: &str) -> i32 {
    let s = match s.strip_suffix(':') {
        Some(s) => s,
        None => s,
    };
    let s = match s.strip_suffix(',') {
        Some(s) => s,
        None => s,
    };
    let blah: Vec<_> = s.split('=').collect();
    blah[1].parse().expect("bad int")
}

#[derive(Debug)]
struct Pair {
    sensor: Loc,
    beacon: Loc,
    dist: i32,
}

#[derive(Debug)]
struct Map {
    spaces: HashMap<Loc, Space>,
    pairs: Vec<Pair>,
}

impl Map {
    fn parse(s: &str) -> Self {
        let mut spaces = HashMap::new();
        let mut pairs = Vec::new();
        for line in util::read_lines(s) {
            let blah: Vec<_> = line.split_ascii_whitespace().collect();
            let sx = parse_assign(blah[2]);
            let sy = parse_assign(blah[3]);
            let bx = parse_assign(blah[8]);
            let by = parse_assign(blah[9]);

            let sensor = Loc(sx, sy);
            let beacon = Loc(bx, by);

            pairs.push(Pair {
                sensor,
                beacon,
                dist: sensor.manhattan_distance(beacon),
            });

            spaces.insert(sensor, Space::Sensor);
            spaces.insert(beacon, Space::Beacon);
        }

        Self { spaces, pairs }
    }

    fn fill_empty(&mut self) {
        for pair in self.pairs.iter() {
            let dist = pair.dist;

            for dx in -dist..=dist {
                let disty = dist - dx.abs();
                for dy in -disty..=disty {
                    let l = Loc(pair.sensor.0 + dx, pair.sensor.1 + dy);
                    self.spaces.entry(l).or_insert(Space::Empty);
                }
            }
        }
    }

    fn fill_empty_row(&mut self, row: i32) {
        for pair in self.pairs.iter() {
            let dist = pair.dist;
            let dy = (pair.sensor.1 - row).abs();
            println!(
                "sensor: {:?}, beacon: {:?}, dist: {}, dy: {}",
                pair.sensor, pair.beacon, dist, dy
            );
            if dy > dist {
                continue;
            }

            let distx = dist - dy;

            for dx in -distx..=distx {
                let l = Loc(pair.sensor.0 + dx, row);
                self.spaces.entry(l).or_insert(Space::Empty);
            }
        }
    }

    // only one spot for a beacon
    // therefore it must lie at manhatten distance + 1 to
    // one or more sensors
    fn possible(&self, min: i32, max: i32) -> HashMap<Loc, usize> {
        let mut possible = HashMap::new();

        for pair in self.pairs.iter() {
            let dist = pair.dist + 1;
            for dx in -dist..=dist {
                let disty = dist - dx.abs();
                let x = pair.sensor.0 + dx;
                if x >= min && x <= max {
                    let mut blah = Vec::new();
                    if disty == 0 {
                        blah.push(Loc(x, pair.sensor.1));
                    } else {
                        let miny = pair.sensor.1 - disty;
                        if miny >= min && miny <= max {
                            blah.push(Loc(x, miny));
                        }
                        let maxy = pair.sensor.1 + disty;
                        if maxy >= min && maxy <= max {
                            blah.push(Loc(x, maxy));
                        }
                    }

                    for p in blah {
                        possible
                            .entry(p)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                }
            }
        }

        possible
    }

    fn is_outside(&self, l: Loc) -> bool {
        for p in self.pairs.iter() {
            if l.manhattan_distance(p.sensor) <= p.dist {
                return false;
            }
        }
        true
    }
}

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut map = Map::parse(input);

    // let row = 10;
    let row = 2000000;
    map.fill_empty_row(row);

    let non_beacon = map
        .spaces
        .iter()
        .filter(|(&l, &s)| l.1 == row && s != Space::Beacon)
        .count();

    println!("non beacon: {}", non_beacon);
    Ok(())
}

pub fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    // let map = Map::parse(TEST_INPUT);
    // let pos = map.possible(0, 20);
    let map = Map::parse(input);
    let pos = map.possible(0, 4000000);

    // sort by number of overlaps decending.
    // the point we're looking for should be on the edge of many ranges
    let mut pos: Vec<_> = pos.iter().collect();
    pos.sort_by(|x, y| y.1.cmp(x.1));

    // go through the points and do a real test
    for (p, _) in pos {
        if map.is_outside(*p) {
            println!("soln: {:?}", p);
            let freq = p.0 as i64 * 4000000 + p.1 as i64;
            println!("freq: {}", freq);
            break;
        }
    }
    Ok(())
}
