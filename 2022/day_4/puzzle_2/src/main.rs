use std::error::Error;

use aoc_utils::{day4::parse_sections, read_input_lines};

fn need_replan(line: &String) -> bool {
    let (left, right) = parse_sections(line);

    left.overlap(&right)
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = read_input_lines("2022/day_4/input.txt")?;

    let to_replan = lines.iter().filter(|x| need_replan(x)).count();

    dbg!(to_replan);

    Ok(())
}
