use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;
mod utils;

const YEAR: &str = "2023";
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

    //let start = Instant::now();
    //println!("task 2 attempt 1 answer: {}", task2_attempt1(&input_path));
    //println!("execution took: {}μs", start.elapsed().as_micros());
}

fn task1(file: &Path) -> u32 {
    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {
        for line in lines {
            let line = line.unwrap();
            let line = line.chars();
            let char: Vec<char> = line.filter(|c| ('0'..='9').contains(c)).collect();
            //println!("{:?}", char);

            let num = (char.first().unwrap().to_digit(10).unwrap() * 10)
                + char.last().unwrap().to_digit(10).unwrap();
            //println!("{}", num);
            n += num;
        }
    }

    return n;
}

fn task2(file: &Path) -> i32 {
    let translate: Vec<(&str, i32)> = vec![
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {
        for line in lines {
            let line = line.unwrap();
            //println!("{}", line);

            let mut start_index = 0;
            let mut first_digit = None;
            loop {
                for str in &translate {
                    let end_index = start_index + str.0.len();
                    //println!("{}..{}, line.len: {}", start_index, end_index, str.0.len());
                    if end_index > line.len() {
                        continue;
                    }
                    let test = &line[start_index..end_index];
                    //println!("{} == {}", str.0, test);
                    if *str.0 == line[start_index..end_index] {
                        first_digit = Some(str.1);
                    }
                }
                start_index += 1;

                match first_digit {
                    None => continue,
                    Some(_) => break,
                }
            }
            //println!("first digit: {}", first_digit.unwrap());

            let mut end_index = line.len();
            let mut last_digit = None;
            loop {
                for str in &translate {
                    let start_index = end_index.checked_sub(str.0.len());
                    match start_index {
                        None => continue,
                        Some(start_index) => {
                            if *str.0 == line[start_index..end_index] {
                                last_digit = Some(str.1);
                            }
                        }
                    }
                }
                end_index -= 1;

                match last_digit {
                    None => continue,
                    Some(_) => break,
                }
            }
            //println!("last digit: {}", last_digit.unwrap());

            n += first_digit.unwrap() * 10 + last_digit.unwrap();
        }
    }

    return n;
}

fn task2_attempt1(file: &Path) -> u32 {

    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {

        for line in lines {
            let line = line.unwrap();
            //println!("{}", line);
            let line = line.replace("one", "one1one");
            let line = line.replace("two", "two2two");
            let line = line.replace("three", "three3three");
            let line = line.replace("four", "four4four");
            let line = line.replace("five", "five5five");
            let line = line.replace("six", "six6six");
            let line = line.replace("seven", "seven7seven");
            let line = line.replace("eight", "eight8eight");
            let line = line.replace("nine", "nine9nine");
            let line = line.chars();
            let char: Vec<char>= line.filter(|c| ('0'..='9').contains(c)).collect();
            //println!("{:?}", char);

            let num = (char.first().unwrap().to_digit(10).unwrap() * 10) + char.last().unwrap().to_digit(10).unwrap();
            //println!("{}", num);
            n += num;
            //println!("{}",n);
        }
    }

    return n;
}

