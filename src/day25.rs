use std::error::Error;

use crate::util;

static TEST_INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

fn from_snafu(s: &str) -> i64 {
    let mut p = 1;
    let mut sum = 0;
    for c in s.chars().rev() {
        let x = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("bad snafu: {}", c),
        };
        sum += x * p;
        p *= 5;
    }
    sum
}

fn to_snafu(i: i64) -> String {
    let mut num = i;
    let mut digits = Vec::new();

    while num > 0 {
        let d = num % 5;
        digits.push(d);
        num /= 5;
    }

    // add a leading 0 to make things easier
    digits.push(0);

    // convert range
    for i in 0..digits.len() {
        if digits[i] == 3 {
            digits[i] = -2;
            digits[i + 1] += 1;
        }
        if digits[i] == 4 {
            digits[i] = -1;
            digits[i + 1] += 1;
        }
        if digits[i] == 5 {
            digits[i] = 0;
            digits[i + 1] += 1;
        }
    }

    // remove unnecessary leading zeros
    if let Some(0) = digits.last() {
        digits.pop();
    }

    digits
        .iter()
        .rev()
        .map(|x| match x {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            -2 => '=',
            _ => panic!("bah!"),
        })
        .collect()
}

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let result: i64 = util::read_lines(input).map(from_snafu).sum();
    println!("{}", to_snafu(result));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_snafu() {
        assert_eq!(from_snafu("1=-0-2"), 1747);
        assert_eq!(from_snafu("12111"), 906);
        assert_eq!(from_snafu("2=0="), 198);
        assert_eq!(from_snafu("21"), 11);
        assert_eq!(from_snafu("2=01"), 201);
        assert_eq!(from_snafu("111"), 31);
        assert_eq!(from_snafu("20012"), 1257);
        assert_eq!(from_snafu("112"), 32);
        assert_eq!(from_snafu("1=-1="), 353);
        assert_eq!(from_snafu("1-12"), 107);
        assert_eq!(from_snafu("12"), 7);
        assert_eq!(from_snafu("1="), 3);
        assert_eq!(from_snafu("122"), 37);
    }

    #[test]
    fn test_to_snafu() {
        assert_eq!("1=-0-2", to_snafu(1747));
        assert_eq!("12111", to_snafu(906));
        assert_eq!("2=0=", to_snafu(198));
        assert_eq!("21", to_snafu(11));
        assert_eq!("2=01", to_snafu(201));
        assert_eq!("111", to_snafu(31));
        assert_eq!("20012", to_snafu(1257));
        assert_eq!("112", to_snafu(32));
        assert_eq!("1=-1=", to_snafu(353));
        assert_eq!("1-12", to_snafu(107));
        assert_eq!("12", to_snafu(7));
        assert_eq!("1=", to_snafu(3));
        assert_eq!("122", to_snafu(37));
    }
}
