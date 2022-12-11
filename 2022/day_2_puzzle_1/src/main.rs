use std::{fs};

fn read_input_lines() -> Vec<String> {

    let file_path = "./input.txt";
    // --snip--
    println!("In file {}", file_path);

    // Read the file
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // Split by lines
    return contents.split("\n").map(|x| x.to_string()).collect();
}

#[derive(PartialEq)]
enum RPS {
    Rock = 0,
    Paper = 1,
    Scissor = 2,
}

fn parse_player_1(play: &str) -> RPS {
    return match play {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissor,
        _ => panic!("Unexpected player 1 play"),
    }
}

fn parse_player_2(play: &str) -> RPS {
    return match play {
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissor,
        _ => panic!("Unexpected player 2 play"),
    }
}

fn rps_selection_score(rps: &RPS) -> i32{
    return match rps {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissor => 3
    };
}

fn rps_match_score(rps1: &RPS, rps2: &RPS) -> i32 {
    match (rps1, rps2) {
        (RPS::Rock, RPS::Rock) => 3,
        (RPS::Rock, RPS::Paper) => 0,
        (RPS::Rock, RPS::Scissor) => 6,
        (RPS::Paper, RPS::Rock) => 6,
        (RPS::Paper, RPS::Paper) => 3,
        (RPS::Paper, RPS::Scissor) => 0,
        (RPS::Scissor, RPS::Rock) => 0,
        (RPS::Scissor, RPS::Paper) => 6,
        (RPS::Scissor, RPS::Scissor) => 3,
    }
}

fn compute_score(line: &String) -> i32 {
    let plays : Vec<&str> = line.split(" ").collect();

    let opponent = match plays.get(0) {
        Some(x) => parse_player_1(x),
        _ => panic!("Player 1 play missing"),
    };
    let me = match plays.get(1) {
        Some(x) => parse_player_2(x),
        _ => panic!("Player 1 play missing"),
    };

    return rps_match_score(&me, &opponent) + rps_selection_score(&me);

}

fn main() {
    let lines = read_input_lines();
    let scores : i32 = lines.iter().map(compute_score).sum();

    print!("Scores {scores:?}")
}
