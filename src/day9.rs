use std::{collections::HashSet, error::Error};

use crate::util;

static TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

static TEST_INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn add(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    fn diff(&self, other: &Self) -> (i32, i32) {
        (other.x - self.x, other.y - self.y)
    }
}

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut h = Point::new(0, 0);
    let mut t = Point::new(0, 0);

    let mut set = HashSet::new();
    set.insert(t.clone());

    for line in util::read_lines(input) {
        let (d, x): (&str, usize) = line
            .split_once(' ')
            .map(|(x, y)| (x, y.parse().unwrap()))
            .unwrap();

        for _ in 0..x {
            // move the head
            match d {
                "U" => h.add(0, 1),
                "D" => h.add(0, -1),
                "R" => h.add(1, 0),
                "L" => h.add(-1, 0),
                _ => panic!("bah"),
            }

            // move the tail
            match t.diff(&h) {
                (-1 | 0 | 1, -1 | 0 | 1) => {} // noop
                (2, y) => t.add(1, y),
                (-2, y) => t.add(-1, y),
                (x, 2) => t.add(x, 1),
                (x, -2) => t.add(x, -1),
                p => panic!("bah: {:?}", p),
            }

            set.insert(t.clone());
        }
    }
    println!("len: {}", set.len());
    Ok(())
}

pub fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let mut rope = Vec::new();
    for _ in 0..10 {
        rope.push(Point::new(0, 0));
    }

    let mut set = HashSet::new();
    set.insert(rope[9].clone());

    for line in util::read_lines(input) {
        let (d, x): (&str, usize) = line
            .split_once(' ')
            .map(|(x, y)| (x, y.parse().unwrap()))
            .unwrap();

        for _ in 0..x {
            // move the head
            match d {
                "U" => rope[0].add(0, 1),
                "D" => rope[0].add(0, -1),
                "R" => rope[0].add(1, 0),
                "L" => rope[0].add(-1, 0),
                _ => panic!("bah"),
            }

            // move the tail
            for i in 1..10 {
                match rope[i].diff(&rope[i - 1]) {
                    (-1 | 0 | 1, -1 | 0 | 1) => {} // noop
                    // this seems a special case? not covered?
                    (2, 2) => rope[i].add(1, 1),
                    (2, -2) => rope[i].add(1, -1),
                    (-2, 2) => rope[i].add(-1, 1),
                    (-2, -2) => rope[i].add(-1, -1),
                    // covered
                    (2, y) => rope[i].add(1, y),
                    (-2, y) => rope[i].add(-1, y),
                    (x, 2) => rope[i].add(x, 1),
                    (x, -2) => rope[i].add(x, -1),
                    p => {
                        println!("{:?}", rope);
                        panic!("i: {}, bad diff: {:?}", i, p);
                    }
                }
            }

            set.insert(rope[9].clone());
        }
        // println!("{:?}", rope);
    }
    println!("len: {}", set.len());
    Ok(())
}
