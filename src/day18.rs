use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
};

use crate::util;

static TEST_INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn inside(&self, min: Self, max: Self) -> bool {
        self.x >= min.x
            && self.x <= max.x
            && self.y >= min.y
            && self.y <= max.y
            && self.z >= min.z
            && self.z <= max.z
    }

    fn neighbours(&self) -> [Self; 6] {
        [
            Point::new(self.x, self.y, self.z - 1),
            Point::new(self.x, self.y, self.z + 1),
            Point::new(self.x, self.y - 1, self.z),
            Point::new(self.x, self.y + 1, self.z),
            Point::new(self.x - 1, self.y, self.z),
            Point::new(self.x + 1, self.y, self.z),
        ]
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let blah: Vec<_> = value.split(",").collect();
        Self {
            x: blah[0].parse().unwrap(),
            y: blah[1].parse().unwrap(),
            z: blah[2].parse().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Side {
    axis: Axis,
    x: i64,
    y: i64,
    z: i64,
}

impl Side {
    fn new(axis: Axis, x: i64, y: i64, z: i64) -> Self {
        Self { axis, x, y, z }
    }
}

fn point_to_sides(p: Point) -> Vec<Side> {
    vec![
        Side::new(Axis::X, p.x - 1, p.y, p.z),
        Side::new(Axis::X, p.x, p.y, p.z),
        Side::new(Axis::Y, p.x, p.y - 1, p.z),
        Side::new(Axis::Y, p.x, p.y, p.z),
        Side::new(Axis::Z, p.x, p.y, p.z - 1),
        Side::new(Axis::Z, p.x, p.y, p.z),
    ]
}

fn sides(points: impl Iterator<Item = Point>) -> usize {
    let mut side_count = HashMap::new();
    for side in points.flat_map(point_to_sides) {
        side_count
            .entry(side)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    side_count.iter().filter(|x| *x.1 == 1).count()
}

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let sides = sides(util::read_lines(input).map(Point::from));
    println!("sides: {}", sides);
    Ok(())
}

fn split_into_regions(mut hs: HashSet<Point>) -> Vec<HashSet<Point>> {
    let mut regions = Vec::new();

    loop {
        let p = match hs.iter().next() {
            Some(p) => *p,
            None => break,
        };

        let mut region = HashSet::new();
        let mut test = VecDeque::new();
        test.push_back(p);

        loop {
            let t = match test.pop_front() {
                Some(t) => t,
                None => break,
            };
            region.insert(t);
            hs.remove(&t);

            for n in t.neighbours().iter().filter(|p| hs.contains(p)) {
                if test.contains(n) {
                    continue;
                }
                test.push_back(*n);
            }
        }

        regions.push(region);
    }

    regions
}

pub fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    // let input = TEST_INPUT;
    let blocks: HashSet<Point> = util::read_lines(input).map(Point::from).collect();

    // get bounding area
    let mut min = *blocks.iter().next().unwrap();
    let mut max = *blocks.iter().next().unwrap();
    for b in blocks.iter() {
        if b.x < min.x {
            min.x = b.x
        }
        if b.y < min.y {
            min.y = b.y
        }
        if b.z < min.z {
            min.z = b.z
        }
        if b.x > max.x {
            max.x = b.x
        }
        if b.y > max.y {
            max.y = b.y
        }
        if b.z > max.z {
            max.z = b.z
        }
    }

    // add 1 to min and max to get a boundary
    min.x -= 1;
    min.y -= 1;
    min.z -= 1;
    max.x += 1;
    max.y += 1;
    max.z += 1;

    println!("bounding: {:?} -> {:?}", min, max);

    // get all spaces in the area
    let mut all = HashSet::new();
    for x in min.x..=max.x {
        for y in min.y..=max.y {
            for z in min.z..=max.z {
                all.insert(Point::new(x, y, z));
            }
        }
    }

    let space: HashSet<Point> = all.difference(&blocks).map(|p| *p).collect();

    let regions = split_into_regions(space);

    let mut side_count = sides(blocks.into_iter());

    for r in regions {
        if !r.contains(&min) {
            side_count -= sides(r.into_iter());
        }
    }

    println!("sides: {}", side_count);

    Ok(())
}
