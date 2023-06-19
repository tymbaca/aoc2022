use std::{fs::File, io::Read};

/*
A - Rock (1)
B - Paper (2)
C - Scissors (3)

Win = 6
Draw = 3
Lose = 0

---
Opp. / You.
Rock, Scissors --> 3 + 0 = 3
Rock, Paper --> 2 + 6 = 8
 */

struct Match {
    player_choice: Choice,
    opponent_choice: Choice
}

impl Match {
    fn new(match_data: &str) -> Match {
        let opponent_choice = Choice::get_opponent_choice(match_data);
        let match_result = MatchResult::get(match_data);
        let player_choice = Choice::predict_player_choice(opponent_choice, match_result);
        let _match = Match {
            player_choice,
            opponent_choice
        };
        return _match;
    }

    fn score(&self) -> i32 {
        let mut score = 0;
        score += Match::win_score(self.player_choice, self.opponent_choice);
        score += self.player_choice.value();
        return score;
    }

    fn win_score(player_choice: Choice, opponent_choice: Choice) -> i32 {
        match (player_choice, opponent_choice) {
            (Choice::Rock, Choice::Rock) | (Choice::Paper, Choice::Paper) | (Choice::Scissors, Choice::Scissors) => 3,  // Draw
            (Choice::Rock, Choice::Paper) | (Choice::Paper, Choice::Scissors) | (Choice::Scissors, Choice::Rock) => 0, // Lose
            (Choice::Rock, Choice::Scissors) | (Choice::Paper, Choice::Rock) | (Choice::Scissors, Choice::Paper) => 6 // Win
        }
    }
}

#[derive(Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors
}

impl Choice {
    fn value(&self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3
        }
    }

    fn get_opponent_choice(data: &str) -> Choice {
        let raw = get_data_part(data, DataPart::OpponentChoice);
        match raw {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Weird character: {}", raw)
        }
    }

    fn predict_player_choice(opponent_choice: Choice, match_result: MatchResult) -> Choice {
        match opponent_choice {
            Choice::Rock => match match_result {
                MatchResult::Lose => Choice::Scissors,
                MatchResult::Draw => Choice::Rock,
                MatchResult::Win => Choice::Paper,
            }
            Choice::Paper => match match_result {
                MatchResult::Lose => Choice::Rock,
                MatchResult::Draw => Choice::Paper,
                MatchResult::Win => Choice::Scissors,
            }
            Choice::Scissors => match match_result {
                MatchResult::Lose => Choice::Paper,
                MatchResult::Draw => Choice::Scissors,
                MatchResult::Win => Choice::Rock,
            }
        }
    }
}

enum MatchResult {
    Lose,
    Draw,
    Win,
}

impl MatchResult {
    fn get(match_data: &str) -> Self {
        let raw = get_data_part(match_data, DataPart::MatchResult);
        match raw {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Weird character: {}", raw)
        }
    }
}

enum DataPart {
    OpponentChoice,
    MatchResult
}

fn get_data_part(data: &str, part: DataPart) -> &str {
    let data: Vec<&str> = data.split(" ").collect();
    match part {
        DataPart::OpponentChoice => {
            return data[0];
        },
        DataPart::MatchResult => {
            return data[1];
        }
    }
}

fn main() {
    let mut file_content = String::new();
    let mut file = File::open("./src/input.txt").unwrap();
    file.read_to_string(&mut file_content).unwrap();
    let matches = file_content.lines();

    let mut score = 0;
    for match_data in matches {
        let _match = Match::new(match_data);
        score += _match.score()
    }
    println!("{}", score);
}