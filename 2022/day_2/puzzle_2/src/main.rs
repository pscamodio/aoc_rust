use aoc_utils::read_input_lines;

#[derive(PartialEq, Clone)]
enum RPS {
    Rock = 0,
    Paper = 1,
    Scissor = 2,
}

fn read_move(play: &str) -> RPS {
    return match play {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissor,
        _ => panic!("Unexpected player 1 play"),
    }
}

fn winner(opponent: &RPS) -> RPS {
    match opponent {
        RPS::Rock => RPS::Paper,
        RPS::Paper => RPS::Scissor,
        RPS::Scissor => RPS::Rock,
    }
}

fn loser(opponent: &RPS) -> RPS {
    match opponent {
        RPS::Rock => RPS::Scissor,
        RPS::Paper => RPS::Rock,
        RPS::Scissor => RPS::Paper,
    }
}

fn compute_best_move(opponent: &RPS, play: &str) -> RPS {
    return match play {
        "X" => loser(opponent),
        "Y" => opponent.clone(),
        "Z" => winner(opponent),
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

    let opponent = read_move(plays.get(0).expect("opponent move missing"));

    let me = compute_best_move(&opponent, plays.get(1).expect("suggestion missing"));

    return rps_match_score(&me, &opponent) + rps_selection_score(&me);
}

fn main() {
    let lines = read_input_lines("./2022/day_2/input.txt");
    let scores : i32 = lines.iter().map(compute_score).sum();

    print!("Scores {scores:?}")
}
