use std::error::Error;
use std::io;

#[derive(PartialEq, Copy, Clone, Debug)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    fn win_against(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn lose_against(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

fn move_from_letter(letter: &str) -> Move {
    match letter {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        &_ => unreachable!(),
    }
}

fn outcome_from_letter(letter: &str) -> Outcome {
    match letter {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        &_ => unreachable!(),
    }
}

fn get_move_for_outcome(current_move: &Move, outcome: &Outcome) -> Move {
    match outcome {
        Outcome::Lose => current_move.win_against(),
        Outcome::Draw => *current_move,
        Outcome::Win => current_move.lose_against(),
    }
}

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

fn main() -> Result<(), Box<dyn Error>> {
    let result: i64 = stdin_lines()
        .map(|line| {
            let mut letters = line.split_whitespace();
            let letter1 = letters.next().unwrap();
            let letter2 = letters.next().unwrap();
            let move1 = move_from_letter(letter1);
            let outcome = outcome_from_letter(letter2);
            let move2 = get_move_for_outcome(&move1, &outcome);

            move2 as i64 + outcome as i64
        })
        .sum();

    println!("{result}");

    Ok(())
}
