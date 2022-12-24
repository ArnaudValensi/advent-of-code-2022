use std::cmp::max;
use std::collections::HashMap;
use std::error::Error;
use std::io;

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

const MAP_WIDTH: i64 = 7;
const MAP_HEIGHT: i64 = 131072 / MAP_WIDTH;

struct Map {
    data: [u8; 131072],
    previous_draw: Vec<(i64, i64)>,
}

impl Map {
    fn set(&mut self, x: i64, y: i64, value: bool) {
        let index = y * MAP_WIDTH + x;
        self.data[index as usize] = value as u8;
    }

    fn get(&self, x: i64, y: i64) -> bool {
        let index = y * MAP_WIDTH + x;
        self.data[index as usize] != 0
    }

    fn render(&self, height: i64) {
        let h = height.min(MAP_HEIGHT);
        for y in 0..h {
            for x in 0..MAP_WIDTH {
                print!("{}", if self.get(x, y) { '#' } else { '.' });
            }
            println!();
        }
    }

    fn draw_piece(&mut self, x: i64, y: i64, piece: &Piece) {
        self.previous_draw.clear();
        for (dy, row) in piece.data.iter().enumerate() {
            for (dx, value) in row.iter().enumerate() {
                if *value {
                    self.set(x + dx as i64, y + dy as i64, true);
                    self.previous_draw.push((x + dx as i64, y + dy as i64));
                }
            }
        }
    }

    fn clear_previous_piece(&mut self) {
        // TODO: Remove allocation?
        for (x, y) in self.previous_draw.clone() {
            self.set(x, y, false);
        }
        self.previous_draw.clear();
    }

    fn has_collision(&self, piece: &Piece, x: i64, y: i64) -> bool {
        for (dy, row) in piece.data.iter().enumerate() {
            for (dx, value) in row.iter().enumerate() {
                // let is_in_bounds = x >= 0 && x < MAP_WIDTH && y >= 0 && y < MAP_HEIGHT;
                let is_in_bounds = x + (dx as i64) >= 0
                    && x + (dx as i64) < MAP_WIDTH
                    && y + (dy as i64) >= 0
                    && y + (dy as i64) < MAP_HEIGHT;

                if *value && (!is_in_bounds || self.get(x + dx as i64, y + dy as i64)) {
                    return true;
                }
            }
        }
        false
    }

    fn get_row(&self, y: i64) -> String {
        let mut row = String::new();
        for x in 0..MAP_WIDTH {
            row.push(if self.get(x, y) { '#' } else { '.' });
        }
        row
    }
}

struct Piece {
    data: Vec<Vec<bool>>,
    height: i64,
}

// Solve the Advent of Code 2022 Day 17 puzzle.
fn main() -> Result<(), Box<dyn Error>> {
    let pieces: [Piece; 5] = [
        // ####
        Piece {
            data: vec![vec![true, true, true, true]],
            height: 1,
        },
        // .#.
        // ###
        // .#.
        Piece {
            data: vec![
                vec![false, true, false],
                vec![true, true, true],
                vec![false, true, false],
            ],
            height: 3,
        },
        // ..#
        // ..#
        // ###
        Piece {
            data: vec![
                vec![true, true, true],
                vec![false, false, true],
                vec![false, false, true],
            ],
            height: 3,
        },
        // #
        // #
        // #
        // #
        Piece {
            data: vec![vec![true], vec![true], vec![true], vec![true]],
            height: 4,
        },
        // ##
        // ##
        Piece {
            data: vec![vec![true, true], vec![true, true]],
            height: 2,
        },
    ];

    let binding = stdin_lines().next().unwrap();
    let mut jets = binding
        .chars()
        .map(|c| if c == '<' { -1 as i64 } else { 1 as i64 })
        .enumerate()
        .cycle();

    let mut map = Map {
        data: [0; 131072],
        previous_draw: Vec::with_capacity(5),
    };
    let mut highest = 0;
    let mut highest_non_moving = 0;
    let mut piece_index = 0;
    let mut indices_map: HashMap<(usize, usize, String), i64> = HashMap::new();
    let mut first_cycle_index: Option<(usize, i64)> = None;
    let mut last_cycle_index: Option<(usize, i64)> = None;
    let mut skipped_height = 0;

    let mut n = 0;
    const TOTAL_CYCLES: usize = 1000000000000;
    // TODO: Uncomment this for part 1.
    // const TOTAL_CYCLES: usize = 2022;
    while n < TOTAL_CYCLES {
        let piece = &pieces[piece_index];
        let mut x = 2;
        let mut y = highest_non_moving + 3;
        map.draw_piece(x, y - skipped_height, piece);
        highest = y + piece.height;

        loop {
            let (jet_index, push) = jets.next().unwrap();

            map.clear_previous_piece();
            if !map.has_collision(&piece, x + push, y - skipped_height) {
                x += push;
                map.draw_piece(x, y - skipped_height, piece);
            } else {
                map.draw_piece(x, y - skipped_height, piece);
            }

            map.clear_previous_piece();
            if map.has_collision(&piece, x, y - 1 - skipped_height) {
                map.draw_piece(x, y - skipped_height, piece);
                highest_non_moving = max(highest_non_moving, highest);

                let last_row = map.get_row(highest_non_moving - 1 - skipped_height);
                let entry = indices_map
                    .entry((piece_index, jet_index, last_row))
                    .or_insert(0);
                *entry += 1;

                if *entry > 1 {
                    if first_cycle_index.is_none() {
                        first_cycle_index = Some((n, highest_non_moving));
                    } else {
                        if *entry == 3 && last_cycle_index.is_none() {
                            last_cycle_index = Some((n, highest_non_moving));

                            // Skip n by the cycle length times the maximum number of cycles we
                            // can.
                            let cycle_length =
                                last_cycle_index.unwrap().0 - first_cycle_index.unwrap().0;
                            let cycle_height =
                                last_cycle_index.unwrap().1 - first_cycle_index.unwrap().1;
                            let max_cycles = (TOTAL_CYCLES - n) / cycle_length;
                            n += cycle_length * max_cycles;
                            skipped_height = cycle_height * max_cycles as i64;
                            highest_non_moving += skipped_height;

                            println!("Skipping {} cycles", max_cycles);
                        }
                    }
                }

                break;
            } else {
                y -= 1;
                highest = max(highest_non_moving, highest - 1);
                map.draw_piece(x, y - skipped_height, piece);
            }
        }

        n += 1;
        piece_index = (piece_index + 1) % pieces.len();
    }

    println!("{}", highest_non_moving);

    Ok(())
}
