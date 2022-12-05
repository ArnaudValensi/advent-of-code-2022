use itertools::Itertools;
use std::error::Error;
use std::io;

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

fn has_smaller_trees_in_direction(
    map: &Vec<Vec<i64>>,
    pos_x: usize,
    pos_y: usize,
    dir_x: i64,
    dir_y: i64,
) -> bool {
    let current_height = map[pos_y][pos_x];
    let map_width = map[0].len() as i64;
    let map_height = map.len() as i64;
    let mut x: i64 = pos_x as i64;
    let mut y: i64 = pos_y as i64;

    loop {
        x += dir_x;
        y += dir_y;

        if x < 0 || x >= map_width || y < 0 || y >= map_height {
            return true;
        }

        if map[y as usize][x as usize] >= current_height {
            return false;
        }
    }
}

fn num_trees_viewed_in_direction(
    map: &Vec<Vec<i64>>,
    pos_x: usize,
    pos_y: usize,
    dir_x: i64,
    dir_y: i64,
) -> i64 {
    let current_height = map[pos_y][pos_x];
    let map_width = map[0].len() as i64;
    let map_height = map.len() as i64;
    let mut x: i64 = pos_x as i64;
    let mut y: i64 = pos_y as i64;
    let mut num: i64 = 0;

    loop {
        x += dir_x;
        y += dir_y;

        if x < 0 || x >= map_width || y < 0 || y >= map_height {
            return num;
        }

        if map[y as usize][x as usize] >= current_height {
            return num + 1;
        }

        num += 1;
    }
}

// Solve the Advent of Code 2022 Day 8 puzzle.
fn main() -> Result<(), Box<dyn Error>> {
    let map: Vec<Vec<i64>> = stdin_lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect_vec()
        })
        .collect_vec();

    let map_width = map[0].len();
    let map_height = map.len();

    let mut visible_tree_count = map_width * 2 + map_height * 2 - 4;
    let mut max_scenic_score = 0;

    for y in 1..map_height - 1 {
        for x in 1..map_width - 1 {
            // Part 1.
            if has_smaller_trees_in_direction(&map, x, y, 0, -1)
                || has_smaller_trees_in_direction(&map, x, y, 1, 0)
                || has_smaller_trees_in_direction(&map, x, y, 0, 1)
                || has_smaller_trees_in_direction(&map, x, y, -1, 0)
            {
                visible_tree_count += 1;
            }

            // Part 2.
            let mut score = 1;
            score *= num_trees_viewed_in_direction(&map, x, y, 0, -1);
            score *= num_trees_viewed_in_direction(&map, x, y, 1, 0);
            score *= num_trees_viewed_in_direction(&map, x, y, 0, 1);
            score *= num_trees_viewed_in_direction(&map, x, y, -1, 0);
            if score > max_scenic_score {
                max_scenic_score = score;
            }
        }
    }

    println!("Part 1: {visible_tree_count}");
    println!("Part 2: {max_scenic_score}");

    Ok(())
}
