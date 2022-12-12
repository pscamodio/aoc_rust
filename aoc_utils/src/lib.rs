use std::{fs, io};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day7;

/// Read a files and return a vector of string one for line
/// # Arguments
/// * 'file_path' - Path to the input file from the project root
pub fn read_input_lines(file_path: &str) -> io::Result<Vec<String>> {
    println!("In file {}", file_path);

    // Read the file
    let contents = fs::read_to_string(file_path)?;

    // Split by lines
    return Ok(contents.split("\n").map(|x| x.to_string()).collect());
}
