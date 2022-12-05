use std::error::Error;
use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod util;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("../day5.txt")?;

    day5::part2(&input);

    Ok(())
}
