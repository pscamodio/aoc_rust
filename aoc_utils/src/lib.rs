use std::fs;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn read_input_lines(file_path: &str) -> Vec<String> {
    println!("In file {}", file_path);

    // Read the file
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // Split by lines
    return contents.split("\n").map(|x| x.to_string()).collect();
}
