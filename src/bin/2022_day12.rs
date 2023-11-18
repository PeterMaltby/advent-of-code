use std::path::{Path, PathBuf};
use std::time::Instant;
mod utils;

const YEAR: &str = "2022";
const DAY: &str = "12";

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

struct Map {
    map: Vec<Vec<u32>>,
    distances: Vec<Vec<u32>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
    fn new(file: &Path) -> Self {
        let mut map: Vec<Vec<u32>> = Vec::new();
        let mut distances: Vec<Vec<u32>> = Vec::new();
        let mut start: (usize, usize) = (0, 0);
        let mut end: (usize, usize) = (0, 0);
        if let Ok(lines) = utils::read_lines(file) {
            for (x, line) in lines.enumerate() {
                let line = line.unwrap();

                let mut row: Vec<u32> = Vec::new();
                let mut d_row: Vec<u32> = Vec::new();
                for (y, charecter) in line.chars().enumerate() {
                    d_row.push(std::u32::MAX);
                    match charecter {
                        'S' => {
                            row.push(1);
                            start = (x, y);
                        }
                        'E' => {
                            row.push(26);
                            end = (x, y);
                        }
                        'a'..='z' => row.push(charecter as u32 - 96),
                        _ => panic!("unknown value {}", charecter),
                    };
                }
                map.push(row);
                distances.push(d_row);
            }
        }

        return Map { map, distances, start, end };
    }

    fn print(&self) {
        let mut x: usize = 0;
        let mut y: usize = 0;

        for row in &self.map {
            for char in row {
                //println!("{:?} == {:?}",(x,y), self.start);
                if (x, y) == self.start {
                    print!("|SS");
                } else if (x, y) == self.end {
                    print!("|XX");
                } else if *char > 9 {
                    print!("|{}", char);
                } else {
                    print!("|0{}", char);
                };
                y += 1;
            }
            print!("|\n");
            for _ in 0..row.len() {
                print!("+--");
            }
            print!("|\n");
            x += 1;
            y =0;
        }
        println!("start: {:?}, end: {:?}", self.start, self.end);
    }

    fn print_distance(&self) {
        for row in &self.distances {
            for char in row {
                if *char > 99 {
                    print!("|9+");
                } else if *char > 9 {
                    print!("|{}", char);
                } else {
                    print!("|0{}", char);
                }
            }
            print!("|\n");
            for _ in 0..row.len() {
                print!("+--");
            }
            print!("|\n");
        }
    }

}

fn task1(file: &Path) -> i32 {
    let mut n = 0;

    let map: Map = Map::new(file);
    map.print();
    map.print_distance();

    return n;
}

fn task2(file: &Path) -> i32 {
    return 32;
}
