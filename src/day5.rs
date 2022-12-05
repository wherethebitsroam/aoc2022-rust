use crate::util;

pub fn part1(input: &str) {
    let mut stacks = vec![
        vec![],
        vec!['N', 'C', 'R', 'T', 'M', 'Z', 'P'],
        vec!['D', 'N', 'T', 'S', 'B', 'Z'],
        vec!['M', 'H', 'Q', 'R', 'F', 'C', 'T', 'G'],
        vec!['G', 'R', 'Z'],
        vec!['Z', 'N', 'R', 'H'],
        vec!['F', 'H', 'S', 'W', 'P', 'Z', 'L', 'D'],
        vec!['W', 'D', 'Z', 'R', 'C', 'G', 'M'],
        vec!['S', 'J', 'F', 'L', 'H', 'W', 'Z', 'Q'],
        vec!['S', 'Q', 'P', 'W', 'N'],
    ];

    let mut moves = false;
    for line in util::read_lines(input) {
        if moves {
            let c: Vec<_> = line.split(' ').collect();
            let count: usize = c[1].parse().expect("bad int");
            let from: usize = c[3].parse().expect("bad int");
            let to: usize = c[5].parse().expect("bad int");

            for _ in 0..count {
                let x = stacks[from].pop().expect("empty stack!!");
                stacks[to].push(x);
            }
        }

        if line == "" {
            moves = true;
        }
    }

    for i in 1..stacks.len() {
        print!("{}", stacks[i].last().unwrap())
    }
    println!("")
}

pub fn part2(input: &str) {
    let mut stacks = vec![
        vec![],
        vec!['N', 'C', 'R', 'T', 'M', 'Z', 'P'],
        vec!['D', 'N', 'T', 'S', 'B', 'Z'],
        vec!['M', 'H', 'Q', 'R', 'F', 'C', 'T', 'G'],
        vec!['G', 'R', 'Z'],
        vec!['Z', 'N', 'R', 'H'],
        vec!['F', 'H', 'S', 'W', 'P', 'Z', 'L', 'D'],
        vec!['W', 'D', 'Z', 'R', 'C', 'G', 'M'],
        vec!['S', 'J', 'F', 'L', 'H', 'W', 'Z', 'Q'],
        vec!['S', 'Q', 'P', 'W', 'N'],
    ];

    let mut moves = false;
    for line in util::read_lines(input) {
        if moves {
            let c: Vec<_> = line.split(' ').collect();
            let count: usize = c[1].parse().expect("bad int");
            let from: usize = c[3].parse().expect("bad int");
            let to: usize = c[5].parse().expect("bad int");

            let mut tmp = Vec::new();
            for _ in 0..count {
                let x = stacks[from].pop().expect("empty stack!!");
                tmp.push(x);
            }

            for _ in 0..count {
                stacks[to].push(tmp.pop().unwrap())
            }
        }

        if line == "" {
            moves = true;
        }
    }

    for i in 1..stacks.len() {
        print!("{}", stacks[i].last().unwrap())
    }
    println!("")
}
