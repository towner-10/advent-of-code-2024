use std::env;
use std::fs;

const WORD: &str = "XMAS";

#[derive(Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

fn update_point(point: &mut Point, direction: &Direction) {
    match direction {
        Direction::Up => {
            point.y -= 1;
        }
        Direction::Down => {
            point.y += 1;
        }
        Direction::Left => {
            point.x -= 1;
        }
        Direction::Right => {
            point.x += 1;
        }
        Direction::UpLeft => {
            point.x -= 1;
            point.y -= 1;
        }
        Direction::UpRight => {
            point.x += 1;
            point.y -= 1;
        }
        Direction::DownLeft => {
            point.x -= 1;
            point.y += 1;
        }
        Direction::DownRight => {
            point.x += 1;
            point.y += 1;
        }
    }
}

fn check_direction(
    grid: &Vec<Vec<char>>,
    point: &Point,
    direction: &Direction,
    letter: char,
) -> bool {
    match direction {
        Direction::Up => {
            if point.y == 0 {
                return false;
            }
            return grid[point.y - 1][point.x] == letter;
        }
        Direction::Down => {
            if point.y == grid.len() - 1 {
                return false;
            }
            return grid[point.y + 1][point.x] == letter;
        }
        Direction::Left => {
            if point.x == 0 {
                return false;
            }
            return grid[point.y][point.x - 1] == letter;
        }
        Direction::Right => {
            if point.x == grid[0].len() - 1 {
                return false;
            }
            return grid[point.y][point.x + 1] == letter;
        }
        Direction::UpLeft => {
            if point.y == 0 || point.x == 0 {
                return false;
            }
            return grid[point.y - 1][point.x - 1] == letter;
        }
        Direction::UpRight => {
            if point.y == 0 || point.x == grid[0].len() - 1 {
                return false;
            }
            return grid[point.y - 1][point.x + 1] == letter;
        }
        Direction::DownLeft => {
            if point.y == grid.len() - 1 || point.x == 0 {
                return false;
            }
            return grid[point.y + 1][point.x - 1] == letter;
        }
        Direction::DownRight => {
            if point.y == grid.len() - 1 || point.x == grid[0].len() - 1 {
                return false;
            }
            return grid[point.y + 1][point.x + 1] == letter;
        }
    }
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

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    let mut part_1 = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != WORD.chars().nth(0).unwrap() {
                continue;
            }

            let point = Point { x, y };

            // Check all directions
            for direction in vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
                Direction::UpLeft,
                Direction::UpRight,
                Direction::DownLeft,
                Direction::DownRight,
            ] {
                let mut letter_point = point.clone();
                let mut word_index = 1;
                let mut found_direction = false;

                while word_index < WORD.len() {
                    found_direction = check_direction(
                        &grid,
                        &letter_point,
                        &direction,
                        WORD.chars().nth(word_index).unwrap(),
                    );

                    if !found_direction {
                        break;
                    }

                    word_index += 1;
                    update_point(&mut letter_point, &direction);
                }

                if found_direction {
                    part_1 += 1;
                }
            }
        }
    }

    println!("Part 1: {}", part_1);

    let mut part_2 = 0;

    for y in 1..grid.len() - 1 {
        for x in 1..grid[y].len() - 1 {
            if grid[y][x] != 'A' {
                continue;
            }

            let mut found_left_right = false;
            let mut found_right_left = false;

            let point = Point { x, y };

            if check_direction(&grid, &point, &Direction::UpLeft, 'M')
                && check_direction(&grid, &point, &Direction::DownRight, 'S')
            {
                found_left_right = true;
            } else if check_direction(&grid, &point, &Direction::UpLeft, 'S')
                && check_direction(&grid, &point, &Direction::DownRight, 'M')
            {
                found_left_right = true;
            }

            if check_direction(&grid, &point, &Direction::UpRight, 'M')
                && check_direction(&grid, &point, &Direction::DownLeft, 'S')
            {
                found_right_left = true;
            } else if check_direction(&grid, &point, &Direction::UpRight, 'S')
                && check_direction(&grid, &point, &Direction::DownLeft, 'M')
            {
                found_right_left = true;
            }

            if found_left_right && found_right_left {
                part_2 += 1;
            }
        }
    }

    println!("Part 2: {}", part_2);
}
