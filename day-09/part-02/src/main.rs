use sscanf::sscanf;
use std::collections::HashSet;
use std::error::Error;
use std::io;

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

fn normalize(n: i32) -> i32 {
    if n < 0 {
        return -1;
    }
    if n > 0 {
        return 1;
    }
    n
}

// Solve the Advent of Code 2022 Day 9 puzzle.
fn main() -> Result<(), Box<dyn Error>> {
    let lines = stdin_lines();
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    let mut rope = vec![
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();

    visited_positions.insert((0, 0));

    for line in lines {
        println!("---\n{}\n", line);

        let (direction, steps) = sscanf!(line, "{char} {i64}").unwrap();

        for _ in 0..steps {
            match direction {
                'U' => rope.get_mut(0).unwrap().1 += 1,
                'D' => rope.get_mut(0).unwrap().1 -= 1,
                'R' => rope.get_mut(0).unwrap().0 += 1,
                'L' => rope.get_mut(0).unwrap().0 -= 1,
                _ => println!("Error: unknown direction {}", direction),
            }

            for i in 1..10 {
                let (head_x, head_y) = rope.get(i - 1).unwrap().clone();
                let (tail_x, tail_y) = rope.get_mut(i).unwrap();

                let is_tail_next_to_head =
                    i32::abs(head_x - *tail_x) <= 1 && i32::abs(head_y - *tail_y) <= 1;

                if !is_tail_next_to_head {
                    *tail_x += normalize(head_x - *tail_x);
                    *tail_y += normalize(head_y - *tail_y);

                    if i == 9 {
                        visited_positions.insert((*tail_x, *tail_y));
                    }
                }
            }

            {
                let (head_x, head_y) = rope.get(0).unwrap();

                if *head_x < min_x {
                    min_x = *head_x;
                }
                if *head_x > max_x {
                    max_x = *head_x;
                }
                if *head_y < min_y {
                    min_y = *head_y;
                }
                if *head_y > max_y {
                    max_y = *head_y;
                }
            }

            // Print steps.
            for y in (min_y..=max_y).rev() {
                for x in min_x..=max_x {
                    let mut found = false;
                    for (n, (tail_x, tail_y)) in rope.iter().enumerate() {
                        if x == *tail_x && y == *tail_y {
                            print!("{n}");
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        print!(".");
                    }
                }
                println!();
            }
            println!();
        }
    }

    // Print trail.
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            if visited_positions.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    let result = visited_positions.len();
    println!("\n{result}");

    Ok(())
}
