use sscanf::sscanf;
use std::collections::HashMap;
use std::collections::HashSet;
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
            println!("{}", line);
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

#[derive(Clone)]
enum Action<'a> {
    Open(&'a Valve),
    GoTo(&'a Valve),
}

struct Context {
    valves: Vec<Valve>,
    cache: HashMap<String, (i64, String)>,
}

fn generate_cache_key(
    action: &Action,
    minutes_left: i64,
    opened_valves: &HashSet<String>,
) -> String {
    let mut v = opened_valves
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    v.sort();
    let str = v.join(",");
    let cache_key = format!(
        "{}{}{}",
        match action {
            Action::Open(v) => format!("o{}", v.name),
            Action::GoTo(v) => format!("g{}", v.name),
        },
        minutes_left - 1,
        str
    );
    cache_key
}

fn get_best_total_for_action(
    ctx: &mut Context,
    action: &Action,
    minutes_left: i64,
    opened_valves: &HashSet<String>,
    depth: i64,
) -> (i64, String) {
    if minutes_left == 0 {
        return (0, "-".to_string());
    }

    let mut total = 0;
    let mut new_opened_valves = opened_valves.clone();

    let current_valve = match action {
        Action::Open(ref valve) => {
            assert!(!new_opened_valves.contains(&*valve.name));

            total = (minutes_left - 1) * valve.rate;
            let n = valve.name.clone();
            new_opened_valves.insert(n);

            valve
        }
        Action::GoTo(ref valve) => valve,
    };

    // println!(
    //     "{}{}, minutes_left: {}",
    //     "| ".repeat(depth as usize),
    //     match action {
    //         Action::Open(v) => format!("Open {}, adds: {}", v.name, total),
    //         Action::GoTo(v) => format!("Go {}", v.name),
    //     },
    //     minutes_left
    // );

    let mut best_sub_total: i64 = -1;
    let mut best_path: String = "".to_string();

    for name in &current_valve.connections {
        let valve = ctx
            .valves
            .iter()
            .find(|v| &*v.name == &*name)
            .unwrap()
            .clone();

        // let cache_key = format!("{}{}", valve.name.clone(), minutes_left - 1);
        let cache_key =
            generate_cache_key(&Action::GoTo(&valve), minutes_left - 1, &new_opened_valves);
        let (sub_total, path) = if let Some((cached, path)) = ctx.cache.get(&cache_key) {
            // println!(
            //     "{}Go {},total {} (cached: {})",
            //     "| ".repeat((depth + 1) as usize),
            //     valve.name,
            //     cached,
            //     cache_key,
            // );
            (*cached, path.clone())
        } else {
            get_best_total_for_action(
                ctx,
                &Action::GoTo(&valve),
                minutes_left - 1,
                &new_opened_valves,
                depth + 1,
            )
        };

        if sub_total > best_sub_total {
            best_sub_total = sub_total;
            best_path = path;
            // println!(
            //     "{}Best sub-path for {} is {}: {}",
            //     "| ".repeat((depth) as usize),
            //     current_valve.name,
            //     best_path,
            //     best_sub_total
            // );
        }
    }

    if current_valve.rate != 0 && !new_opened_valves.contains(&*current_valve.name) {
        // let cache_key = format!("{}{}", current_valve.name.clone(), minutes_left - 1);
        let cache_key = generate_cache_key(
            &Action::Open(&current_valve),
            minutes_left - 1,
            &new_opened_valves,
        );
        let (sub_total, path) = if let Some((cached, path)) = ctx.cache.get(&cache_key) {
            // println!(
            //     "{}Open {}, adds: {} (cached: {})",
            //     "| ".repeat((depth + 1) as usize),
            //     current_valve.name,
            //     cached,
            //     cache_key,
            // );
            (*cached, path.clone())
        } else {
            get_best_total_for_action(
                ctx,
                &Action::Open(&current_valve),
                minutes_left - 1,
                &new_opened_valves,
                depth + 1,
            )
        };

        if sub_total > best_sub_total {
            best_sub_total = sub_total;
            best_path = path;
            // println!(
            //     "{}Best sub-path for {} is {}: {}",
            //     "| ".repeat((depth) as usize),
            //     current_valve.name,
            //     best_path,
            //     best_sub_total
            // );
        }
    }

    let result = total + best_sub_total;

    // println!("{}| result: {}", "  ".repeat(depth as usize), result);

    let path = format!(
        "{}{}",
        match action {
            Action::Open(v) => format!("O{}", v.name),
            Action::GoTo(v) => format!("G{}", v.name),
        },
        best_path
    );

    // println!(
    //     "{}>{}, result: {} ({} + {}), minutes_left: {}, path: {}",
    //     "| ".repeat(depth as usize),
    //     match action {
    //         Action::Open(v) => format!("Open {}", v.name),
    //         Action::GoTo(v) => format!("Go {}", v.name),
    //     },
    //     result,
    //     total,
    //     best_sub_total,
    //     minutes_left,
    //     path
    // );

    // let cache_key = format!("{}{}", current_valve.name.clone(), minutes_left);
    let cache_key = generate_cache_key(&action, minutes_left, &new_opened_valves);
    assert!(!ctx.cache.contains_key(&cache_key));
    ctx.cache.insert(cache_key, (result, path.clone()));

    (result, path)
}

// Solve the Advent of Code 2022 Day 16 puzzle.
fn main() -> Result<(), Box<dyn Error>> {
    let valves = parse_input();
    println!("Valves: {:?}", valves);

    let minutes_left = 31;
    let current_valve = valves
        .iter()
        .find(|v| v.name == "AA")
        .expect("Could not find valve AA")
        .clone();

    let mut context = Context {
        valves,
        cache: HashMap::new(),
    };
    let opened_valves: HashSet<String> = HashSet::new();

    let (total, path) = get_best_total_for_action(
        &mut context,
        &Action::GoTo(&current_valve),
        minutes_left,
        &opened_valves,
        0,
    );

    println!("Total: {}, path: {}", total, path);

    Ok(())
}
