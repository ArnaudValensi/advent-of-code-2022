use std::error::Error;
use std::fs;

#[derive(PartialEq)]
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
}

enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

fn move_from_letter(letter: &str) -> Move {
    match letter {
        "A" | "X" => Move::Rock,
        "B" | "Y" => Move::Paper,
        "C" | "Z" => Move::Scissors,
        &_ => unreachable!(),
    }
}

fn get_outcome(move1: &Move, move2: &Move) -> Outcome {
    if &move1.win_against() == move2 {
        return Outcome::Lose;
    } else if &move2.win_against() == move1 {
        return Outcome::Win;
    } else {
        return Outcome::Draw;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("./input1.txt")?;
    // let text = fs::read_to_string("./test.txt")?;
    println!("{text}\n");

    let result: Vec<i64> = text
        .split("\n")
        .map(|line| {
            println!("{line}");
            let mut letters = line.split_whitespace();
            let letter1 = letters.next().unwrap();
            let letter2 = letters.next().unwrap();
            let move1 = move_from_letter(letter1);
            let move2 = move_from_letter(letter2);

            let outcome = get_outcome(&move1, &move2);

            let result = move2 as i64 + outcome as i64;

            println!("  {result}\n");

            result
        })
        .collect();

    println!("{result:#?}");

    let res: i64 = result.iter().sum();

    println!("{res:#?}");

    Ok(())
}
