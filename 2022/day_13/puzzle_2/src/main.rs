use std::{
    error::Error,
    fs
};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("2022/day_12/input.txt")?;

    Ok(())
}
