use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;
use std::io;

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

fn draw_map(map: &HashMap<(i64, i64), char>) {
    // Find the min and max of the map.
    let (min_x, max_x) = map.keys().map(|(x, _)| x).minmax().into_option().unwrap();
    let (_min_y, max_y) = map.keys().map(|(_, y)| y).minmax().into_option().unwrap();

    let mut str = String::new();

    str.push_str(&format!("{esc}[2J{esc}[1;1H", esc = 27 as char));
    for y in 0..=*max_y {
        str.push_str(&format!("{y:<10} ", y = y));
        for x in *min_x..=*max_x {
            str.push_str(&format!("{}", map.get(&(x, y)).unwrap_or(&'.')));
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

    let max_y = paths.iter().flat_map(|n| n).map(|n| n.1).max().unwrap() + 2;

    let mut map: HashMap<(i64, i64), char> = HashMap::new();

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
                    map.insert((*x1, y), '#');
                }
            } else {
                // Horizontal line.
                let (x1, x2) = if x1 < x2 { (*x1, *x2) } else { (*x2, *x1) };
                for x in x1..=x2 {
                    map.insert((x, *y1), '#');
                }
            }
        }
    }

    let directions = [(0, 1), (-1, 1), (1, 1)];
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

            if pos.1 >= max_y {
                return None;
            }

            if map.get(&pos).unwrap_or(&'.') == &'.' {
                return Some(pos);
            }

            None
        }) {
            map.insert(sand_pos, '.');
            map.insert(pos, '+');

            falling_sand_pos = Some(pos);
        } else {
            map.insert(sand_pos, 'o');

            num_resting_units += 1;
            falling_sand_pos = None;

            if sand_pos == (500, 0) {
                draw_map(&map);
                println!("Result: {}", num_resting_units);
                return Ok(());
            }
        }
    }
}
