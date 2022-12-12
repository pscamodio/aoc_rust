use std::error::Error;

use aoc_utils::{day1, read_input_lines};

fn main() -> Result<(), Box<dyn Error>> {
    // Split by lines
    let lines = read_input_lines("2022/day_1/input.txt")?;

    let snacks = day1::compute_elf_snack_weights(&lines);

    // Compute max value of array
    let max = snacks.iter().max().expect("Should have a max");

    println!("With text:\n{max:?}");

    return Ok(());
}
