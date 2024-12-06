use std::{env, fs};

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone, Copy)]
struct GridPoint {
    traversed: bool,
    obstacle: bool,
    possible_infinite_loop: bool,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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

    let mut grid: Vec<Vec<GridPoint>> = Vec::new();
    let mut start = Point { x: 0, y: 0 };
    let mut direction = Direction::Up;

    let mut x = 0;
    let mut y = 0;

    for line in contents.lines() {
        let mut row: Vec<GridPoint> = Vec::new();
        for c in line.chars() {
            match c {
                '^' => {
                    row.push(GridPoint {
                        traversed: true,
                        obstacle: false,
                        possible_infinite_loop: false,
                    });
                    start = Point { x, y };
                    direction = Direction::Up;
                }
                'v' => {
                    row.push(GridPoint {
                        traversed: true,
                        obstacle: false,
                        possible_infinite_loop: false,
                    });
                    start = Point { x, y };
                    direction = Direction::Down;
                }
                '<' => {
                    row.push(GridPoint {
                        traversed: true,
                        obstacle: false,
                        possible_infinite_loop: false,
                    });
                    start = Point { x, y };
                    direction = Direction::Left;
                }
                '>' => {
                    row.push(GridPoint {
                        traversed: true,
                        obstacle: false,
                        possible_infinite_loop: false,
                    });
                    start = Point { x, y };
                    direction = Direction::Right;
                }
                '#' => {
                    row.push(GridPoint {
                        traversed: false,
                        obstacle: true,
                        possible_infinite_loop: false,
                    });
                }
                '.' => {
                    row.push(GridPoint {
                        traversed: false,
                        obstacle: false,
                        possible_infinite_loop: false,
                    });
                }
                _ => {
                    eprintln!("Invalid character in input file: {}", c);
                    std::process::exit(1);
                }
            }

            x += 1;
        }

        grid.push(row);
        y += 1;
        x = 0;
    }

    let mut current = start;

    loop {
        let mut hit_edge = false;

        match direction {
            Direction::Up => {
                if current.y == 0 {
                    hit_edge = true;
                } else {
                    current.y -= 1;

                    if current.x < grid[current.y as usize].len() as u32
                        && grid[current.y as usize][(current.x + 1) as usize].traversed
                    {
                        grid[current.y as usize][current.x as usize].possible_infinite_loop = true;
                    }
                }
            }
            Direction::Down => {
                if current.y == grid.len() as u32 - 1 {
                    hit_edge = true;
                } else {
                    current.y += 1;

                    if current.x < grid[current.y as usize].len() as u32
                        && grid[current.y as usize][(current.x - 1) as usize].traversed
                    {
                        grid[current.y as usize][current.x as usize].possible_infinite_loop = true;
                    }
                }
            }
            Direction::Left => {
                if current.x == 0 {
                    hit_edge = true;
                } else {
                    current.x -= 1;

                    if current.y < grid.len() as u32
                        && grid[(current.y - 1) as usize][current.x as usize].traversed
                    {
                        grid[current.y as usize][current.x as usize].possible_infinite_loop = true;
                    }
                }
            }
            Direction::Right => {
                if current.x == grid[0].len() as u32 - 1 {
                    hit_edge = true;
                } else {
                    current.x += 1;

                    if current.y < grid.len() as u32
                        && grid[(current.y + 1) as usize][current.x as usize].traversed
                    {
                        grid[current.y as usize][current.x as usize].possible_infinite_loop = true;
                    }
                }
            }
        }

        // Check if the next point is an obstacle
        if grid[current.y as usize][current.x as usize].obstacle {
            // Move back
            match direction {
                Direction::Up => {
                    current.y += 1;
                }
                Direction::Down => {
                    current.y -= 1;
                }
                Direction::Left => {
                    current.x += 1;
                }
                Direction::Right => {
                    current.x -= 1;
                }
            }

            match direction {
                Direction::Up => {
                    direction = Direction::Right;
                }
                Direction::Down => {
                    direction = Direction::Left;
                }
                Direction::Left => {
                    direction = Direction::Up;
                }
                Direction::Right => {
                    direction = Direction::Down;
                }
            }
        } else {
            grid[current.y as usize][current.x as usize].traversed = true;
        }

        if hit_edge {
            break;
        }
    }

    let mut part_1 = 0;
    let mut part_2 = 0;

    for row in &grid {
        for point in row {
            if point.traversed {
                print!("X");
                part_1 += 1;
            } else if point.possible_infinite_loop {
                print!("O");
                part_2 += 1;
            } else if point.obstacle {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();

    }

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
