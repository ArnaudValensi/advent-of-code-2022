use sscanf::sscanf;
use std::error::Error;
use std::io;

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

fn main() -> Result<(), Box<dyn Error>> {
    let result: i64 = stdin_lines().fold(0, |acc, line| {
        let (x_min, x_max, y_min, y_max) = sscanf!(line, "{i64}-{i64},{i64}-{i64}").unwrap();

        if (y_min >= x_min && y_max <= x_max) || (x_min >= y_min && x_max <= y_max) {
            return acc + 1;
        }
        return acc;
    });

    println!("{result}");

    Ok(())
}
