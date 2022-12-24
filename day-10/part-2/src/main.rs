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

fn draw_crt(crt: &Vec<Vec<char>>) {
    for line in crt {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

// Solve the Advent of Code 2022 Day 10 puzzle.
fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = stdin_lines();
    let mut x: i64 = 1;
    let mut pending_instruction: Option<Instruction> = None;
    let mut signal_strength = 0;
    let mut crt: Vec<Vec<char>> = vec![vec![' '; 40]; 6];

    for cycle in 0..240 {
        let current_line = cycle / 40;
        let current_column = cycle % 40;

        if current_column >= x - 1 && current_column <= x + 1 {
            crt[current_line as usize][current_column as usize] = '#';
        } else {
            crt[current_line as usize][current_column as usize] = '.';
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
        } else {
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

        draw_crt(&crt);
    }

    Ok(())
}
