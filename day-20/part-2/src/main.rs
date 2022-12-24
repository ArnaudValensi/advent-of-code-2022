use sscanf::sscanf;
use std::error::Error;
use std::io;

const DECRYPTION_KEY: i64 = 811589153;

#[derive(Debug, Clone, Copy)]
struct Item {
    id: i64,
    value: i64,
}

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

fn mod_floor(a: i64, base: i64) -> i64 {
    ((a % base) + base) % base
}

#[test]
fn test_mod_floor() {
    assert_eq!(mod_floor(0, 3), 0);
    assert_eq!(mod_floor(1, 3), 1);
    assert_eq!(mod_floor(2, 3), 2);
    assert_eq!(mod_floor(3, 3), 0);
    assert_eq!(mod_floor(4, 3), 1);
    assert_eq!(mod_floor(5, 3), 2);
    assert_eq!(mod_floor(-1, 3), 2);
    assert_eq!(mod_floor(-2, 3), 1);
    assert_eq!(mod_floor(-3, 3), 0);
    assert_eq!(mod_floor(-4, 3), 2);
    assert_eq!(mod_floor(-5, 3), 1);
}

// Solve the Advent of Code 2022 Day 20 puzzle.
fn main() -> Result<(), Box<dyn Error>> {
    let numbers: Vec<i64> = stdin_lines()
        .map(|line| sscanf!(line, "{i64}"))
        .collect::<Result<Vec<_>, _>>()?;

    let list1 = numbers
        .iter()
        .enumerate()
        .map(|(i, &n)| Item {
            id: i as i64,
            value: n * DECRYPTION_KEY,
        })
        .collect::<Vec<_>>();

    let mut list2 = list1.clone();
    let list2_len = list2.len();

    let mut line = String::new();
    for item in list2.iter() {
        line.push_str(&format!("{} ", item.value));
    }
    println!("initial: {}", line);

    for _ in 0..10 {
        for item in list1.iter() {
            let index = list2.iter().position(|x| x.id == item.id).unwrap();

            let value: i64 = item.value % (list2_len as i64 - 1);
            let abs_value: i64 = i64::abs(value);
            let sign: i64 = if item.value < 0 { -1 } else { 1 };

            for n in 0..abs_value {
                let i = mod_floor(index as i64 + sign * n, list2_len as i64);
                let j = mod_floor(index as i64 + sign * (n + 1), list2_len as i64);

                // Swap values.
                let tmp = list2[i as usize];
                list2[i as usize] = list2[j as usize];
                list2[j as usize] = tmp;
            }
        }
    }

    let index = list2.iter().position(|x| x.value == 0).unwrap();
    let num1 = list2[(index + 1000) % list2_len].value;
    let num2 = list2[(index + 2000) % list2_len].value;
    let num3 = list2[(index + 3000) % list2_len].value;
    let result = num1 + num2 + num3;
    println!(
        "result: {}, num1: {}, num2: {}, num3: {}",
        result, num1, num2, num3
    );

    Ok(())
}
