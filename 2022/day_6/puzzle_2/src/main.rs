use std::{error::Error, fs};

fn has_duplicates(s: &str) -> bool {
    for c in s.chars() {
        if s.matches(c).count() > 1 {
            return true;
        }
    }
    return false;
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("2022/day_6/input.txt")?;

    let mut iter = input.chars();

    let mut marker: String = String::new();

    let mut idx = 0;

    loop {
        let next = iter.next().unwrap();

        marker.push(next);

        if marker.len() > 14 {
            marker.drain(0..1);
        }

        if marker.len() == 14 && !has_duplicates(&marker) {
            println!("End of marker: {}", idx + 1);
            break;
        }

        idx += 1;
    }

    Ok(())
}
