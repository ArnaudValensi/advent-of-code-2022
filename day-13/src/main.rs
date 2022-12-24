use std::error::Error;
use std::io;

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

#[derive(Debug, Clone)]
enum Value {
    Number(i64),
    List(Vec<Value>),
}

// The input parameter looks like this: "[1,[],[2,[3,[4,[5,6,7]]]],8,9]"
fn parse_list(input: &String) -> Value {
    let mut chars = input.chars().collect::<Vec<char>>();
    chars.pop();
    chars.remove(0);
    let mut values = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if c == '[' {
            let mut depth = 1;
            let mut j = i + 1;
            while depth > 0 {
                if chars[j] == '[' {
                    depth += 1;
                } else if chars[j] == ']' {
                    depth -= 1;
                }
                j += 1;
            }
            let sublist = chars[i..j].iter().collect::<String>();
            values.push(parse_list(&sublist));
            i = j;
        } else if c == ',' {
            i += 1;
        } else {
            let mut j = i + 1;
            while j < chars.len() && chars[j] != ',' {
                j += 1;
            }
            let number = chars[i..j].iter().collect::<String>();
            values.push(Value::Number(number.parse::<i64>().unwrap()));
            i = j;
        }
    }

    Value::List(values)
}

fn compare_lists(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => left < right,
        (Value::List(left), Value::List(right)) => {
            for (left, right) in left.iter().zip(right.iter()) {
                let (left, right) = match (left, right) {
                    (Value::Number(left), Value::List(right)) => (
                        Value::List(vec![Value::Number(*left)]),
                        Value::List((*right).clone()),
                    ),
                    (Value::List(left), Value::Number(right)) => (
                        Value::List((*left).clone()),
                        Value::List(vec![Value::Number(*right)]),
                    ),
                    (left, right) => ((*left).clone(), (*right).clone()),
                };

                if compare_lists(&left, &right) {
                    return true;
                } else if compare_lists(&right, &left) {
                    return false;
                }
            }
            left.len() < right.len()
        }
        _ => false,
    }
}

// Solve the Advent of Code 2022 Day 13 puzzle.
fn main() -> Result<(), Box<dyn Error>> {
    let input = stdin_lines().collect::<Vec<String>>();
    let mut current_pair = 0;
    let mut result = 0;

    let mut lines = input.iter();

    loop {
        current_pair += 1;

        let packet1_str = lines.next().unwrap();
        let packet2_str = lines.next().unwrap();

        let list1 = parse_list(&packet1_str);
        let list2 = parse_list(&packet2_str);

        if compare_lists(&list1, &list2) {
            result += current_pair;
        }

        if lines.next().is_none() {
            break;
        }
    }

    println!("\nPart 1: {}", result);

    let mut packets = input
        .clone()
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();

    packets.push("[[2]]".to_string());
    packets.push("[[6]]".to_string());

    packets.sort_by(|a, b| {
        let list1 = parse_list(&a);
        let list2 = parse_list(&b);
        if compare_lists(&list1, &list2) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    println!("{:#?}", packets);

    // Find the index of the packets with the value "[[2]]" and "[[6]]"
    let mut index1 = 0;
    let mut index2 = 0;
    for (i, packet) in packets.iter().enumerate() {
        if packet == "[[2]]" {
            index1 = i + 1;
        } else if packet == "[[6]]" {
            index2 = i + 1;
        }
    }

    println!("\nPart 2: {}", index1 * index2);

    Ok(())
}
