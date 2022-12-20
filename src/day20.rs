use std::error::Error;

use crate::util;

static TEST_INPUT: &str = "1
2
-3
3
-2
0
4";

fn parse(s: &str) -> Vec<i64> {
    util::read_lines(s).map(|x| x.parse().unwrap()).collect()
}

fn to_move(value: i64, len: usize) -> usize {
    // because we remove the thing being moved
    let len = len - 1;
    let mut m = value;
    if m < 0 {
        let lens = (-m) / (len as i64) + 1;
        m += lens * len as i64;
    }
    m as usize % len
}

struct Mix {
    values: Vec<i64>,
    zero: usize,
    moves: Vec<usize>,
    prev: Vec<usize>,
    next: Vec<usize>,
}

impl Mix {
    fn new(values: Vec<i64>) -> Self {
        let zero = values.iter().position(|x| *x == 0).unwrap();
        let moves: Vec<usize> = values.iter().map(|x| to_move(*x, values.len())).collect();
        let prev: Vec<usize> = (0..values.len())
            .map(|x| (values.len() + x - 1) % values.len())
            .collect();
        let next: Vec<usize> = (0..values.len()).map(|x| (x + 1) % values.len()).collect();
        Self {
            values,
            zero,
            moves,
            prev,
            next,
        }
    }

    fn mov(&mut self, i: usize) {
        if self.moves[i] == 0 {
            return;
        }

        // remove the thing being moved
        let n = self.next[i];
        let p = self.prev[i];
        self.next[p] = n;
        self.prev[n] = p;

        // find where we go
        let mut after = i;
        for _ in 0..self.moves[i] {
            after = self.next[after];
        }

        let n = self.next[after];
        self.next[after] = i;
        self.prev[i] = after;
        self.next[i] = n;
        self.prev[n] = i;
    }

    fn print_from_zero(&self) {
        let mut i = self.zero;
        loop {
            print!("{}, ", self.values[i]);
            i = self.next[i];
            if i == self.zero {
                break;
            }
        }
        println!();
    }

    fn find_from_zero(&self, n: usize) -> i64 {
        let mut i = self.zero;
        let n = n % self.values.len();
        for _ in 0..n {
            i = self.next[i];
        }
        self.values[i]
    }
}

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    // let input = TEST_INPUT;

    let values = parse(input);
    let mut mix = Mix::new(values);

    for i in 0..mix.values.len() {
        mix.mov(i);
    }

    let v1000 = mix.find_from_zero(1000);
    let v2000 = mix.find_from_zero(2000);
    let v3000 = mix.find_from_zero(3000);

    println!("{}, {}, {}", v1000, v2000, v3000);

    let sum = v1000 + v2000 + v3000;

    println!("sum: {}", sum);

    Ok(())
}

pub fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    // let input = TEST_INPUT;
    let values = parse(input).iter().map(|x| *x * 811589153).collect();
    let mut mix = Mix::new(values);

    // mix 10 times
    for j in 0..10 {
        println!("round: {}", j);
        for i in 0..mix.values.len() {
            mix.mov(i);
        }
    }

    let v1000 = mix.find_from_zero(1000);
    let v2000 = mix.find_from_zero(2000);
    let v3000 = mix.find_from_zero(3000);

    println!("{}, {}, {}", v1000, v2000, v3000);

    let sum = v1000 + v2000 + v3000;

    println!("sum: {}", sum);
    Ok(())
}
