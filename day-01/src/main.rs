#![feature(binary_heap_into_iter_sorted)]

use std::collections::BinaryHeap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("./input1.txt")?;

    let result: i64 = text
        .split("\n\n")
        .map(|elve_bag| {
            elve_bag
                .split_whitespace()
                .fold(0, |acc, calory| acc + calory.parse::<i64>().unwrap())
        })
        .collect::<BinaryHeap<i64>>()
        .into_iter_sorted()
        .take(3)
        .sum();

    println!("{result:#?}");

    Ok(())
}
