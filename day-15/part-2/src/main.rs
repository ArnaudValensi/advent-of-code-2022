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

    let max = 4000000;
    let mut beacon_pos_opt = None;

    let is_pos_in_sensor_range_or_is_beacon = |p: &Pos| {
        if p.x < 0 || p.x > max || p.y < 0 || p.y > max {
            return true;
        }

        let is_in_range = sensors_distances.iter().any(|(sensor_pos, distance)| {
            i64::abs(p.x - sensor_pos.x) + i64::abs(p.y - sensor_pos.y) <= *distance
        });

        let has_beacon = beacons.contains(&p);

        is_in_range || has_beacon
    };

    for (pos, distance) in sensors_distances.iter() {
        let dist = distance + 1;

        for x1 in 0..=dist {
            let y1 = dist - x1;
            let y2 = -y1;
            let x2 = -x1;

            let pos1 = Pos {
                x: pos.x + x1,
                y: pos.y + y1,
            };

            let pos2 = Pos {
                x: pos.x + x2,
                y: pos.y + y2,
            };

            let pos3 = Pos {
                x: pos.x + x1,
                y: pos.y + y2,
            };

            let pos4 = Pos {
                x: pos.x + x2,
                y: pos.y + y1,
            };

            if !is_pos_in_sensor_range_or_is_beacon(&pos1) {
                beacon_pos_opt = Some(pos1);
                break;
            }

            if !is_pos_in_sensor_range_or_is_beacon(&pos2) {
                beacon_pos_opt = Some(pos2);
                break;
            }

            if !is_pos_in_sensor_range_or_is_beacon(&pos3) {
                beacon_pos_opt = Some(pos3);
                break;
            }

            if !is_pos_in_sensor_range_or_is_beacon(&pos4) {
                beacon_pos_opt = Some(pos4);
                break;
            }
        }
    }

    let beacon_pos = beacon_pos_opt.unwrap();
    let result = beacon_pos.x * 4000000 + beacon_pos.y;

    println!("{}", result);

    Ok(())
}
