use regex::Regex;
use std::{cmp::Ordering, collections::HashMap, env, fs};

const MAP_REGEX: &str = r"([\d]+)\|([\d]+)";
const UPDATE_REGEX: &str = r"(\d+?)(?:,|$)";

fn check_valid(page_map: &HashMap<u16, Vec<u16>>, update: &Vec<u16>) -> bool {
    return update.is_sorted_by(|a, b| {
        if !page_map.contains_key(a) {
            return false;
        }

        let page = page_map.get(a).unwrap();

        if !page.contains(b) {
            return false;
        }

        return true;
    });
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

    let mut page_map: HashMap<u16, Vec<u16>> = HashMap::new();
    let map_re = Regex::new(MAP_REGEX).unwrap();

    let mut updates: Vec<Vec<u16>> = Vec::new();
    let update_re = Regex::new(UPDATE_REGEX).unwrap();

    for line in contents.lines() {
        if map_re.is_match(line) {
            let caps = map_re.captures(line).unwrap();
            let key = caps[1].parse::<u16>().unwrap();
            let value = caps[2].parse::<u16>().unwrap();

            if page_map.contains_key(&key) {
                let pages = page_map.get_mut(&key).unwrap();
                pages.push(value);
            } else {
                let mut pages: Vec<u16> = Vec::new();
                pages.push(value);
                page_map.insert(key, pages);
            }
        } else if update_re.is_match(line) {
            let mut update: Vec<u16> = Vec::new();
            for cap in update_re.captures_iter(line) {
                update.push(cap[1].parse::<u16>().unwrap());
            }
            updates.push(update);
        }
    }

    let mut part_1 = 0;
    let mut part_2 = 0;

    for update in updates {
        if !check_valid(&page_map, &update) {
            let mut sorted = update.clone();

            sorted.sort_by(|a, b| {
                if !page_map.contains_key(a) {
                    return Ordering::Greater;
                }

                let page = page_map.get(a).unwrap();

                if !page.contains(b) {
                    return Ordering::Greater;
                }

                return Ordering::Less;
            });

            part_2 += sorted[(sorted.len() as f64 / 2.0).floor() as usize] as u32;
        } else {
            part_1 += update[(update.len() as f64 / 2.0).floor() as usize] as u32;
        }
    }

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
