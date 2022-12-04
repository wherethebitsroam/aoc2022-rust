use std::collections::HashSet;

use crate::util;

fn find_duplicate_char(s: &str) -> Option<char> {
    let cs: Vec<char> = s.chars().collect();
    let size = cs.len() / 2;

    let first: HashSet<char> = cs[..size].iter().map(|c| *c).collect();
    let second: HashSet<char> = cs[size..].iter().map(|c| *c).collect();

    first.intersection(&second).next().map(|c| *c)
}

fn score(c: &char) -> u32 {
    if *c >= 'a' && *c <= 'z' {
        return *c as u32 - 'a' as u32 + 1;
    }
    if *c >= 'A' && *c <= 'Z' {
        return *c as u32 - 'A' as u32 + 27;
    }
    return 0;
}

pub fn part1(input: &str) -> u32 {
    let dups: Vec<char> = util::read_lines(input)
        .map(|line| find_duplicate_char(line).expect("missing duplicate"))
        .collect();

    println!("dups: {:?}", dups);

    let scores: Vec<u32> = dups.iter().map(|c| score(c)).collect();

    println!("scores: {:?}", scores);

    let sum = scores.iter().sum();

    println!("sum: {}", sum);

    sum
}

pub fn part2(input: &str) {
    let lines: Vec<&str> = util::read_lines(input).collect();

    let mut scores = Vec::new();
    for i in 0..lines.len() / 3 {
        let start = i * 3;

        let one: HashSet<char> = lines[start].chars().collect();
        let two: HashSet<char> = lines[start + 1].chars().collect();
        let three: HashSet<char> = lines[start + 2].chars().collect();

        let i1: HashSet<char> = one.intersection(&two).map(|c| *c).collect();

        let i2 = three
            .intersection(&i1)
            .next()
            .map(|c| score(c))
            .expect("one intersection");

        scores.push(i2);
    }

    let sum: u32 = scores.iter().sum();
    println!("sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(score(&'z'), 26);
        assert_eq!(score(&'Z'), 52);
    }

    #[test]
    fn part1_test() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(part1(input), 157)
    }
}
