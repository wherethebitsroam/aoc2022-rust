use std::{collections::HashMap, error::Error, fmt::Display};

use crate::util;

static TEST_INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

#[derive(Debug, Clone, Copy)]
enum Op {
    Plus,
    Minus,
    Mul,
    Div,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Plus => '+',
                Self::Minus => '-',
                Self::Mul => '*',
                Self::Div => '/',
            }
        )
    }
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("bad op"),
        }
    }
}

// #[derive(Debug, Clone)]
// enum MonkeyOp {
//     Num(i64),
//     Op(usize, Op, usize),
// }

enum MonkeyOp {
    Num(i64),
    Op(String, Op, String),
}

fn parse(value: &str) -> (String, MonkeyOp) {
    let parts: Vec<_> = value.split_ascii_whitespace().collect();
    let id = parts[0].strip_suffix(':').unwrap().to_owned();
    match parts.len() {
        2 => (id, MonkeyOp::Num(parts[1].parse().unwrap())),
        4 => (
            id,
            MonkeyOp::Op(parts[1].to_owned(), Op::from(parts[2]), parts[3].to_owned()),
        ),
        x => panic!("bad monkey {}", x),
    }
}

fn derive(id: &str, ops: &HashMap<String, MonkeyOp>, derived: &mut HashMap<String, i64>) {
    if let Some(op) = ops.get(id) {
        match op {
            MonkeyOp::Num(x) => {
                derived.insert(id.to_owned(), *x);
            }
            MonkeyOp::Op(a, op, b) => {
                derive(a, ops, derived);
                derive(b, ops, derived);
                match (derived.get(a), derived.get(b)) {
                    (Some(a), Some(b)) => {
                        let x = match op {
                            Op::Plus => a + b,
                            Op::Minus => a - b,
                            Op::Div => a / b,
                            Op::Mul => a * b,
                        };
                        derived.insert(id.to_owned(), x);
                    }
                    _ => {}
                }
            }
        }
    }
}

fn dump(ops: &HashMap<String, MonkeyOp>) {
    let mut keys: Vec<_> = ops.keys().collect();
    keys.sort();
    for id in keys.iter() {
        match &ops[*id] {
            MonkeyOp::Num(x) => println!("{}: {}", id, x),
            MonkeyOp::Op(a, op, b) => println!("{}: {} {} {}", id, a, op, b),
        }
    }
}

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    // let input = TEST_INPUT;

    let ops: HashMap<String, MonkeyOp> = util::read_lines(input).map(parse).collect();
    let mut derived = HashMap::new();
    // dump(&ops);

    derive("root", &ops, &mut derived);

    println!("result: {}", derived["root"]);

    Ok(())
}

fn derive_backwards(
    id: &str,
    value: i64,
    ops: &HashMap<String, MonkeyOp>,
    derived: &HashMap<String, i64>,
) -> i64 {
    if id == "humn" {
        println!("derive_backwards: id: {}, value: {}", id, value);
        return value;
    }
    match &ops[id] {
        MonkeyOp::Num(_) => panic!("unexpected num arm for, {}", id),
        MonkeyOp::Op(a, op, b) => {
            // expect one of the arms to be derived
            if let Some(a) = derived.get(a) {
                let v = match op {
                    Op::Plus => value - *a,
                    Op::Minus => *a - value,
                    Op::Mul => value / *a,
                    Op::Div => *a / value,
                };

                derive_backwards(b, v, ops, derived);
            } else if let Some(b) = derived.get(b) {
                let v = match op {
                    Op::Plus => value - *b,
                    Op::Minus => value + *b,
                    Op::Mul => value / *b,
                    Op::Div => value * *b,
                };

                derive_backwards(a, v, ops, derived);
            } else {
                panic!("didn't find derived values for {} or {}", a, b);
            }
        }
    }
    0
}

pub fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    // let input = TEST_INPUT;

    let ops: HashMap<String, MonkeyOp> = util::read_lines(input)
        .map(parse)
        .filter(|(id, _)| id != "humn")
        .collect();
    let mut derived = HashMap::new();

    derive("root", &ops, &mut derived);

    if let MonkeyOp::Op(a, _, b) = &ops["root"] {
        let result = if let Some(a) = derived.get(a) {
            derive_backwards(&b, *a, &ops, &derived)
        } else if let Some(b) = derived.get(b) {
            derive_backwards(&a, *b, &ops, &derived)
        } else {
            panic!("didn't find derived values for {} or {}", a, b);
        };
        // FIXME for some reason this is always zero??
        println!("result: {}", result);
    }

    Ok(())
}
