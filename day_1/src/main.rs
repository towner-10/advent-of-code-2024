use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    // --input <filename>
    if args.len() != 3 {
        eprintln!("Usage: {} --input <filename>", args[0]);
        std::process::exit(1);
    }

    if args[1] != "--input" {
        eprintln!("Usage: {} --input <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[2];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in lines {
        let pair: Vec<&str> = line.trim().split("   ").collect();
        let x1: u32 = pair[0].parse().unwrap();
        let y1: u32 = pair[1].parse().unwrap();

        left.push(x1);
        right.push(y1);
    }

    // PART 1

    left.sort();
    right.sort();

    let mut total_distance: u32 = 0;

    for pair in left.iter().zip(right.iter()) {
        let (x1, y1) = pair;
        total_distance += x1.abs_diff(*y1);
    }

    println!("Total distance: {}", total_distance);

    // PART 2

    let mut pairings: HashMap<u32, u32> = HashMap::new();

    for id in left.iter() {
        pairings.entry(*id).or_insert(0);
    }

    for id in right.iter() {
        pairings.entry(*id).and_modify(|e| *e += 1);
    }

    let mut similarity: u32 = 0;

    for (id, count) in pairings {
        similarity += id * count;
    }

    println!("Similarity: {}", similarity);
}
