use std::time::Instant;
use std::path::{PathBuf, Path};
mod utils;

const YEAR: &str = "2022";
const DAY: &str = "1";

fn main() {
    println!("advent of code {}, day {}", YEAR, DAY);
    let mut input_path: PathBuf = Path::new("input").join(YEAR).join(DAY);
    input_path.set_extension("txt");

    println!("input: {}", input_path.display());

    let start = Instant::now();
    println!("task 1 answer: {}", task1(&input_path));
    println!("execution took: {}μs", start.elapsed().as_micros());

    let start = Instant::now();
    println!("task 2 answer: {}", task2(&input_path));
    println!("execution took: {}μs", start.elapsed().as_micros());

}

fn task1(file: &Path) -> i32 {

    let mut count: Vec<i32> = Vec::new();
    if let Ok(lines) = utils::read_lines(file) {

        let mut tmp = 0;
        for line in lines {
            if let Ok(val) = line {
                if val.is_empty() {
                    count.push(tmp);
                    tmp = 0;
                    continue;
                }
                tmp += val.parse::<i32>().unwrap();
            }

        }
    }

    count.sort();
    return *count.last().unwrap();
}

fn task2(file: &Path) -> i32 {

    let mut count: Vec<i32> = Vec::new();
    if let Ok(lines) = utils::read_lines(file) {

        let mut tmp = 0;
        for line in lines {
            if let Ok(val) = line {
                if val.is_empty() {
                    count.push(tmp);
                    tmp = 0;
                    continue;
                }
                tmp += val.parse::<i32>().unwrap();
            }

        }
    }

    count.sort();
    return count.iter().rev().take(3).sum();
}
