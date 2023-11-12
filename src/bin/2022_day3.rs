use std::path::{Path, PathBuf};
use std::time::Instant;
mod utils;

const YEAR: &str = "2022";
const DAY: &str = "3";

fn main() {
    println!("advent of code {}, day {}", YEAR, DAY);
    let mut input_path: PathBuf = Path::new("input").join(YEAR).join(DAY);
    input_path.set_extension("txt");

    println!("input: {}", input_path.display());

    let start = Instant::now();
    println!("task 1 answer: {}", task1(&input_path));
    println!("execution took: {}ms", start.elapsed().as_micros());

    let start = Instant::now();
    println!("task 2 answer: {}", task2(&input_path));
    println!("execution took: {}ms", start.elapsed().as_micros());
}

fn item_to_priority(i: &char) -> i32 {
    match i {
        'a'..='z' => return *i as i32 - 96,
        'A'..='Z' => return *i as i32 - 38,
        _ => panic!("not valid item"),
    }
}

fn find_match(compartments: Vec<&[char]>) -> i32 {
    for x in compartments[0] {
        for y in compartments[1] {
            if x == y {
                return item_to_priority(x)
            }
        }
    }

    panic!("shouldnt reach here");
    return 0;
}

fn task1(file: &Path) -> i32 {
    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {
        for line in lines {
            let var: Vec<char> = line.unwrap().chars().collect();
            let compartments: Vec<&[char]> = var.chunks(var.len() / 2).collect();
            n += find_match(compartments);
        }
    }

    return n;
}

fn find_tuple_match(l1: Vec<char>,l2: Vec<char>,l3: Vec<char>) -> i32 {
    for x in l1 {
        for y in &l2 {
            if x == *y {
                for z in &l3 {
                    if *z == *y {
                        return item_to_priority(z);
                    }
                }
            }
        }
    }

    panic!("shouldnt get here!");
    return 0;
}

fn task2(file: &Path) -> i32 {
    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {

        let mut line_iter = lines.into_iter();

        loop {
            let next = line_iter.next();
            let val1: Vec<char> = match next {
                Some(val) => val.unwrap().chars().collect(),
                _ => break,
            };
            let next = line_iter.next();
            let val2: Vec<char> = match next {
                Some(val) => val.unwrap().chars().collect(),
                _ => break,
            };
            let next = line_iter.next();
            let val3: Vec<char> = match next {
                Some(val) => val.unwrap().chars().collect(),
                _ => break,
            };

            n += find_tuple_match(val1, val2, val3);
        }
    }

    return n;
}
