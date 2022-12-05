use sscanf::sscanf;
use std::collections::HashMap;
use std::error::Error;
use std::io;

fn stdin_lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = stdin_lines();
    let mut directories: HashMap<String, i64> = HashMap::new();
    let mut path: Vec<String> = vec![];

    for line in lines {
        if line == "$ cd .." {
            let directory_size = directories.get(&path.join("/")).unwrap().clone();
            path.pop();
            *directories.get_mut(&path.join("/")).unwrap() += directory_size;

            continue;
        }

        if let Ok(dirname) = sscanf!(line, "$ cd {}", String) {
            path.push(dirname.clone());
            directories.insert(path.join("/"), 0);

            continue;
        }

        if let Ok((size, _filename)) = sscanf!(line, "{} {}", i64, String) {
            *directories.get_mut(&path.join("/")).unwrap() += size;

            continue;
        }
    }

    for _ in 0..path.len() - 1 {
        let directory_size = directories.get(&path.join("/")).unwrap().clone();
        path.pop();
        *directories.get_mut(&path.join("/")).unwrap() += directory_size;
    }

    let total = directories.get("/").unwrap().clone();
    let space_to_free = 30000000 - (70000000 - total);

    let mut sizes: Vec<i64> = directories.values().cloned().collect();
    sizes.sort();
    let result = sizes.iter().find(|&&x| x > space_to_free).unwrap();

    println!("{}", result);

    Ok(())
}
