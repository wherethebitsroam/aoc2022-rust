use std::{cmp::Ordering, error::Error, str::Chars};

use crate::util;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Value {
    Int(i32),
    List(Vec<Value>),
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::List(ll), Value::List(rl)) => {
                for (l, r) in ll.iter().zip(rl) {
                    match l.cmp(r) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        _ => {}
                    }
                }
                // after the matched elements, compare the length
                ll.len().cmp(&rl.len())
            }
            (Value::Int(li), Value::Int(ri)) => li.cmp(&ri),
            (Value::List(ll), Value::Int(ri)) => {
                Value::List(ll.to_vec()).cmp(&Value::List(vec![Value::Int(*ri)]))
            }
            (Value::Int(li), Value::List(rl)) => {
                Value::List(vec![Value::Int(*li)]).cmp(&Value::List(rl.to_vec()))
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Parser<'a> {
    iter: Chars<'a>,
}

impl<'a> Parser<'a> {
    fn parse(s: &'a str) -> Value {
        let mut iter = s.chars();
        // get the first char, which should be a `[`
        let first = iter.next();
        if first != Some('[') {
            panic!("Bad start: {:?}", first);
        }
        let mut parser = Self { iter };
        parser.parse_list()
    }

    fn parse_int(&mut self, c: char) -> (Value, /* end bracket found */ bool) {
        let mut v = c as i32 - '0' as i32;
        while let Some(c) = self.iter.next() {
            match c {
                c @ '0'..='9' => v = v * 10 + (c as i32 - '0' as i32),
                ',' => {
                    return (Value::Int(v), false);
                }
                ']' => {
                    return (Value::Int(v), true);
                }
                c => panic!("Unexpected in int: {:?}", c),
            }
        }
        panic!("Ran out of items!");
    }

    fn parse_list(&mut self) -> Value {
        let mut list = Vec::new();
        while let Some(c) = self.iter.next() {
            match c {
                '[' => {
                    list.push(self.parse_list());

                    // expect a `,` or `]`
                    match self.iter.next() {
                        Some(',') => {} // noop, just eat
                        Some(']') => return Value::List(list),
                        x => panic!("Unexpected after list item: {:?}", x),
                    }
                }
                ']' => return Value::List(list),
                c @ '0'..='9' => {
                    let (i, done) = self.parse_int(c);
                    list.push(i);
                    if done {
                        return Value::List(list);
                    }
                }
                c => panic!("Unexpected after list item: {:?}", c),
            }
        }
        panic!("Ran out of items!");
    }
}

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut idx = 0;
    let lines: Vec<_> = util::read_lines(input).collect();

    let mut right = Vec::new();

    while idx < lines.len() {
        let l = Parser::parse(lines[idx]);
        let r = Parser::parse(lines[idx + 1]);

        // next pair
        idx += 3;

        if l.cmp(&r) == Ordering::Less {
            right.push(idx / 3);
        }
    }

    println!("{}", right.iter().sum::<usize>());

    Ok(())
}

pub fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let mut lines: Vec<_> = util::read_lines(input).filter(|s| !s.is_empty()).collect();
    lines.push("[[2]]");
    lines.push("[[6]]");
    let mut values: Vec<_> = lines.into_iter().map(|l| Parser::parse(l)).collect();
    values.sort();

    let d1 = Parser::parse("[[2]]");
    let d2 = Parser::parse("[[6]]");

    // find the dividers
    let mut di1 = 0;
    let mut di2 = 0;
    for (i, v) in values.iter().enumerate() {
        if *v == d1 {
            di1 = i + 1;
        }
        if *v == d2 {
            di2 = i + 1;
        }
    }
    println!("{}", di1 * di2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v = Parser::parse("[]");
        assert_eq!(v, Value::List(vec![]));
    }

    #[test]
    fn test2() {
        let v = Parser::parse("[1,[1,3],10,1]");
        assert_eq!(
            v,
            Value::List(vec![
                Value::Int(1),
                Value::List(vec![Value::Int(1), Value::Int(3),]),
                Value::Int(10),
                Value::Int(1)
            ])
        );
    }

    fn test_compare(l: &str, r: &str, expected: Ordering) {
        let l = Parser::parse(l);
        let r = Parser::parse(r);
        assert_eq!(expected, l.cmp(&r));
    }

    #[test]
    fn test_compare1() {
        test_compare("[1,1,3,1,1]", "[1,1,5,1,1]", Ordering::Less);
    }

    #[test]
    fn test_compare2() {
        test_compare("[[1],[2,3,4]]", "[[1],4]", Ordering::Less);
    }

    #[test]
    fn test_compare3() {
        test_compare("[9]", "[[8,7,6]]", Ordering::Greater);
    }

    #[test]
    fn test_compare4() {
        test_compare("[[4,4],4,4]", "[[4,4],4,4,4]", Ordering::Less);
    }

    #[test]
    fn test_compare5() {
        test_compare(
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
            Ordering::Greater,
        );
    }
}
