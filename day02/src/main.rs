use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

const ROCK_SCORE: i32 = 1;
const PAPER_SCORE: i32 = 2;
const SCISSORS_SCORE: i32 = 3;
const LOSS_SCORE: i32 = 0;
const DRAW_SCORE: i32 = 3;
const WIN_SCORE: i32 = 6;

enum Moves {
    Rock,
    Paper,
    Scissors
}
enum Results {
    Loss,
    Draw,
    Win,
}

fn main() {
    let mut total_score = 0;
    if let Ok(guide) = read_lines("input") {
        for line in guide {
            if let Ok(strategy) = line {
                total_score += score_game(read_strategy(&strategy).unwrap());
            }
        }
    }
    println!("{}", total_score);

}

fn read_strategy(strategy: &str) -> Option<(Moves, Results)> {
    let mut strategy_iter = strategy.chars();
    let mut moves = (Moves::Rock, Results::Loss);
    if let Some(move_choice) = strategy_iter.next() { 
        if move_choice == 'A' {
            moves.0 = Moves::Rock;
        } else if move_choice == 'B' {
            moves.0 = Moves::Paper;
        } else if move_choice == 'C' {
            moves.0 = Moves::Scissors;
        } else {
            return None;
        }
    } else {
        return None;
    }
    strategy_iter.next();
    if let Some(move_choice) = strategy_iter.next() {
        if move_choice == 'X' {
            moves.1 = Results::Loss;
        } else if move_choice == 'Y' {
            moves.1 = Results::Draw;
        } else if move_choice == 'Z' {
            moves.1 = Results::Win;
        } else {
            return None;
        }
    } else {
        return None;
    }
    return Some(moves);
    
}

fn score_game((opponent, your): (Moves, Results)) -> i32 {
    match (opponent, your) {
        (Moves::Rock, Results::Loss) => LOSS_SCORE + SCISSORS_SCORE,
        (Moves::Rock, Results::Draw) => DRAW_SCORE + ROCK_SCORE,
        (Moves::Rock, Results::Win) => WIN_SCORE + PAPER_SCORE,
        (Moves::Paper, Results::Loss) => LOSS_SCORE + ROCK_SCORE,
        (Moves::Paper, Results::Draw) => DRAW_SCORE + PAPER_SCORE,
        (Moves::Paper, Results::Win) => WIN_SCORE + SCISSORS_SCORE,
        (Moves::Scissors, Results::Loss) => LOSS_SCORE + PAPER_SCORE,
        (Moves::Scissors, Results::Draw) => DRAW_SCORE + SCISSORS_SCORE,
        (Moves::Scissors, Results::Win) => WIN_SCORE + ROCK_SCORE,
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}