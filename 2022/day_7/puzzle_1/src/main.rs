use std::{cell::RefCell, error::Error, fs, rc::Rc};

use aoc_utils::day7;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("2022/day_7/input.txt")?;

    let root = day7::parse_fs(input);

    root.borrow_mut().update_size();

    let mut candidates: Vec<Rc<RefCell<day7::FSEntry>>> = Vec::new();

    day7::find_dir_rec(&root, Box::new(|sz: u32| sz < 100000), &mut candidates);

    let sum: u32 = candidates.iter().map(|x| x.borrow().size).sum();

    println!("sum {sum}");

    Ok(())
}
