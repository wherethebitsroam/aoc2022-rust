use crate::util;

struct Assignment {
    lo: u32,
    hi: u32,
}

impl From<&str> for Assignment {
    fn from(s: &str) -> Self {
        let v: Vec<&str> = s.split('-').collect();
        if v.len() != 2 {
            panic!("expected 2 bits, {}", s)
        }
        Assignment {
            lo: v[0].parse().expect("bad int"),
            hi: v[1].parse().expect("bad int"),
        }
    }
}

impl Assignment {
    fn contains(&self, other: &Self) -> bool {
        self.lo <= other.lo && self.hi >= other.hi
    }
}

struct Pair {
    a1: Assignment,
    a2: Assignment,
}

impl From<&str> for Pair {
    fn from(s: &str) -> Self {
        let v: Vec<&str> = s.split(',').collect();
        if v.len() != 2 {
            panic!("expected 2 bits, {}", s)
        }
        Pair {
            a1: v[0].into(),
            a2: v[1].into(),
        }
    }
}

impl Pair {
    fn one_contained(&self) -> bool {
        self.a1.contains(&self.a2) || self.a2.contains(&self.a1)
    }

    fn overlap(&self) -> bool {
        self.a1.hi >= self.a2.lo && self.a2.hi >= self.a1.lo
    }
}

pub fn part1(input: &str) {
    let count = util::read_lines(input)
        .map(|s| Pair::from(s))
        .filter(|p| p.one_contained())
        .count();

    println!("count: {}", count);
}

pub fn part2(input: &str) {
    let count = util::read_lines(input)
        .map(|s| Pair::from(s))
        .filter(|p| p.overlap())
        .count();

    println!("count: {}", count);
}
