use itertools::Itertools;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::io;

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

fn parse_map(lines: impl Iterator<Item = String>) -> (Vec<Vec<usize>>, Pos, Pos) {
    let mut start: Option<Pos> = None;
    let mut end: Option<Pos> = None;

    let map: Vec<Vec<usize>> = lines
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = Some(Pos { x, y });
                        return 0;
                    }
                    if c == 'E' {
                        end = Some(Pos { x, y });
                        return 25;
                    }
                    c as usize - 'a' as usize
                })
                .collect_vec()
        })
        .collect();

    (map, start.unwrap(), end.unwrap())
}

fn get_walkable_surrounding_pos(map: &Vec<Vec<usize>>, pos: &Pos) -> Vec<Pos> {
    let current_level = map[pos.y][pos.x];

    let mut surrounding = Vec::new();
    if pos.x > 0 {
        if map[pos.y][pos.x - 1] as i64 >= current_level as i64 - 1 {
            surrounding.push(Pos {
                x: pos.x - 1,
                y: pos.y,
            });
        }
    }
    if pos.x < map[0].len() - 1 {
        if map[pos.y][pos.x + 1] as i64 >= current_level as i64 - 1 {
            surrounding.push(Pos {
                x: pos.x + 1,
                y: pos.y,
            });
        }
    }
    if pos.y > 0 {
        if map[pos.y - 1][pos.x] as i64 >= current_level as i64 - 1 {
            surrounding.push(Pos {
                x: pos.x,
                y: pos.y - 1,
            });
        }
    }
    if pos.y < map.len() - 1 {
        if map[pos.y + 1][pos.x] as i64 >= current_level as i64 - 1 {
            surrounding.push(Pos {
                x: pos.x,
                y: pos.y + 1,
            });
        }
    }
    surrounding
}

#[derive(Debug)]
struct Node {
    distance: usize,
    previous: Option<Pos>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    distance: usize,
    pos: Pos,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Solve the Advent of Code 2022 Day 12 puzzle.
fn main() -> Result<(), Box<dyn Error>> {
    let lines = stdin_lines();
    let (map, start, end) = parse_map(lines);
    let mut distances: HashMap<Pos, Node> = HashMap::new();
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut to_visit: BinaryHeap<State> = BinaryHeap::new();

    to_visit.push(State {
        distance: 0,
        pos: end,
    });
    distances.insert(
        end,
        Node {
            distance: 0,
            previous: None,
        },
    );

    // Implement Disjkstra's algorithm.
    while let Some(State { distance, pos }) = to_visit.pop() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);

        let surrounding = get_walkable_surrounding_pos(&map, &pos);
        for next_pos in surrounding {
            let next_distance = distance + 1;
            let next_node = Node {
                distance: next_distance,
                previous: Some(pos),
            };
            if let Some(node) = distances.get(&next_pos) {
                if node.distance > next_distance {
                    distances.insert(next_pos, next_node);
                    to_visit.push(State {
                        distance: next_distance,
                        pos: next_pos,
                    });
                }
            } else {
                distances.insert(next_pos, next_node);
                to_visit.push(State {
                    distance: next_distance,
                    pos: next_pos,
                });
            }
        }
    }

    let path: HashMap<Pos, char> = {
        let mut path = HashMap::new();
        let mut current = Some(start);
        while current.is_some() {
            let current_pos = current.unwrap();
            let current_node = distances.get(&current_pos).unwrap();
            let previous_opt = current_node.previous;
            current = previous_opt;

            if previous_opt.is_none() {
                path.insert(current_pos, 'E');
                continue;
            }

            let previous = previous_opt.unwrap();
            if current_pos.y as i64 == previous.y as i64 - 1 {
                path.insert(current_pos, 'v');
            } else if current_pos.y as i64 == previous.y as i64 + 1 {
                path.insert(current_pos, '^');
            } else if current_pos.x as i64 == previous.x as i64 + 1 {
                path.insert(current_pos, '<');
            } else if current_pos.x as i64 == previous.x as i64 - 1 {
                path.insert(current_pos, '>');
            }
        }
        path
    };

    println!("--=== Part 1 ===--\n");
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let pos = Pos { x, y };
            if pos == start {
                print!("S");
            } else if pos == end {
                print!("E");
            } else if path.contains_key(&pos) {
                let c = path.get(&pos).unwrap();
                print!("{c}");
            } else {
                print!(".");
            }
        }
        println!();
    }

    let result = distances.get(&start).unwrap();
    println!("\nResult: {:?}", result.distance);
    println!("\n--=== Part 2 ===--\n");

    // Find the smallest distance to the end for map entries that have a value of 0.
    let positions_with_value_0: Vec<Pos> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &value)| value == 0)
                .map(|(x, _)| Pos { x, y })
                .collect_vec()
        })
        .collect();

    let min_distance = positions_with_value_0
        .iter()
        .filter_map(|pos| distances.get(pos))
        .map(|node| node.distance)
        .min()
        .unwrap();

    println!("Min distance: {}", min_distance);

    Ok(())
}
