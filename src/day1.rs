use crate::util;

pub fn part1(input: &str) {
    let lines = util::read_lines(input);
    let mut sum = 0;
    let mut sums: Vec<i32> = Vec::new();
    for line in lines {
        if line == "" {
            sums.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().expect("not a number");
        }
    }

    let max = sums.iter().max();

    println!("max: {:?}", max)
}

pub fn part2(input: &str) {
    let lines = util::read_lines(input);
    let mut sum = 0;
    let mut sums: Vec<i32> = Vec::new();
    for line in lines {
        if line == "" {
            sums.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().expect("not a number");
        }
    }

    sums.sort();

    let total: i32 = sums.iter().rev().take(3).sum();

    println!("total: {:?}", total)
}
