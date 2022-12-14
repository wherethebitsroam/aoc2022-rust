use std::error::Error;

use crate::util;

enum Instr {
    Addx(i32),
    Noop,
}

impl From<&str> for Instr {
    fn from(value: &str) -> Self {
        let x: Vec<_> = value.split(" ").collect();
        match x[0] {
            "addx" => Self::Addx(x[1].parse().unwrap()),
            "noop" => Self::Noop,
            i => panic!("Unknown instruction: {}", i),
        }
    }
}

impl Instr {
    fn cycle_length(&self) -> i32 {
        match self {
            Self::Addx(_) => 2,
            Self::Noop => 1,
        }
    }
}

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut x = 1;
    let mut cycle = 0;
    let mut checkpoint = 20;
    let mut sum = 0;

    for instr in util::read_lines(input).map(|s| Instr::from(s)) {
        cycle += instr.cycle_length();

        if cycle >= checkpoint {
            println!("{} {}", checkpoint, x);
            sum += x * checkpoint;
            checkpoint += 40;
        }

        match instr {
            Instr::Addx(inc) => x += inc,
            Instr::Noop => {}
        }
    }

    println!("sum: {}", sum);

    Ok(())
}

pub fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let mut x: i32 = 1;
    let mut cycle = 0;
    let mut screen = [false; 240];

    for instr in util::read_lines(input).map(|s| Instr::from(s)) {
        for _ in 0..instr.cycle_length() {
            screen[cycle] = x - 1 <= cycle as i32 % 40 && x + 1 >= cycle as i32 % 40;
            cycle += 1;
        }

        match instr {
            Instr::Addx(inc) => x += inc,
            Instr::Noop => {}
        }
    }

    for r in 0..6 {
        for c in 0..40 {
            let ch = if screen[r * 40 + c] { "#" } else { "." };
            print!("{}", ch);
        }
        println!();
    }

    Ok(())
}
