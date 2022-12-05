use itertools::Itertools;
use sscanf::sscanf;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use thousands::Separable;

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

            println!(
                "$ cd .. \x1b[90m add {}({}) to \x1b[91m{}\x1b[0m",
                directory_size.separate_with_commas(),
                directories
                    .get(&path.join("/"))
                    .unwrap()
                    .separate_with_commas(),
                path.join("/")
            );

            continue;
        }

        if line == "$ ls" {
            println!("$ ls");
            continue;
        }

        if let Ok(dirname) = sscanf!(line, "$ cd {}", String) {
            path.push(dirname.clone());
            directories.insert(path.join("/"), 0);

            println!("$ cd {} \x1b[91m{}\x1b[0m", dirname, path.join("/"));

            continue;
        }

        if let Ok(dirname) = sscanf!(line, "dir {}", String) {
            println!("dir {}", dirname);

            continue;
        }

        if let Ok((size, filename)) = sscanf!(line, "{} {}", i64, String) {
            *directories.get_mut(&path.join("/")).unwrap() += size;

            println!(
                "{} {}\x1b[90m add {}({}) to \x1b[91m{}\x1b[0m",
                size,
                filename,
                size.separate_with_commas(),
                directories
                    .get(&path.join("/"))
                    .unwrap()
                    .separate_with_commas(),
                path.join("/")
            );

            continue;
        }
    }

    println!("----------------");

    for _ in 0..path.len() - 1 {
        let directory_size = directories.get(&path.join("/")).unwrap().clone();
        path.pop();
        *directories.get_mut(&path.join("/")).unwrap() += directory_size;

        println!(
            "$ cd .. \x1b[90m add {} to \x1b[91m{}\x1b[0m",
            directory_size.separate_with_commas(),
            path.join("/")
        );
    }

    println!("=================");

    let total = directories.get("/").unwrap().clone();
    let space_to_free = 30000000 - (70000000 - total);

    println!("");
    for (dir, size) in directories.iter().sorted() {
        println!("{:>10} {}", (*size).separate_with_commas(), dir);
    }
    println!("");
    for (dir, size) in directories.iter().sorted_by(|a, b| b.1.cmp(a.1)) {
        println!("{:>10} {}", (*size).separate_with_commas(), dir);
    }

    println!("");
    println!("  = total: {}", total.separate_with_commas());
    println!(
        "  = space to free: {}",
        space_to_free.separate_with_commas()
    );
    println!("  = path: {:?}", path);
    println!("");

    let mut sizes: Vec<i64> = directories.values().cloned().collect();
    sizes.sort();
    let result = sizes.iter().find(|&&x| x > space_to_free).unwrap();

    println!("  = result: {}", result);

    Ok(())
}
