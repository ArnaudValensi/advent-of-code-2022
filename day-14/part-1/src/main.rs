use itertools::Itertools;
use std::error::Error;
use std::io;
use std::{thread, time};

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

fn draw_map(map: &Vec<Vec<char>>) {
    let mut str = String::new();

    str.push_str(&format!("{esc}[2J{esc}[1;1H", esc = 27 as char));
    for (y, row) in map.iter().enumerate() {
        str.push_str(&format!("{y} "));
        for c in row {
            str.push_str(&format!("{}", c));
        }
        str.push_str("\n");
    }

    print!("{}", str);
}

// Solve the Advent of Code 2022 Day 14 puzzle.
fn main() -> Result<(), Box<dyn Error>> {
    // Parse paths.
    let paths: Vec<Vec<(i64, i64)>> = stdin_lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coord| {
                    coord
                        .split(',')
                        .map(|n| n.parse::<i64>().unwrap())
                        .collect_tuple::<(i64, i64)>()
                        .unwrap()
                })
                .collect::<Vec<(i64, i64)>>()
        })
        .collect();

    // Get map bounds.
    let min_x = paths.iter().flat_map(|n| n).map(|n| n.0).min().unwrap();
    let max_x = paths.iter().flat_map(|n| n).map(|n| n.0).max().unwrap();
    let max_y = paths.iter().flat_map(|n| n).map(|n| n.1).max().unwrap();

    let is_in_bound = |pos: (i64, i64)| {
        (pos.0 - min_x) >= 0 && (pos.0 - min_x) <= max_x && pos.1 >= 0 && pos.1 <= max_y
    };

    println!("{min_x}, {max_x}; 0, {max_y}");

    // Create and fill the map with paths.
    let mut map: Vec<Vec<char>> =
        vec![vec![' '; (max_x - min_x + 1) as usize]; (max_y + 1) as usize];

    // Initialize the entire map with the char '.'.
    for row in map.iter_mut() {
        for c in row.iter_mut() {
            *c = '.';
        }
    }

    for path in paths {
        // Iterate of path with a window of 2.
        for (a, b) in path.iter().tuple_windows() {
            let (x1, y1) = a;
            let (x2, y2) = b;

            // Draw a line between the two points.
            if x1 == x2 {
                // Vertical line.
                let (y1, y2) = if y1 < y2 { (*y1, *y2) } else { (*y2, *y1) };
                for y in y1..=y2 {
                    map[y as usize][(x1 - min_x) as usize] = '#';
                }
            } else {
                // Horizontal line.
                let (x1, x2) = if x1 < x2 { (*x1, *x2) } else { (*x2, *x1) };
                for x in x1..=x2 {
                    map[*y1 as usize][(x - min_x) as usize] = '#';
                }
            }
        }
    }

    // Draw the map.
    draw_map(&map);

    let directions = [(0, 1), (-1, 1), (1, 1)];
    let sleep_time = time::Duration::from_millis(100);
    let mut falling_sand_pos = None;
    let mut num_resting_units = 0;
    loop {
        if falling_sand_pos.is_none() {
            falling_sand_pos = Some((500, 0));
            continue;
        }

        let sand_pos = falling_sand_pos.unwrap();

        if let Some(pos) = directions.iter().find_map(|direction| {
            let pos = (sand_pos.0 + direction.0, sand_pos.1 + direction.1);

            if !is_in_bound(pos) {
                return Some(pos);
            }

            if map[pos.1 as usize][(pos.0 - min_x) as usize] == '.' {
                return Some(pos);
            }

            None
        }) {
            if !is_in_bound(pos) {
                draw_map(&map);
                println!("Result: {}", num_resting_units);
                return Ok(());
            }
            map[sand_pos.1 as usize][(sand_pos.0 - min_x) as usize] = '.';
            map[pos.1 as usize][(pos.0 - min_x) as usize] = '+';
            falling_sand_pos = Some(pos);
        } else {
            map[sand_pos.1 as usize][(sand_pos.0 - min_x) as usize] = 'o';
            num_resting_units += 1;
            falling_sand_pos = None;
        }

        // draw_map(&map);
        // thread::sleep(sleep_time);
    }
}
