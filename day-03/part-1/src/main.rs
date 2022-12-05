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
        .map(|line| {
            let (half1, half2) = line.split_at(line.len() / 2);
            let vec1: Vec<char> = half1.chars().collect();
            let vec2: Vec<char> = half2.chars().collect();
            let intersection = vec1.intersect(vec2);

            map_char_to_value(intersection[0])
        })
        .sum();

    println!("{result}");

    Ok(())
}
