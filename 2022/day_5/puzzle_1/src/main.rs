use std::{error::Error, fs};

use aoc_utils::day5;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("2022/day_5/input.txt")?;

    // Get an iterator on the file lines
    let mut lines = input.lines();

    // Consume the first part to read the initial stacks state
    let mut stacks = day5::parse_initial_stacks(lines.by_ref());

    // Consume the rest to read the movements
    let movements = day5::parse_movements(lines);

    // Apply the movements to the stacks
    day5::apply_movements_9000(&mut stacks, &movements);

    // Get the top crate for all the stacks
    let top = stacks.map(|x| x.last().unwrap().clone());

    print!("Top: {top:?}");

    Ok(())
}
