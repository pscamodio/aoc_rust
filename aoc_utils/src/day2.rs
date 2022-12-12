/// Possible picks for Rock Paper Scissor
#[derive(PartialEq, Clone)]
pub enum RPS {
    Rock = 0,
    Paper = 1,
    Scissor = 2,
}

/// Read an opponent move from the input file
pub fn read_opponent_move(play: &str) -> RPS {
    return match play {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissor,
        _ => panic!("Unexpected player 1 play"),
    };
}

/// Read a player move from the input file
pub fn read_player_move(play: &str) -> RPS {
    return match play {
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissor,
        _ => panic!("Unexpected player 1 play"),
    };
}

/// Compute the winner move knowing the opponent move
pub fn winner_move(opponent: &RPS) -> RPS {
    match opponent {
        RPS::Rock => RPS::Paper,
        RPS::Paper => RPS::Scissor,
        RPS::Scissor => RPS::Rock,
    }
}

/// Compute the losing move knowing the opponent move
pub fn losing_move(opponent: &RPS) -> RPS {
    match opponent {
        RPS::Rock => RPS::Scissor,
        RPS::Paper => RPS::Rock,
        RPS::Scissor => RPS::Paper,
    }
}

/// Compute the best move following the playbook
pub fn compute_playbook_move(opponent: &RPS, playbook_suggestion: &str) -> RPS {
    return match playbook_suggestion {
        "X" => losing_move(opponent),
        "Y" => opponent.clone(),
        "Z" => winner_move(opponent),
        _ => panic!("Unexpected player 2 play"),
    };
}

/// Compute the score depending on the selected move
pub fn move_score(rps: &RPS) -> i32 {
    return match rps {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissor => 3,
    };
}

/// Compute the score checking if we win or lose
pub fn match_score(player: &RPS, opponent: &RPS) -> i32 {
    match (player, opponent) {
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
