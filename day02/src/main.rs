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

fn read_strategy(strategy: &str) -> Option<(Moves, Moves)> {
    let mut strategy_iter = strategy.chars();
    let mut moves = (Moves::Rock, Moves::Rock);
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
            moves.1 = Moves::Rock;
        } else if move_choice == 'Y' {
            moves.1 = Moves::Paper;
        } else if move_choice == 'Z' {
            moves.1 = Moves::Scissors;
        } else {
            return None;
        }
    } else {
        return None;
    }
    return Some(moves);
    
}

fn score_game((opponent, your): (Moves, Moves)) -> i32 {
    match your {
        Moves::Rock => ROCK_SCORE + match opponent {
            Moves::Rock => DRAW_SCORE,
            Moves::Paper => LOSS_SCORE,
            Moves::Scissors => WIN_SCORE,
        },
        Moves::Paper => PAPER_SCORE + match opponent {
            Moves::Rock => WIN_SCORE,
            Moves::Paper => DRAW_SCORE,
            Moves::Scissors => LOSS_SCORE,
            
        },
        Moves::Scissors =>  SCISSORS_SCORE + match opponent {
            Moves::Rock => LOSS_SCORE,
            Moves::Paper => WIN_SCORE,
            Moves::Scissors => DRAW_SCORE,
        },
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}