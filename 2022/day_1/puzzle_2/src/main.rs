use std::error::Error;

use aoc_utils::{day1, read_input_lines};

fn main() -> Result<(), Box<dyn Error>> {
    // Split by lines
    let lines = read_input_lines("2022/day_1/input.txt")?;

    let mut snacks = day1::compute_elf_snack_weights(&lines);

    // Sort the array
    snacks.sort_unstable_by(|a, b| b.cmp(a));

    // Pick top three
    let best3 = &snacks[0..3];

    // Sum them
    let best: i32 = Vec::from(best3).iter().sum();

    println!("With text:\n{best:?}");

    return Ok(());
}
