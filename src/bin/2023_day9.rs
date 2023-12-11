use std::path::{Path, PathBuf};
use std::time::Instant;
mod utils;

const YEAR: &str = "2023";
const DAY: &str = "9";

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
    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {
        for line in lines {
            let line = line.unwrap();
            let mut v: Vec<Vec<i32>> = vec![line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()];

            let mut depth = v.len() - 1;
            let mut length = v[depth].len() - 1;

            loop {
                if v[depth][length] == 0 && v[depth][length - 1] == 0 {
                    break;
                }

                let mut tmp: Vec<i32> = Vec::new();
                for i in 1..=length {
                    tmp.push(v[depth][i] - v[depth][i - 1]);
                }
                v.push(tmp);

                depth += 1;
                length -= 1;
            }

            //for i in &v {
            //    //println!("{:?}", i);
            //}

            for _ in 0..depth {
                depth -= 1;
                length += 1;
                n += v[depth][length];
                //print!("{}, ", v[depth][length]);
            }
            //print!("\n\n");
        }
    }

    return n;
}

fn task2(file: &Path) -> i32 {
    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {
        for line in lines {
            let line = line.unwrap();
            let mut v: Vec<Vec<i32>> = vec![line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()];

            let mut depth = v.len() - 1;
            let mut length = v[depth].len() - 1;

            loop {
                if v[depth][length] == 0 && v[depth][length - 1] == 0 {
                    break;
                }

                let mut tmp: Vec<i32> = Vec::new();
                for i in 1..=length {
                    tmp.push(v[depth][i] - v[depth][i - 1]);
                }
                v.push(tmp);

                depth += 1;
                length -= 1;
            }

            //for i in &v {
            //    //println!("{:?}", i);
            //}

            let mut sum = 0;
            for _ in 0..depth {
                depth -= 1;
                sum = v[depth][0] - sum;
                //print!("{} ({}), ", v[depth][0], sum);
            }
            //print!("\n\n");

            n+=sum;
        }
    }

    return n;
}
