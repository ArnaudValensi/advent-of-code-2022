use itertools::Itertools;
use sscanf::sscanf;
use std::error::Error;
use std::io;

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug)]
enum Operand {
    Old,
    Value(i64),
}

#[derive(Debug)]
struct Monkey {
    id: i64,
    items: Vec<i64>,
    operator: Operator,
    left_operand: Operand,
    right_operand: Operand,
    test_divisor: i64,
    true_monkey_id: i64,
    false_monkey_id: i64,
    inspection_times: i64,
}

fn get_next_line_debug(lines: &mut impl Iterator<Item = String>) -> Option<String> {
    if let Some(line) = lines.next() {
        println!("line: {}", line);
        Some(line)
    } else {
        None
    }
}

fn parse_monkeys(lines: &mut impl Iterator<Item = String>) -> Result<Vec<Monkey>, Box<dyn Error>> {
    let mut monkeys = Vec::new();

    loop {
        let line1_opt = get_next_line_debug(lines);
        if line1_opt.is_none() {
            break;
        }
        let line1 = line1_opt.unwrap();

        let monkey_id = sscanf!(line1, "Monkey {}:", i64).unwrap();

        let line2 = get_next_line_debug(lines).unwrap();
        let items_string = sscanf!(line2, "  Starting items: {}", String).unwrap();
        let items = items_string
            .split(", ")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        let line3 = get_next_line_debug(lines).unwrap();
        let (left_str, operator_str, right_str) =
            sscanf!(line3, "  Operation: new = {} {} {}", String, String, String).unwrap();
        let left_operand = match left_str.as_str() {
            "old" => Operand::Old,
            _ => Operand::Value(left_str.parse::<i64>().unwrap()),
        };
        let right_operand = match right_str.as_str() {
            "old" => Operand::Old,
            _ => Operand::Value(right_str.parse::<i64>().unwrap()),
        };
        let operator = match operator_str.as_str() {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => panic!("Unknown operator: {}", operator_str),
        };

        let line4 = get_next_line_debug(lines).unwrap();
        let test_divisor = sscanf!(line4, "  Test: divisible by {}", i64).unwrap();

        let line5 = get_next_line_debug(lines).unwrap();
        let true_monkey_id = sscanf!(line5, "    If true: throw to monkey {}", i64).unwrap();

        let line6 = get_next_line_debug(lines).unwrap();
        let false_monkey_id = sscanf!(line6, "    If false: throw to monkey {}", i64).unwrap();

        let monkey = Monkey {
            id: monkey_id,
            items,
            operator,
            left_operand,
            right_operand,
            test_divisor,
            true_monkey_id,
            false_monkey_id,
            inspection_times: 0,
        };

        monkeys.push(monkey);

        lines.next();
    }
    Ok(monkeys)
}

fn compute_worry_level(
    item: i64,
    operator: &Operator,
    left_operand: &Operand,
    right_operand: &Operand,
) -> i64 {
    let left = match left_operand {
        Operand::Old => item,
        Operand::Value(v) => *v,
    };
    let right = match right_operand {
        Operand::Old => item,
        Operand::Value(v) => *v,
    };
    match operator {
        Operator::Add => left + right,
        Operator::Multiply => left * right,
    }
}

// Solve the Advent of Code 2022 Day 11 puzzle.
fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = stdin_lines();
    let mut monkeys = parse_monkeys(&mut lines)?;

    println!("Monkeys: {:#?}", monkeys);

    for round in 0..20 {
        println!("Round {}", round);

        for monkey_index in 0..monkeys.len() {
            loop {
                let (new_worry_level, test_divisor, true_monkey_id, false_monkey_id) = {
                    let monkey = &mut monkeys[monkey_index];
                    if monkey.items.is_empty() {
                        break;
                    }

                    let item = monkey.items.remove(0);

                    // Inspect.
                    monkey.inspection_times += 1;

                    // Update worry level.
                    // Worry level divided by 3.
                    let new_worry_level = compute_worry_level(
                        item,
                        &monkey.operator,
                        &monkey.left_operand,
                        &monkey.right_operand,
                    ) / 3;

                    let test_divisor = monkey.test_divisor;
                    let true_monkey_id = monkey.true_monkey_id as usize;
                    let false_monkey_id = monkey.false_monkey_id as usize;

                    (
                        new_worry_level,
                        test_divisor,
                        true_monkey_id,
                        false_monkey_id,
                    )
                };

                // Check if worry level divisible by monkey's test_divisor.
                // Throw item to other monkey.
                if new_worry_level % test_divisor == 0 {
                    // If true, throw to monkey.true_monkey_id.
                    monkeys[true_monkey_id].items.push(new_worry_level);
                } else {
                    // If false, throw to monkey.false_monkey_id.
                    monkeys[false_monkey_id].items.push(new_worry_level);
                }
            }
        }
    }

    println!("Monkeys: {:#?}", monkeys);

    let product_of_two_highest_inspection_times = monkeys
        .iter()
        .map(|m| m.inspection_times)
        .sorted()
        .rev()
        .take(2)
        .product::<i64>();

    println!(
        "Product of two highest inspection times: {}",
        product_of_two_highest_inspection_times
    );

    Ok(())
}
