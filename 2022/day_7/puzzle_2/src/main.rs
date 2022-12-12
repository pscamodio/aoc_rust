use std::{cell::RefCell, error::Error, fs, rc::Rc};

use aoc_utils::day7;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("2022/day_7/input.txt")?;

    let root = day7::parse_fs(input);

    root.borrow_mut().update_size();

    const TOTAL_SPACE: u32 = 70000000;
    const REQUIRED: u32 = 30000000;
    let used_space = root.borrow().size;
    let free_space = TOTAL_SPACE - used_space;
    let to_free = REQUIRED - free_space;

    println!("total {TOTAL_SPACE}");
    println!("used {used_space}");
    println!("to free {to_free}");

    let mut candidates: Vec<Rc<RefCell<day7::FSEntry>>> = Vec::new();

    day7::find_dir_rec(&root, Box::new(move |sz| sz > to_free), &mut candidates);

    candidates.sort_by(|a, b| a.borrow().size.cmp(&b.borrow().size));

    let first = candidates.first().unwrap().borrow();

    println!("{} {}", first.name, first.size);

    Ok(())
}
