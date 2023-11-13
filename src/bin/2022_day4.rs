use std::path::{Path, PathBuf};
use std::time::Instant;
mod utils;

const YEAR: &str = "2022";
const DAY: &str = "4";

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

fn task1(file: &Path) -> i32 {
    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {
        for line in lines {
            let var = line.unwrap();

            let num_out: Vec<i32> = var
                .split(|c| c=='-' || c ==',')
                .map(|s| s.parse().unwrap())
                .collect();

            let x = num_out[0] - num_out[2];
            let y = num_out[1] - num_out[3];

            if x==0 || y == 0 { n+=1; continue; }

            match (x.is_positive(), y.is_positive()) {
                (true, false) => n+= 1,
                (false, true) => n+= 1,
                (_,_) => ()//dont care,
            }
        }
    }

    return n;
}

fn task2(file: &Path) -> i32 {
    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {
        for line in lines {
            let var = line.unwrap();

            let num_out: Vec<i32> = var
                .split(|c| c=='-' || c ==',')
                .map(|s| s.parse().unwrap())
                .collect();

            
            let x = (num_out[0],  num_out[1]);
            let y = (num_out[2],  num_out[3]);


            if !((x.0 > y.1 && x.0 > y.1) || (x.1 < y.1 && x.1 < y.0 )) { n+=1 };

        }
    }

    return n;
}
