use sscanf::sscanf;
use std::error::Error;
use std::io;

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = stdin_lines();
    let mut sum: i64 = 0;
    let mut levels: Vec<i64> = vec![0];

    for line in lines {
        if line == "$ cd .." {
            if let Some(size) = levels.pop() {
                if size <= 100000 {
                    sum += size;
                }

                *levels.last_mut().unwrap() += size;
            }
            continue;
        }

        if let Ok(_dirname) = sscanf!(line, "$ cd {}", String) {
            levels.push(0);
            continue;
        }

        if let Ok((size, _filename)) = sscanf!(line, "{} {}", i64, String) {
            *levels.last_mut().unwrap() += size;
            continue;
        }
    }

    println!("{}", sum);

    Ok(())
}
