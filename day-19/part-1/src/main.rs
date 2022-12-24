use sscanf::sscanf;
use std::error::Error;
use std::io;

#[derive(sscanf::FromScanf, Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[sscanf(
    format = "Blueprint {id}: Each ore robot costs {ore_robot_ore_cost} ore. Each clay robot costs {clay_robot_ore_cost} ore. Each obsidian robot costs {obsidian_robot_ore_cost} ore and {obsidian_robot_clay_cost} clay. Each geode robot costs {geode_robot_ore_cost} ore and {geode_robot_obsidian_cost} obsidian."
)]
struct Blueprint {
    id: i32,
    ore_robot_ore_cost: i32,
    clay_robot_ore_cost: i32,
    obsidian_robot_ore_cost: i32,
    obsidian_robot_clay_cost: i32,
    geode_robot_ore_cost: i32,
    geode_robot_obsidian_cost: i32,
}

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

#[derive(Debug, Copy, Clone)]
struct Stat {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
    ore_robot: i32,
    clay_robot: i32,
    obsidian_robot: i32,
    geode_robot: i32,
}

struct Context {
    max_ore_cost: i32,
    max_clay_cost: i32,
    max_obsidian_cost: i32,
}

const INIT_TIME: i32 = 24;

fn go(ctx: &Context, blueprint: &Blueprint, previous_stat: Stat, minutes: i32) -> i32 {
    // let depth = INIT_TIME - minutes;
    // println!(
    //     "{}minutes_left: {}, o: {}, c: {}, ob: {}, g: {}, ro: {}, rc: {}, rob: {}, rg: {}",
    //     "| ".repeat(depth as usize),
    //     minutes,
    //     previous_stat.ore,
    //     previous_stat.clay,
    //     previous_stat.obsidian,
    //     previous_stat.geode,
    //     previous_stat.ore_robot,
    //     previous_stat.clay_robot,
    //     previous_stat.obsidian_robot,
    //     previous_stat.geode_robot
    // );

    if minutes == 0 {
        return previous_stat.geode;
    }

    let mut stat = previous_stat.clone();
    stat.ore += stat.ore_robot;
    stat.clay += stat.clay_robot;
    stat.obsidian += stat.obsidian_robot;
    stat.geode += stat.geode_robot;

    let mut max_geodes = 0;

    let can_build_geode_robot = previous_stat.ore >= blueprint.geode_robot_ore_cost
        && previous_stat.obsidian >= blueprint.geode_robot_obsidian_cost;
    if can_build_geode_robot {
        let mut s = stat.clone();
        s.ore -= blueprint.geode_robot_ore_cost;
        s.obsidian -= blueprint.geode_robot_obsidian_cost;
        s.geode_robot += 1;
        let geodes = go(ctx, blueprint, s, minutes - 1);

        max_geodes = std::cmp::max(max_geodes, geodes);
    }

    let can_build_obsidian_robot = previous_stat.ore >= blueprint.obsidian_robot_ore_cost
        && previous_stat.clay >= blueprint.obsidian_robot_clay_cost;
    let too_much_obsidian = previous_stat.obsidian >= (ctx.max_obsidian_cost * minutes);
    if can_build_obsidian_robot && !too_much_obsidian {
        let mut s = stat.clone();
        s.ore -= blueprint.obsidian_robot_ore_cost;
        s.clay -= blueprint.obsidian_robot_clay_cost;
        s.obsidian_robot += 1;
        let geodes = go(ctx, blueprint, s, minutes - 1);

        max_geodes = std::cmp::max(max_geodes, geodes);
        return max_geodes;
    }

    let can_build_clay_robot = previous_stat.ore >= blueprint.clay_robot_ore_cost;
    let too_much_clay = previous_stat.clay >= (ctx.max_clay_cost * minutes);
    if can_build_clay_robot && !too_much_clay {
        let mut s = stat.clone();
        s.ore -= blueprint.clay_robot_ore_cost;
        s.clay_robot += 1;
        let geodes = go(ctx, blueprint, s, minutes - 1);

        max_geodes = std::cmp::max(max_geodes, geodes);
    }

    let can_build_ore_robot = previous_stat.ore >= blueprint.ore_robot_ore_cost;
    let too_much_ore = previous_stat.ore >= (ctx.max_ore_cost * minutes);
    if can_build_ore_robot {
        let mut s = stat.clone();
        s.ore -= blueprint.ore_robot_ore_cost;
        s.ore_robot += 1;
        let geodes = go(ctx, blueprint, s, minutes - 1);

        max_geodes = std::cmp::max(max_geodes, geodes);
    }

    // Wait.
    {
        let s = stat.clone();
        let geodes = go(ctx, blueprint, s, minutes - 1);

        max_geodes = std::cmp::max(max_geodes, geodes);
    }

    // println!("{}max_geodes: {}", "| ".repeat(depth as usize), max_geodes,);

    max_geodes
}

// Solve the Advent of Code 2022 Day 19 puzzle.
fn main() -> Result<(), Box<dyn Error>> {
    let blueprints = stdin_lines()
        .map(|line| sscanf!(line, "{Blueprint}"))
        .collect::<Result<Vec<_>, _>>()?;
    println!("{:#?}", blueprints);

    let mut total = 0;

    for blueprint in &blueprints {
        let ctx = Context {
            // Max between ore_robot_ore_cost, clay_robot_ore_cost, obsidian_robot_ore_cost, geode_robot_ore_cost
            max_ore_cost: std::cmp::max(
                blueprint.ore_robot_ore_cost,
                std::cmp::max(
                    blueprint.clay_robot_ore_cost,
                    std::cmp::max(
                        blueprint.obsidian_robot_ore_cost,
                        blueprint.geode_robot_ore_cost,
                    ),
                ),
            ),
            max_clay_cost: blueprint.obsidian_robot_clay_cost,
            max_obsidian_cost: blueprint.geode_robot_obsidian_cost,
        };

        let result = go(
            &ctx,
            blueprint,
            Stat {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
                ore_robot: 1,
                clay_robot: 0,
                obsidian_robot: 0,
                geode_robot: 0,
            },
            INIT_TIME,
        );

        total += blueprint.id * result;

        println!(
            "result: {}, id: {}, to add: {}",
            result,
            blueprint.id,
            blueprint.id * result
        );
    }

    println!("total: {}", total);

    Ok(())
}
