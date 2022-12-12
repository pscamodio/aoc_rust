use std::error::Error;

use aoc_utils::{day3, read_input_lines};

fn find_badge(team: &[&str; 3]) -> u8 {
    let first = team[0].as_bytes();
    let second = team[1].as_bytes();
    let third = team[2].as_bytes();

    for char in first {
        if second.contains(char) && third.contains(char) {
            return *char;
        }
    }
    panic!("Each team should have a badge");
}

fn compute_priority(teams: &[&str; 3]) -> i32 {
    day3::object_priority(&find_badge(teams))
}

fn split_by_teams(lines: &Vec<String>) -> Vec<[&str; 3]> {
    let mut teams: Vec<[&str; 3]> = Vec::new();

    let mut iter = lines.iter();

    loop {
        let first = match iter.next() {
            Some(x) => x,
            None => break,
        };

        teams.push([
            first,
            iter.next().expect("Groups are by three"),
            iter.next().expect("Groups are by three"),
        ]);
    }

    teams
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = read_input_lines("2022/day_3/input.txt")?;

    let teams = split_by_teams(&lines);

    let priority: i32 = teams.iter().map(compute_priority).sum();

    dbg!(priority);

    Ok(())
}
