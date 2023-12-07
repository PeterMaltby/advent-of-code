use std::path::{Path, PathBuf};
use std::time::Instant;
mod utils;

const YEAR: &str = "2023";
const DAY: &str = "6";

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
        let mut line = lines.into_iter();

        let times = line.next().unwrap().unwrap();
        let mut times: Vec<f32> = times
            .split_whitespace()
            .skip(1)
            .map(|t| t.parse::<f32>().unwrap())
            .collect();

        let distance = line.next().unwrap().unwrap();
        let mut distance: Vec<f32> = distance
            .split_whitespace()
            .skip(1)
            .map(|t| t.parse::<f32>().unwrap())
            .collect();

        for i in 0..times.len() {
            //println!("{}h - h^2 > {}", times[i], distance[i]);
            //println!("{}h - h^2 - {} > 0", times[i], distance[i]);
            let b = times[i] / 2.0;
            let c = b * b - distance[i];
            //println!("-(h - {})^2 + {} > 0", b, c);
            //println!("(h - {})^2 < {}", b, c);

            //println!("- sqrt({}) < h - {} < sqrt({})", c, b, c);
            let lower = (-c.sqrt() + b).floor();
            let upper = (c.sqrt() + b).ceil();
            //println!("{} < h < {}", lower, upper);

            //println!("range: {}", upper - lower);

            if n == 0 {
                n += (upper - lower) as i32 - 1;
            } else {
                n *= (upper - lower) as i32 - 1;
            }

            //print!("\n");
        }
    }

    return n;
}

fn task2(file: &Path) -> i32 {
    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {
        let mut line = lines.into_iter();

        let time = line.next().unwrap().unwrap();
        let mut time: Vec<char> = time.chars().collect();
        time.retain(|c| c.is_numeric());
        let time = time.iter().cloned().collect::<String>();
        let time = time.parse::<f64>().unwrap();

        let distance = line.next().unwrap().unwrap();
        let mut distance: Vec<char> = distance.chars().collect();
        distance.retain(|c| c.is_numeric());
        //println!("{:?}", distance);
        let distance = distance.iter().cloned().collect::<String>();
        let distance = distance.parse::<f64>().unwrap();

        //println!("{}h - h^2 > {}", time, distance);
        //println!("{}h - h^2 - {} > 0", time, distance);
        let b = time / 2.0;
        let c = b * b - distance;
        //println!("-(h - {})^2 + {} > 0", b, c);
        //println!("(h - {})^2 < {}", b, c);

        //println!("- sqrt({}) < h - {} < sqrt({})", c, b, c);
        let lower = (-c.sqrt() + b).floor();
        let upper = (c.sqrt() + b).ceil();
        //println!("{} < h < {}", lower, upper);

        //println!("range: {}", upper - lower);

        n = (upper - lower) as i32 - 1;
    }

    return n;
}
