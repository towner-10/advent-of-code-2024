use std::env;
use std::fs;

fn check_line(line: &Vec<u32>) -> bool {
    if line.len() < 2 {
        return false;
    }

    let mut increasing = true;
    let mut last_value = line[0];

    if last_value > line[1] {
        increasing = false;
    }

    for i in 1..line.len() {
        // Do the faster check first
        if increasing && last_value > line[i] {
            return false;
        } else if !increasing && last_value < line[i] {
            return false;
        }

        let diff = last_value.abs_diff(line[i]);

        if diff == 0 || diff > 3 {
            return false;
        }

        last_value = line[i];
    }

    true
}

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

    let lines = contents.split("\r\n").map(|x| x.trim()).map(|x| {
        x.split(" ")
            .map(|y| y.parse::<u32>().expect("Not a number"))
            .collect()
    });

    // PART 1

    let part1_result = lines.clone().filter(|x| check_line(x)).count();
    println!("Part 1: {}", part1_result);

    // PART 2

    let part2_result = lines
        .clone()
        .filter(|x| {
            let checked = check_line(x);
            if !checked {
                for i in 0..x.len() {
                    let mut new_line = x.clone();
                    new_line.remove(i);

                    if check_line(&new_line) {
                        return true;
                    }
                }
            } else {
                return true;
            }

            false
        })
        .count();

    println!("Part 2: {}", part2_result);
}
