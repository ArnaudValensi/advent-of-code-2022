use sscanf::sscanf;
use std::cmp::min;
use std::error::Error;
use std::io;

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

#[derive(Clone, Debug)]
struct Valve {
    name: String,
    rate: i64,
    connections: Vec<String>,
}

fn parse_input() -> Vec<Valve> {
    stdin_lines()
        .map(|line| {
            // println!("{}", line);
            let (name, rate, _, _, _, connections) = sscanf!(
                line,
                "Valve {} has flow rate={}; {:/tunnels?/} {:/leads?/} to {:/valves?/} {}",
                String,
                i64,
                String,
                String,
                String,
                String,
            )
            .unwrap();

            Valve {
                name: name.to_string(),
                rate,
                connections: connections.split(", ").map(|s| s.to_string()).collect(),
            }
        })
        .collect()
}

fn floyd_warshall(graph: &mut Vec<Vec<i64>>) {
    let n = graph.len();
    for i in 0..n {
        for j in 0..n {
            let v2 = graph[j][i];
            for k in 0..n {
                graph[j][k] = min(graph[j][k], v2 + graph[i][k]);
            }
        }
    }
}

struct Context {
    non_broken_valves: Vec<usize>,
    distances: Vec<Vec<i64>>,
    rates: Vec<i64>,
}

#[derive(Clone, Copy, Debug)]
struct Permutation {
    permutation: u64,
    score: i64,
}

fn find_permutations(
    ctx: &Context,
    permutations: &mut Vec<Permutation>,
    valve: usize,
    visited: u64,
    permutation: Permutation,
    minutes: i64,
) {
    let mut new_visited = visited.clone();
    // Set bit at position 'valve'.
    new_visited |= 1 << (valve as u64);

    let new_score = permutation.score + (minutes * ctx.rates[valve]);

    let new_permutation = Permutation {
        permutation: new_visited,
        score: new_score,
    };

    permutations.push(new_permutation);

    let next_valves = ctx.distances[valve]
        .iter()
        .enumerate()
        .filter(|(valve, _)| ctx.non_broken_valves.contains(valve));

    for (next_valve, distance) in next_valves {
        let is_valve_visited = new_visited & (1 << next_valve) != 0;
        if !is_valve_visited {
            let new_minutes = minutes - (distance + 1);
            if new_minutes >= 0 {
                find_permutations(
                    ctx,
                    permutations,
                    next_valve,
                    new_visited,
                    new_permutation,
                    new_minutes,
                );
            }
        }
    }
}

// Solve the Advent of Code 2022 Day 16 puzzle.
// - Create a weighted graph using floyd_warshall algorithm.
// - Find all permutations of the non broken valves, in the available minutes.
//   The permutations are stored as bits in a u64.
// - To find the best path for ourself and the elephant, we are looking for the best path with
//   no visited nodes in common. To do so we compare each permutations we each other and do a
//   bitwise 'and' between the bitsets.
fn main() -> Result<(), Box<dyn Error>> {
    let graph = parse_input();

    // Prepare the matrice for floyd warshall algorithm.
    let mut distances: Vec<Vec<i64>> = graph
        .iter()
        .map(|v| {
            graph
                .iter()
                .map(|v2| {
                    if v.name == v2.name {
                        0
                    } else if v.connections.contains(&v2.name) {
                        1
                    } else {
                        1_000_000_000
                    }
                })
                .collect()
        })
        .collect();

    // Find the shorted distances from every node to every other.
    floyd_warshall(&mut distances);

    // Find the index of only the non-broken nodes.
    let non_broken_valves: Vec<usize> = graph
        .iter()
        .enumerate()
        .filter(|(_, v)| v.rate > 0)
        .map(|(i, _)| i)
        .collect();

    // Find the starting point.
    let valve_aa = graph.iter().position(|v| v.name == "AA").unwrap();

    // Find all the permutations.
    let mut ctx = Context {
        non_broken_valves,
        distances,
        rates: graph.iter().map(|v| v.rate).collect(),
    };
    let mut permutations: Vec<Permutation> = Vec::new();
    let permutation = Permutation {
        permutation: 0,
        score: 0,
    };
    find_permutations(&mut ctx, &mut permutations, valve_aa, 0, permutation, 26);

    // Find every pair of permutations with no bits in common and get their best score.
    let mut max = 0;
    for (i, p1) in permutations.iter().enumerate() {
        for (j, p2) in permutations.iter().enumerate() {
            if i != j {
                let common_bits =
                    (p1.permutation & !(1 << valve_aa)) & (p2.permutation & !(1 << valve_aa));
                if common_bits == 0 {
                    if p1.score + p2.score > max {
                        max = p1.score + p2.score;
                    }
                }
            }
        }
    }

    println!("max: {}", max);

    Ok(())
}
