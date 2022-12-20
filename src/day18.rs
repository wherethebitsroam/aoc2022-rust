use std::{collections::HashMap, error::Error};

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

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut side_count = HashMap::new();
    for side in util::read_lines(input)
        .map(Point::from)
        .flat_map(point_to_sides)
    {
        side_count
            .entry(side)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let sides = side_count.iter().filter(|x| *x.1 == 1).count();

    println!("sides: {}", sides);

    Ok(())
}

pub fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let mut side_count = HashMap::new();
    for side in util::read_lines(TEST_INPUT)
        .map(Point::from)
        .flat_map(point_to_sides)
    {
        side_count
            .entry(side)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let sides: Vec<_> = side_count
        .into_iter()
        .filter_map(|x| if x.1 == 1 { Some(x.0) } else { None })
        .collect();

    println!("sides: {}", sides.len());

    Ok(())
}
