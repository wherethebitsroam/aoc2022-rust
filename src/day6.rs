extern crate test;

use std::{collections::HashSet, error::Error};

fn distinct(w: &[char]) -> bool {
    let mut hs = HashSet::new();
    for c in w {
        if !hs.insert(*c) {
            return false;
        }
    }
    true
}

pub fn find_repeat(s: &str, size: usize) -> Option<usize> {
    let ch: Vec<_> = s.chars().collect();
    for (i, w) in ch.windows(size).enumerate() {
        if distinct(w) {
            return Some(i + size);
        }
    }

    None
}

pub fn find_repeat2(b: &[u8], size: usize) -> Option<usize> {
    let mut tracker = [0u8; 256];
    let mut dups = 0;

    for i in 0..b.len() {
        // handle the incoming char
        let c = b[i] as usize;
        if tracker[c] == 1 {
            // we are about to add a duplicate
            dups += 1;
        }
        tracker[c] += 1;

        // we need to have read at least size before checking
        if i >= size {
            // handle outgoing char
            let o = b[i - size] as usize;
            if tracker[o] == 2 {
                // we are about to remove a duplicate
                dups -= 1;
            }
            tracker[o] -= 1;

            // check if we have duplicates
            if dups == 0 {
                return Some(i + 1);
            }
        }
    }

    None
}

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let x = find_repeat2(input.as_bytes(), 4);
    println!("x: {:?}", x);
    Ok(())
}

pub fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let x = find_repeat2(input.as_bytes(), 14);
    println!("x: {:?}", x);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use test::Bencher;

    #[test]
    fn test() {
        assert_eq!(find_repeat("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), Some(5));
    }

    #[test]
    fn test2() {
        assert_eq!(
            find_repeat2("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes(), 4),
            Some(5)
        );
    }

    #[bench]
    fn bench_find_repeat_4(b: &mut Bencher) {
        let input = fs::read_to_string("../day6.txt").unwrap();
        b.iter(|| find_repeat(&input, 4))
    }

    #[bench]
    fn bench_find_repeat_8(b: &mut Bencher) {
        let input = fs::read_to_string("../day6.txt").unwrap();
        b.iter(|| find_repeat(&input, 18))
    }

    #[bench]
    fn bench_find_repeat_16(b: &mut Bencher) {
        let input = fs::read_to_string("../day6.txt").unwrap();
        b.iter(|| find_repeat(&input, 16))
    }

    #[bench]
    fn bench_find_repeat2_4(b: &mut Bencher) {
        let input = fs::read_to_string("../day6.txt").unwrap();
        let bytes = input.as_bytes();
        b.iter(|| find_repeat2(bytes, 4))
    }

    #[bench]
    fn bench_find_repeat2_8(b: &mut Bencher) {
        let input = fs::read_to_string("../day6.txt").unwrap();
        let bytes = input.as_bytes();
        b.iter(|| find_repeat2(bytes, 8))
    }

    #[bench]
    fn bench_find_repeat2_16(b: &mut Bencher) {
        let input = fs::read_to_string("../day6.txt").unwrap();
        let bytes = input.as_bytes();
        b.iter(|| find_repeat2(bytes, 16))
    }
}
