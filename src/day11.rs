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
struct Item {
    value: i64,
    ops: Vec<Op>,
}

impl Item {
    fn new(value: i64) -> Self {
        Self {
            value,
            ops: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<Item>,
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

                let new = self.monkeys[i].op.apply(item.value) / 3;
                let dest = self.monkeys[i].dest(new);

                self.monkeys[dest].items.push_back(Item::new(new));
            }
        }
    }

    fn round_v2(&mut self) {
        for i in 0..self.monkeys.len() {
            while let Some(mut item) = self.monkeys[i].items.pop_front() {
                self.monkeys[i].inspections += 1;

                item.ops.push(self.monkeys[i].op);

                let mut v = item.value;
                for op in item.ops.iter() {
                    v = op.apply(v) % self.monkeys[i].div;
                }

                let dest = self.monkeys[i].dest(v);

                self.monkeys[dest].items.push_back(item);
            }
        }
    }
}

fn test_input() -> MonkeyBusiness {
    let monkeys = vec![
        Monkey {
            items: VecDeque::from([Item::new(79), Item::new(98)]),
            inspections: 0,
            op: Op::Mul(19),
            div: 23,
            dest_true: 2,
            dest_false: 3,
        },
        Monkey {
            items: VecDeque::from([Item::new(54), Item::new(65), Item::new(75), Item::new(74)]),
            inspections: 0,
            op: Op::Add(6),
            div: 19,
            dest_true: 2,
            dest_false: 0,
        },
        Monkey {
            items: VecDeque::from([Item::new(79), Item::new(60), Item::new(97)]),
            inspections: 0,
            op: Op::Square,
            div: 13,
            dest_true: 1,
            dest_false: 3,
        },
        Monkey {
            items: VecDeque::from([Item::new(74)]),
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
            items: VecDeque::from([
                Item::new(72),
                Item::new(64),
                Item::new(51),
                Item::new(57),
                Item::new(93),
                Item::new(97),
                Item::new(68),
            ]),
            inspections: 0,
            op: Op::Mul(19),
            div: 17,
            dest_true: 4,
            dest_false: 7,
        },
        Monkey {
            items: VecDeque::from([Item::new(62)]),
            inspections: 0,
            op: Op::Mul(11),
            div: 3,
            dest_true: 3,
            dest_false: 2,
        },
        Monkey {
            items: VecDeque::from([
                Item::new(57),
                Item::new(94),
                Item::new(69),
                Item::new(79),
                Item::new(72),
            ]),
            inspections: 0,
            op: Op::Add(6),
            div: 19,
            dest_true: 0,
            dest_false: 4,
        },
        Monkey {
            items: VecDeque::from([
                Item::new(80),
                Item::new(64),
                Item::new(92),
                Item::new(93),
                Item::new(64),
                Item::new(56),
            ]),
            inspections: 0,
            op: Op::Add(5),
            div: 7,
            dest_true: 2,
            dest_false: 0,
        },
        Monkey {
            items: VecDeque::from([
                Item::new(70),
                Item::new(88),
                Item::new(95),
                Item::new(99),
                Item::new(78),
                Item::new(72),
                Item::new(65),
                Item::new(94),
            ]),
            inspections: 0,
            op: Op::Add(7),
            div: 2,
            dest_true: 7,
            dest_false: 5,
        },
        Monkey {
            items: VecDeque::from([Item::new(57), Item::new(95), Item::new(81), Item::new(61)]),
            inspections: 0,
            op: Op::Square,
            div: 5,
            dest_true: 1,
            dest_false: 6,
        },
        Monkey {
            items: VecDeque::from([Item::new(79), Item::new(99)]),
            inspections: 0,
            op: Op::Add(2),
            div: 11,
            dest_true: 3,
            dest_false: 1,
        },
        Monkey {
            items: VecDeque::from([Item::new(68), Item::new(98), Item::new(62)]),
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
    for _ in 0..10000 {
        mb.round_v2();
    }

    let mut inspections: Vec<_> = mb.monkeys.iter().map(|m| m.inspections).collect();
    println!("inpections: {:?}", inspections);
    inspections.sort();
    inspections.reverse();

    let score: usize = inspections.iter().take(2).product();

    println!("score: {}", score);

    Ok(())
}
