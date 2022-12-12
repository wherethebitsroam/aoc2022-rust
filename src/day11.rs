use std::{collections::VecDeque, error::Error};

#[derive(Debug, Clone, Copy)]
enum Op {
    Add(i64),
    Mul(i64),
    Square,
}

impl Op {
    fn apply(&self, v: i64) -> i64 {
        match self {
            Self::Add(x) => v + x,
            Self::Mul(x) => v * x,
            Self::Square => v * v,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    inspections: usize,
    op: Op,
    div: i64,
    dest_true: usize,
    dest_false: usize,
}

impl Monkey {
    fn dest(&self, v: i64) -> usize {
        if v % self.div == 0 {
            self.dest_true
        } else {
            self.dest_false
        }
    }
}

#[derive(Debug)]
struct MonkeyBusiness {
    monkeys: Vec<Monkey>,
}

impl MonkeyBusiness {
    fn round(&mut self) {
        for i in 0..self.monkeys.len() {
            while let Some(item) = self.monkeys[i].items.pop_front() {
                self.monkeys[i].inspections += 1;

                let new = self.monkeys[i].op.apply(item) / 3;
                let dest = self.monkeys[i].dest(new);

                self.monkeys[dest].items.push_back(new);
            }
        }
    }

    fn round_v2(&mut self, div: i64) {
        for i in 0..self.monkeys.len() {
            while let Some(item) = self.monkeys[i].items.pop_front() {
                self.monkeys[i].inspections += 1;

                let new = self.monkeys[i].op.apply(item) % div;
                let dest = self.monkeys[i].dest(new);

                self.monkeys[dest].items.push_back(new);
            }
        }
    }
}

fn test_input() -> MonkeyBusiness {
    let monkeys = vec![
        Monkey {
            items: VecDeque::from([79, 98]),
            inspections: 0,
            op: Op::Mul(19),
            div: 23,
            dest_true: 2,
            dest_false: 3,
        },
        Monkey {
            items: VecDeque::from([54, 65, 75, 74]),
            inspections: 0,
            op: Op::Add(6),
            div: 19,
            dest_true: 2,
            dest_false: 0,
        },
        Monkey {
            items: VecDeque::from([79, 60, 97]),
            inspections: 0,
            op: Op::Square,
            div: 13,
            dest_true: 1,
            dest_false: 3,
        },
        Monkey {
            items: VecDeque::from([74]),
            inspections: 0,
            op: Op::Add(3),
            div: 17,
            dest_true: 0,
            dest_false: 1,
        },
    ];
    MonkeyBusiness { monkeys }
}

fn input() -> MonkeyBusiness {
    let monkeys = vec![
        Monkey {
            items: VecDeque::from([72, 64, 51, 57, 93, 97, 68]),
            inspections: 0,
            op: Op::Mul(19),
            div: 17,
            dest_true: 4,
            dest_false: 7,
        },
        Monkey {
            items: VecDeque::from([62]),
            inspections: 0,
            op: Op::Mul(11),
            div: 3,
            dest_true: 3,
            dest_false: 2,
        },
        Monkey {
            items: VecDeque::from([57, 94, 69, 79, 72]),
            inspections: 0,
            op: Op::Add(6),
            div: 19,
            dest_true: 0,
            dest_false: 4,
        },
        Monkey {
            items: VecDeque::from([80, 64, 92, 93, 64, 56]),
            inspections: 0,
            op: Op::Add(5),
            div: 7,
            dest_true: 2,
            dest_false: 0,
        },
        Monkey {
            items: VecDeque::from([70, 88, 95, 99, 78, 72, 65, 94]),
            inspections: 0,
            op: Op::Add(7),
            div: 2,
            dest_true: 7,
            dest_false: 5,
        },
        Monkey {
            items: VecDeque::from([57, 95, 81, 61]),
            inspections: 0,
            op: Op::Square,
            div: 5,
            dest_true: 1,
            dest_false: 6,
        },
        Monkey {
            items: VecDeque::from([79, 99]),
            inspections: 0,
            op: Op::Add(2),
            div: 11,
            dest_true: 3,
            dest_false: 1,
        },
        Monkey {
            items: VecDeque::from([68, 98, 62]),
            inspections: 0,
            op: Op::Add(3),
            div: 13,
            dest_true: 5,
            dest_false: 6,
        },
    ];
    MonkeyBusiness { monkeys }
}

pub fn part1(_: &str) -> Result<(), Box<dyn Error>> {
    let mut mb = input();
    for _ in 0..20 {
        mb.round();
    }

    let mut inspections: Vec<_> = mb.monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort();
    inspections.reverse();

    let score: usize = inspections.iter().take(2).product();

    println!("score: {}", score);

    Ok(())
}

pub fn part2(_: &str) -> Result<(), Box<dyn Error>> {
    let mut mb = input();
    let div: i64 = mb.monkeys.iter().map(|m| m.div).product();
    for _ in 0..10000 {
        mb.round_v2(div);
    }

    let mut inspections: Vec<_> = mb.monkeys.iter().map(|m| m.inspections).collect();
    println!("inpections: {:?}", inspections);
    inspections.sort();
    inspections.reverse();

    let score: usize = inspections.iter().take(2).product();

    println!("score: {}", score);

    Ok(())
}
