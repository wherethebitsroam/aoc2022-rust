#![feature(test)]
#![allow(dead_code)]

use std::error::Error;
use std::fs;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod util;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("../day12.txt")?;

    day12::part2(&input);

    Ok(())
}
