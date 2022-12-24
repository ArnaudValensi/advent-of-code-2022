use itertools::Itertools;
use sscanf::sscanf;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::io;

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Pos {
    x: i64,
    y: i64,
}

// Solve the Advent of Code 2022 Day 15 puzzle.
fn main() -> Result<(), Box<dyn Error>> {
    let lines = stdin_lines();
    let mut sensors_distances: HashMap<Pos, i64> = HashMap::new();
    let mut beacons: HashSet<Pos> = HashSet::new();

    for line in lines {
        let (sensor_x, sensor_y, beacon_x, beacon_y) = sscanf!(
            line,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            i64,
            i64,
            i64,
            i64
        )
        .unwrap();

        let manhattan_distance = i64::abs(sensor_x - beacon_x) + i64::abs(sensor_y - beacon_y);

        sensors_distances.insert(
            Pos {
                x: sensor_x,
                y: sensor_y,
            },
            manhattan_distance,
        );

        beacons.insert(Pos {
            x: beacon_x,
            y: beacon_y,
        });
    }

    let (min_x, max_x) = sensors_distances
        .keys()
        .chain(beacons.iter())
        .map(|p| p.x)
        .minmax()
        .into_option()
        .unwrap();

    let is_pos_in_sensor_range_and_not_beacon = |p: &Pos| {
        let is_in_range = sensors_distances.iter().any(|(sensor_pos, distance)| {
            i64::abs(p.x - sensor_pos.x) + i64::abs(p.y - sensor_pos.y) <= *distance
        });

        let has_beacon = beacons.contains(&p);
        // let has_sensor = sensors_distances.contains_key(&p);

        // is_in_range && !has_beacon && !has_sensor
        is_in_range && !has_beacon
    };

    // From min_x and max_y, check how many sensor positions are in range and not beacons.
    let result = ((min_x - 10000000)..=(max_x + 1000000)).fold(0, |acc, x| {
        let y = 2000000;
        let pos = Pos { x, y };

        if is_pos_in_sensor_range_and_not_beacon(&pos) {
            acc + 1
        } else {
            acc
        }
    });

    println!("{}", result);

    // let print_map = |sensors_distances: &HashMap<Pos, i64>, beacons: &HashSet<Pos>| {
    //     let (min_y, max_y) = sensors_distances
    //         .keys()
    //         .chain(beacons.iter())
    //         .map(|p| p.y)
    //         .minmax()
    //         .into_option()
    //         .unwrap();

    //     for y in min_y..=max_y {
    //         for x in min_x..=max_x {
    //             let pos = Pos { x, y };

    //             if beacons.contains(&pos) {
    //                 print!("B");
    //             } else if sensors_distances.contains_key(&pos) {
    //                 print!("S");
    //             } else if is_pos_in_sensor_range_and_not_beacon(&pos) {
    //                 print!("#");
    //             } else {
    //                 print!(".");
    //             }
    //         }
    //         println!();
    //     }
    // };

    // print_map(&sensors_distances, &beacons);

    Ok(())
}
