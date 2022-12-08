use std::error::Error;

use crate::util;

static test_input: &str = "30373
25512
65332
33549
35390";

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    util::read_lines(input)
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let grid = parse_input(input);

    // minus 4 because all corners are counted twice
    let mut visible = 2 * grid.len() + 2 * grid[0].len() - 4;

    // take one from the lengths because the edge is visible
    for r in 1..grid.len() - 1 {
        let row = &grid[r];
        for c in 1..row.len() - 1 {
            let h = row[c];

            // get the visibility lines
            let left = *row[..c].iter().max().unwrap();
            let right = *row[c + 1..].iter().max().unwrap();
            let up = grid[..r].iter().map(|x| x[c]).max().unwrap();
            let down = grid[r + 1..].iter().map(|x| x[c]).max().unwrap();

            if h > left || h > right || h > up || h > down {
                visible += 1;
            }
        }
    }
    println!("visible: {}", visible);
    Ok(())
}

fn count_trees(h: u8, trees: impl Iterator<Item = u8>) -> u32 {
    let mut count = 0;
    for t in trees {
        count += 1;
        if t >= h {
            return count;
        }
    }
    count
}

fn tree_count(grid: &Vec<Vec<u8>>, r: usize, c: usize) -> [u32; 4] {
    let row = &grid[r];
    let h = row[c];

    // get the visibility lines
    let left = count_trees(h, row[..c].iter().rev().map(|&x| x));
    let right = if c == row.len() {
        0
    } else {
        count_trees(h, row[c + 1..].iter().map(|&x| x))
    };
    let up = count_trees(h, grid[..r].iter().map(|x| x[c]).rev());
    let down = if r == grid.len() {
        0
    } else {
        count_trees(h, grid[r + 1..].iter().map(|x| x[c]))
    };

    [left, right, up, down]
}

pub fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let grid = parse_input(input);

    let mut max_score = 0;

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let x = tree_count(&grid, r, c);

            let score: u32 = x.iter().product();
            if score > max_score {
                max_score = score
            }
        }
    }
    println!("max_score: {}", max_score);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, 1);
    }
}
