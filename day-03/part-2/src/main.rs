#![feature(iter_array_chunks)]
use array_tool::vec::Intersect;
use std::error::Error;
use std::io;

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

fn map_char_to_value(c: char) -> i64 {
    match c {
        'a'..='z' => c as i64 - 'a' as i64 + 1,
        'A'..='Z' => c as i64 - 'A' as i64 + 1 + 26,
        _ => unreachable!(),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let result: i64 = stdin_lines()
        .array_chunks()
        .map(|[rucksack1, rucksack2, rucksack3]| {
            let vec1: Vec<char> = rucksack1.chars().collect();
            let vec2: Vec<char> = rucksack2.chars().collect();
            let vec3: Vec<char> = rucksack3.chars().collect();

            let intersection = vec1.intersect(vec2).intersect(vec3);
            map_char_to_value(intersection[0])
        })
        .sum();

    println!("{result}");

    Ok(())
}
