use std::path::{Path, PathBuf};
use std::time::Instant;

use utils::read_lines;
mod utils;

const YEAR: &str = "2022";
const DAY: &str = "5";

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

pub struct Stacks {
    stack: [Vec<char>; 9],
}

impl Stacks {
    pub fn new(input: &Path) -> Self {
        let mut stacks: [Vec<char>; 9] = Default::default();
        if let Ok(lines) = read_lines(input) {
            for line in lines {
                let line = line.unwrap();
                if line.is_empty() {
                    break;
                };

                let mut iter = line.chars();
                iter.next();
                for (index, charecter) in iter.step_by(4).enumerate() {
                    if charecter != ' ' {
                        stacks[index].push(charecter)
                    };
                }
            }
        }

        for x in 0..stacks.len() {
            stacks[x].reverse();
        }

        return Stacks { stack: stacks };
    }

    pub fn do_instruct(&mut self, from: usize, to: usize, n: usize) {
        for _ in 0..n {
            let tmp = self.stack[from].pop().unwrap();
            self.stack[to].push(tmp);
        }
    }

    pub fn do_instruct2(&mut self, from: usize, to: usize, n: usize) {
        let mut tmp: Vec<char> = Vec::new();
        for _ in 0..n {
            tmp.push(self.stack[from].pop().unwrap());
        }
        for _ in 0..n {
            self.stack[to].push(tmp.pop().unwrap());
        }
    }

    pub fn get_ans(&self) -> String {
        let mut str: String = String::new();
        for s in &self.stack {
            let tmp = s.last().unwrap().clone();
            str.push(tmp);
        }

        return str;
    }
}

fn task1(file: &Path) -> String {
    let mut stacks = Stacks::new(file);

    if let Ok(lines) = read_lines(file) {
        let mut line = lines.into_iter();
        loop {
            let line = line.next().unwrap().unwrap();
            if line.is_empty() {
                break;
            }
        }

        for line in line {
            let line = line.unwrap();
            let v: Vec<&str> = line.split(|c| c == ' ').collect();
            stacks.do_instruct(
                (v[3].parse::<usize>().unwrap() - 1),
                (v[5].parse::<usize>().unwrap() - 1),
                v[1].parse().unwrap(),
            );
        }
    }

    return stacks.get_ans();
}

fn task2(file: &Path) -> String {
    let mut stacks = Stacks::new(file);

    if let Ok(lines) = read_lines(file) {
        let mut line = lines.into_iter();
        loop {
            let line = line.next().unwrap().unwrap();
            if line.is_empty() {
                break;
            }
        }

        for line in line {
            let line = line.unwrap();
            let v: Vec<&str> = line.split(|c| c == ' ').collect();
            stacks.do_instruct2(
                (v[3].parse::<usize>().unwrap() - 1),
                (v[5].parse::<usize>().unwrap() - 1),
                v[1].parse().unwrap(),
            );
        }
    }

    return stacks.get_ans();
}
