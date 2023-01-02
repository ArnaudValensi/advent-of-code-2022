use sscanf::sscanf;
use std::collections::HashMap;
use std::error::Error;
use std::io;

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

#[derive(Debug, Clone)]
struct Operation {
    key1: String,
    key2: String,
    operand: char,
}

#[derive(Debug, Clone)]
enum Statement {
    Operation(Operation),
    Value(i64),
}

fn get_value(items: &HashMap<String, Statement>, key: &str) -> i64 {
    let item = items.get(key);

    if let Some(Statement::Value(value)) = item {
        return *value;
    } else if let Some(Statement::Operation(op)) = item {
        let value1 = get_value(items, &op.key1);
        let value2 = get_value(items, &op.key2);

        return match op.operand {
            '+' => value1 + value2,
            '-' => value1 - value2,
            '*' => value1 * value2,
            '/' => value1 / value2,
            _ => unreachable!(),
        };
    }

    unreachable!();
}

// Solve the Advent of Code 2022 Day 21 puzzle.
fn main() -> Result<(), Box<dyn Error>> {
    let mut items: HashMap<String, Statement> = stdin_lines()
        .map(|line| {
            let (key, statement) = sscanf!(line, "{}: {}", String, String).unwrap();

            // Try to parse statement as i64.
            if let Ok(value) = statement.parse::<i64>() {
                return (key, Statement::Value(value));
            } else {
                let (key1, operand, key2) =
                    sscanf!(statement, "{} {} {}", String, char, String).unwrap();
                return (
                    key,
                    Statement::Operation(Operation {
                        key1,
                        key2,
                        operand,
                    }),
                );
            }
        })
        .collect();

    let root_value = get_value(&items, "root");

    println!("Part1: {:#?}", root_value);

    let root = items.get("root").cloned().unwrap();
    let op = if let Statement::Operation(op) = root {
        op
    } else {
        unreachable!();
    };

    let mut humn_value = 0;
    let mut increment = 1_000_000_000;
    let mut is_incrementing = true;
    loop {
        items.insert("humn".to_owned(), Statement::Value(humn_value));

        let value1 = get_value(&items, &op.key1);
        let value2 = get_value(&items, &op.key2);
        let sub = value1 - value2;

        if value1 == value2 {
            break;
        }

        // Doing a dichotomic search.
        if is_incrementing {
            if sub > 0 {
                humn_value += increment;
            } else {
                is_incrementing = false;
                increment /= 2;
                humn_value -= increment;
            }
        } else {
            if sub < 0 {
                humn_value -= increment;
            } else {
                is_incrementing = true;
                increment /= 2;
                humn_value += increment;
            }
        }

        humn_value -= 1;
    }

    println!("Part2: {}", humn_value);

    Ok(())
}
