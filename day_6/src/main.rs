use std::{collections::HashSet, env, fs, hash::Hash, ops};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct GridPoint {
    traversed: bool,
    infinite_loop: bool,
    obstacle: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

trait DirectionTrait {
    fn rotate_clockwise(&self) -> Direction;
}

impl DirectionTrait for Direction {
    fn rotate_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

trait PointTrait {
    fn move_direction(&self, direction: &Direction) -> Point;
    fn move_opposite(&self, direction: &Direction) -> Point;
}

impl PointTrait for Point {
    fn move_direction(&self, direction: &Direction) -> Point {
        match direction {
            Direction::Up => Point {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Point {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Point {
                x: self.x + 1,
                y: self.y,
            },
        }
    }

    fn move_opposite(&self, direction: &Direction) -> Point {
        match direction {
            Direction::Up => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Down => Point {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Left => Point {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Right => Point {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

impl ops::Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Point {
        self.move_direction(&rhs)
    }
}

impl ops::Sub<Direction> for Point {
    type Output = Point;

    fn sub(self, rhs: Direction) -> Point {
        self.move_opposite(&rhs)
    }
}

fn grid_contains(grid: &Vec<Vec<GridPoint>>, point: Point) -> bool {
    point.x < grid[0].len() as i32 && point.y < grid.len() as i32 && point.x >= 0 && point.y >= 0
}

// If it returns to the same point and direction, it's an infinite loop
fn has_cycle(grid: &Vec<Vec<GridPoint>>, point: Point, direction: Direction) -> bool {
    let mut seen: HashSet<(Point, Direction)> = HashSet::new();
    seen.insert((point, direction));

    let mut current = point;
    let mut current_direction = direction;

    while grid_contains(grid, current + current_direction) {
        let next = current + current_direction;

        if grid[next.y as usize][next.x as usize].obstacle {
            current_direction = current_direction.rotate_clockwise();
            continue;
        }

        if seen.contains(&(next, current_direction)) {
            return true;
        }

        seen.insert((next, current_direction));

        current = next;
    }

    false
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
                        infinite_loop: false,
                    });
                    start = Point { x, y };
                    direction = Direction::Up;
                }
                '#' => {
                    row.push(GridPoint {
                        traversed: false,
                        obstacle: true,
                        infinite_loop: false,
                    });
                }
                '.' => {
                    row.push(GridPoint {
                        traversed: false,
                        obstacle: false,
                        infinite_loop: false,
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

    while grid_contains(&grid, current + direction) {
        let next = current + direction;

        if grid[next.y as usize][next.x as usize].obstacle {
            direction = direction.rotate_clockwise();
            continue;
        }

        grid[next.y as usize][next.x as usize].obstacle = true;
        if !grid[next.y as usize][next.x as usize].infinite_loop {
            let cycle = has_cycle(&grid, current, direction);
            grid[next.y as usize][next.x as usize].infinite_loop = cycle;
        }
        grid[next.y as usize][next.x as usize].obstacle = false;

        grid[current.y as usize][current.x as usize].traversed = true;
        current = next;
    }

    let mut part_1 = 1;
    let mut part_2 = 0;

    for row in &grid {
        for point in row {
            if point.traversed {
                part_1 += 1;
            }
            if point.infinite_loop {
                part_2 += 1;
            }
        }
    }

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
