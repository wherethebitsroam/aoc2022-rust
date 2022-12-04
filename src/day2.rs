use crate::util;

enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn from_letter(l: &str) -> Self {
        match l {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,
            _ => panic!("bad"),
        }
    }

    fn from_result(l: &str, them: &Self) -> Self {
        match (l, them) {
            // lose
            ("X", Self::Rock) => Self::Scissors,
            ("X", Self::Paper) => Self::Rock,
            ("X", Self::Scissors) => Self::Paper,
            // draw
            ("Y", Self::Rock) => Self::Rock,
            ("Y", Self::Paper) => Self::Paper,
            ("Y", Self::Scissors) => Self::Scissors,
            // win
            ("Z", Self::Rock) => Self::Paper,
            ("Z", Self::Paper) => Self::Scissors,
            ("Z", Self::Scissors) => Self::Rock,
            _ => panic!("bad"),
        }
    }

    fn played_points(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn result_points(them: &Self, me: &Self) -> i32 {
        match (them, me) {
            (Self::Rock, Self::Rock) => 3,
            (Self::Rock, Self::Paper) => 6,
            (Self::Rock, Self::Scissors) => 0,
            (Self::Paper, Self::Rock) => 0,
            (Self::Paper, Self::Paper) => 3,
            (Self::Paper, Self::Scissors) => 6,
            (Self::Scissors, Self::Rock) => 6,
            (Self::Scissors, Self::Paper) => 0,
            (Self::Scissors, Self::Scissors) => 3,
        }
    }
}

pub fn part1(input: &str) {
    let lines = util::read_lines(input);

    let score: i32 = lines
        .filter(|line| *line != "")
        .map(|line| {
            let chars: Vec<&str> = line.split(' ').collect();
            if chars.len() != 2 {
                panic!("expected 2 chars, got >{}<", line)
            }
            (RPS::from_letter(chars[0]), RPS::from_letter(chars[1]))
        })
        .map(|(them, me)| me.played_points() + RPS::result_points(&them, &me))
        .sum();

    println!("score: {}", score);
}

pub fn part2(input: &str) {
    let lines = util::read_lines(input);

    let score: i32 = lines
        .filter(|line| *line != "")
        .map(|line| {
            let chars: Vec<&str> = line.split(' ').collect();
            if chars.len() != 2 {
                panic!("expected 2 chars, got >{}<", line)
            }
            let them = RPS::from_letter(chars[0]);
            let me = RPS::from_result(chars[1], &them);
            (them, me)
        })
        .map(|(them, me)| me.played_points() + RPS::result_points(&them, &me))
        .sum();

    println!("score: {}", score);
}
