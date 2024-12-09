use std::env;
use std::fs;

struct Equation {
    result: u64,
    numbers: Vec<u64>,
}

fn is_valid_equation_part_1(equation: &Equation, accumulator: u64, index: usize) -> bool {
    if index == equation.numbers.len() {
        return equation.result == accumulator;
    }

    let num = equation.numbers[index];
    is_valid_equation_part_1(equation, accumulator + num, index + 1)
        || is_valid_equation_part_1(equation, accumulator * num, index + 1)
}

fn is_valid_equation_part_2(equation: &Equation, accumulator: u64, index: usize) -> bool {
    if index == equation.numbers.len() {
        return equation.result == accumulator;
    }

    let num = equation.numbers[index];

    let concatenated = format!("{}{}", accumulator, num).parse::<u64>().unwrap();

    is_valid_equation_part_2(equation, accumulator + num, index + 1)
        || is_valid_equation_part_2(
            equation,
            if index == 0 { 0 } else { accumulator } * num,
            index + 1,
        )
        || is_valid_equation_part_2(equation, concatenated, index + 1)
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

    let equations: Vec<Equation> = contents
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();

            Equation {
                result: parts[0].trim().parse::<u64>().unwrap(),
                numbers: parts[1]
                    .trim()
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
            }
        })
        .collect();

    let part_1: u64 = equations
        .iter()
        .filter_map(|equation| {
            is_valid_equation_part_1(&equation, equation.numbers[0], 1).then_some(equation.result)
        })
        .sum();

    let part_2: u64 = equations
        .iter()
        .filter_map(|equation| {
            is_valid_equation_part_2(&equation, equation.numbers[0], 1).then_some(equation.result)
        })
        .sum();

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
