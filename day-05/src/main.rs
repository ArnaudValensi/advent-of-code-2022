use sscanf::sscanf;
use std::error::Error;
use std::io;

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stack_lines: Vec<String> = Vec::new();
    let mut lines = stdin_lines();

    loop {
        let line = lines.next().unwrap();

        if line.chars().nth(1).unwrap() == '1' {
            break;
        }

        stack_lines.push(line);
    }

    let num_stacks = (stack_lines[0].len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(num_stacks);
    for _ in 0..num_stacks {
        stacks.push(Vec::new())
    }

    for stack_line in stack_lines.iter().rev() {
        for (n, char) in stack_line.chars().skip(1).step_by(4).enumerate() {
            if char != ' ' {
                stacks[n].push(char);
            }
        }
    }

    // Skip the empty line;
    lines.next();

    let mut stacks2 = stacks.clone();

    for line in lines {
        let (amount, from, to) = sscanf!(line, "move {usize} from {usize} to {usize}").unwrap();

        // Part 1.
        for _ in 0..amount {
            let item = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(item);
        }

        // Part 2.
        let from_stack = &mut stacks2[from - 1];
        let mut items = from_stack.split_off(from_stack.len() - amount);
        stacks2[to - 1].append(&mut items);
    }

    let part1: String = stacks.iter().fold(String::new(), |mut acc, value| {
        acc.push(*value.last().unwrap());
        acc
    });
    let part2: String = stacks2.iter().fold(String::new(), |mut acc, value| {
        acc.push(*value.last().unwrap());
        acc
    });

    println!("part1: {part1}");
    println!("part2: {part2}");

    Ok(())
}
