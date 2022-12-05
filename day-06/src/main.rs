use itertools::Itertools;
use std::error::Error;
use std::io;

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = stdin_lines().next().unwrap().chars().collect::<Vec<char>>();

    for (i, chars) in input.windows(4).enumerate() {
        if chars.iter().unique().count() == 4 {
            let result = i + 4;
            println!("part1: {result}");
            break;
        }
    }

    for (i, chars) in input.windows(14).enumerate() {
        if chars.iter().unique().count() == 14 {
            let result = i + 14;
            println!("part2: {result}");
            return Ok(());
        }
    }

    Ok(())
}
