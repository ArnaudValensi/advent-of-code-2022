use sscanf::sscanf;
use std::collections::HashMap;
use std::error::Error;
use std::io;

#[derive(Debug, PartialEq)]
enum BlockType {
    Lava,
    OutsideAir,
}

#[derive(sscanf::FromScanf, Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[sscanf(format = "{x},{y},{z}")]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

// Solve the Advent of Code 2022 Day 18 puzzle.
fn main() -> Result<(), Box<dyn Error>> {
    let mut blocks = HashMap::new();
    for line in stdin_lines() {
        let pos = sscanf!(line, "{Pos}").unwrap();
        blocks.insert(pos, BlockType::Lava);
    }

    let directions = [
        Pos { x: 0, y: 1, z: 0 },
        Pos { x: 0, y: -1, z: 0 },
        Pos { x: 1, y: 0, z: 0 },
        Pos { x: -1, y: 0, z: 0 },
        Pos { x: 0, y: 0, z: 1 },
        Pos { x: 0, y: 0, z: -1 },
    ];

    let mut count = 0;
    for (pos, _) in blocks.iter() {
        for dir in directions.iter() {
            let adj = Pos {
                x: pos.x + dir.x,
                y: pos.y + dir.y,
                z: pos.z + dir.z,
            };
            if !blocks.contains_key(&adj) {
                count += 1;
            }
        }
    }

    println!("Part 1: {}", count);

    // Get the min and max between x, y, and z of all blocks.
    let min = blocks
        .keys()
        .map(|pos| pos.x.min(pos.y).min(pos.z))
        .min()
        .unwrap()
        - 1;
    let max = blocks
        .keys()
        .map(|pos| pos.x.max(pos.y).max(pos.z))
        .max()
        .unwrap()
        + 1;

    // Flood fill with air all the blocks that are outisde the lava.
    let mut queue = Vec::new();
    queue.push(Pos {
        x: min,
        y: min,
        z: min,
    });
    while !queue.is_empty() {
        let pos = queue.pop().unwrap();
        if blocks.contains_key(&pos) {
            continue;
        }
        blocks.insert(pos, BlockType::OutsideAir);
        for dir in directions.iter() {
            let adj = Pos {
                x: pos.x + dir.x,
                y: pos.y + dir.y,
                z: pos.z + dir.z,
            };
            if adj.x < min
                || adj.x > max
                || adj.y < min
                || adj.y > max
                || adj.z < min
                || adj.z > max
            {
                continue;
            }
            if !blocks.contains_key(&adj) {
                queue.push(adj);
            }
        }
    }

    let mut count = 0;
    for (pos, _) in blocks.iter() {
        if blocks.get(pos) != Some(&BlockType::Lava) {
            continue;
        }
        for dir in directions.iter() {
            let adj = Pos {
                x: pos.x + dir.x,
                y: pos.y + dir.y,
                z: pos.z + dir.z,
            };
            if blocks.get(&adj) == Some(&BlockType::OutsideAir) {
                count += 1;
            }
        }
    }

    println!("Part 2: {}", count);

    Ok(())
}
