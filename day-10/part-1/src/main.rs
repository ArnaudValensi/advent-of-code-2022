use sscanf::sscanf;
use std::error::Error;
use std::io;

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

enum Instruction {
    AddX(i64),
    Noop,
}

fn get_next_instruction(lines: &mut impl Iterator<Item = String>) -> Option<Instruction> {
    let line = lines.next()?;

    if line == "noop" {
        return Some(Instruction::Noop);
    }

    if let Ok((_instruction, argument)) = sscanf!(line, "{} {}", String, i64) {
        return Some(Instruction::AddX(argument));
    }

    panic!("Invalid instruction: {}", line);
}

// Solve the Advent of Code 2022 Day 10 puzzle.
fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = stdin_lines();
    let mut x = 1;
    let mut pending_instruction: Option<Instruction> = None;
    let mut signal_strength = 0;

    for cycle in 1..=220 {
        if vec![20, 60, 100, 140, 180, 220].contains(&cycle) {
            let new_strength = cycle * x;
            signal_strength += new_strength;
            println!(
                "Cycle {}: x = {}, signal strength = {}, total strength = {}",
                cycle, x, new_strength, signal_strength
            );
        }

        if let Some(ref instruction) = pending_instruction {
            match instruction {
                Instruction::AddX(argument) => {
                    x += argument;
                    println!("Cycle {}, addx {} done, x: {}", cycle, argument, x);
                }
                Instruction::Noop => (),
            }
            pending_instruction = None;
            continue;
        }

        if let Some(instruction) = get_next_instruction(&mut lines) {
            match instruction {
                Instruction::AddX(argument) => {
                    pending_instruction = Some(Instruction::AddX(argument));
                    println!("Cycle {}, addx {}", cycle, argument);
                }
                Instruction::Noop => {
                    println!("Cycle {}, noop", cycle);
                }
            }
        } else {
            println!("Cycle {}, end of input", cycle);
            break;
        }
    }

    println!("Final signal strength: {}", signal_strength);

    Ok(())
}
