use std::error::Error;

use aoc_utils::{day2, read_input_lines};

fn compute_score(line: &String) -> i32 {
    let plays: Vec<&str> = line.split(" ").collect();

    let opponent = day2::read_opponent_move(plays.get(0).expect("Missing opponent move"));

    let me = day2::read_player_move(plays.get(1).expect("Missing player move"));

    day2::match_score(&me, &opponent) + day2::move_score(&me)
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = read_input_lines("2022/day_2/input.txt")?;
    let score: i32 = lines.iter().map(compute_score).sum();

    print!("Scores {score:?}");

    return Ok(());
}
