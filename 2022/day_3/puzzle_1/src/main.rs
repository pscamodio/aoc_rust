use std::error::Error;

use aoc_utils::{day3, read_input_lines};

fn split_compartments(line: &String) -> (&str, &str) {
    let length = line.len();
    (&line[..length / 2], &line[length / 2..])
}

fn same_items(left: &str, right: &str) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for char in left.as_bytes() {
        if right.as_bytes().contains(char) && !res.contains(char) {
            res.push(char.clone());
        }
    }
    return res;
}

fn compute_priority(line: &String) -> i32 {
    let (left, right) = split_compartments(&line);

    let same = same_items(left, right);

    same.iter().map(day3::object_priority).sum::<i32>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = read_input_lines("2022/day_3/input.txt")?;

    let val: i32 = lines.iter().map(compute_priority).sum();

    dbg!(val);

    Ok(())
}
