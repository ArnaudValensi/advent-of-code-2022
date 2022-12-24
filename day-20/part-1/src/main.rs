use sscanf::sscanf;
use std::error::Error;
use std::io;

#[derive(Debug, Clone, Copy)]
struct Item {
    id: i32,
    value: i32,
}

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

fn mod_floor(a: i32, base: i32) -> i32 {
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
    let numbers: Vec<i32> = stdin_lines()
        .map(|line| sscanf!(line, "{i32}"))
        .collect::<Result<Vec<_>, _>>()?;

    // println!("{:#?}", numbers);

    let list1 = numbers
        .iter()
        .enumerate()
        .map(|(i, &n)| Item {
            id: i as i32,
            value: n,
        })
        .collect::<Vec<_>>();

    let mut list2 = list1.clone();

    for item in list1.iter() {
        // println!("item: {:#?}", item);

        let index = list2.iter().position(|x| x.id == item.id).unwrap();

        let abs_value = i32::abs(item.value);
        let sign = if item.value < 0 { -1 } else { 1 };

        for n in 0..abs_value {
            let i = mod_floor(index as i32 + sign * n, list2.len() as i32);
            let j = mod_floor(index as i32 + sign * (n + 1), list2.len() as i32);

            // Swap values.
            let tmp = list2[i as usize];
            list2[i as usize] = list2[j as usize];
            list2[j as usize] = tmp;

            // let mut line = String::new();
            // for item in list2.iter() {
            //     line.push_str(&format!("{} ", item.value));
            // }
            // println!("step: {}", line);
        }

        // println!("list2: {:#?}", list2);

        // Print all value of list2 on a single line.
        let mut line = String::new();
        for item in list2.iter() {
            line.push_str(&format!("{} ", item.value));
        }
        // println!("final: {}", line);
    }

    let index = list2.iter().position(|x| x.value == 0).unwrap();
    let num1 = list2[(index + 1000) % list2.len()].value;
    let num2 = list2[(index + 2000) % list2.len()].value;
    let num3 = list2[(index + 3000) % list2.len()].value;
    let result = num1 + num2 + num3;
    println!(
        "result: {}, num1: {}, num2: {}, num3: {}",
        result, num1, num2, num3
    );

    Ok(())
}
