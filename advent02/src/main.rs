use std::{fs::File, io::{BufReader, BufRead, Lines}, fmt::Debug};

fn main() {
    let file = File::open("advent02/input").expect("no such file");
    let buf = BufReader::new(file);
    let total = get_total_score(buf.lines());
    println!("total {}", total);
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

impl RPS {
    fn new_from_opponent(opponent: char) -> RPS {
        match opponent {
            'A' => RPS::Rock,
            'B' => RPS::Paper,
            'C' => RPS::Scissors,
            _ => panic!("wrong char")
        }
    }

    fn new_from_me(instruction: char, opponent: &RPS) -> RPS {
        match instruction {
            'X' => opponent.wins_against(),
            'Y' => opponent.clone(),
            'Z' => opponent.looses_against(),
            _ => panic!("wrong char")
        }
    }

    fn wins(&self, comp: RPS) -> bool {
        match self {
            RPS::Rock => comp == RPS::Scissors,
            RPS::Paper => comp == RPS::Rock,
            RPS::Scissors => comp == RPS::Paper,
        }
    }

    fn score(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn wins_against(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }

    fn looses_against(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }
}

fn get_total_score(lines: Lines<BufReader<File>>) -> u32 {
    let mut score = 0;
    for line in lines {
        let s_line = line.unwrap();
        let mut s_line_chars = s_line.chars();
        let opponent_rps = RPS::new_from_opponent(s_line_chars.next().unwrap());
        let my_rps = RPS::new_from_me(s_line_chars.nth(1).unwrap(), &opponent_rps);
        score += get_score(opponent_rps, my_rps);
    }
    score
}

fn get_score(opponent: RPS, my: RPS) -> u32 {
    if opponent == my {
        my.score() + 3
    } else if my.wins(opponent) {
        my.score() + 6
    } else {
        my.score()
    }
}