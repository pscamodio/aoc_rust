use std::{error::Error, fs};

use aoc_utils::day8;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("2022/day_8/input.txt")?;

    let matrix = day8::parse(&input);

    let test = matrix.to_string();

    assert_eq!(input, test);

    let visible = matrix
        .iter()
        .filter(|(_, row, col)| matrix.is_visible(*row, *col))
        .count();

    println!("Visible {visible}");

    Ok(())
}
