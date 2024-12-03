use regex::Regex;
use std::env;
use std::fs;

const PART_1: &str = r"mul\((\d{1,3}),(\d{1,3})\)";
const PART_2: &str = r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)";

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

    // Part 1

    let part_1: u32 = Regex::new(PART_1)
        .unwrap()
        .captures_iter(&contents)
        .map(|capture| {
            let a: u32 = capture[1].parse().unwrap();
            let b: u32 = capture[2].parse().unwrap();
            a * b
        })
        .sum();

    println!("Part 1: {}", part_1);

    // Part 2

    let mut multiply = true;

    let part_2: u32 = Regex::new(PART_2)
        .unwrap()
        .captures_iter(&contents)
        .map(|capture| {
            if capture.get(0).unwrap().as_str() == "do()" {
                multiply = true;
                return 0;
            } else if capture.get(0).unwrap().as_str() == "don't()" {
                multiply = false;
                return 0;
            } else {
                let a: u32 = capture[1].parse().unwrap();
                let b: u32 = capture[2].parse().unwrap();
                if multiply {
                    return a * b;
                }
            }
            return 0;
        })
        .sum();

    println!("Part 2: {}", part_2);
}
