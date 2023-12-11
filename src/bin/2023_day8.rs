use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::usize;
mod utils;

const YEAR: &str = "2023";
const DAY: &str = "8";

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

enum Dir {
    Left,
    Right,
}

struct Map {
    map: HashMap<u32, (u32, u32)>,
    instruct: Vec<Dir>,
    ghost_start: Vec<u32>,
    ghost_end: Vec<u32>,
}

fn str_to_id(s: &str) -> u32 {
    let mut n: u32 = 0;
    let base: u32 = 26;
    for (i, c) in s.chars().enumerate() {
        n += (c as u32 - 65) * base.pow(i as u32) ;
    }

    return n;
}

impl Map {
    fn new(file: &Path) -> Self {
        if let Ok(lines) = utils::read_lines(file) {
            let mut lines = lines.into_iter();
            let line = lines.next().unwrap().unwrap();

            let instruct = line
                .chars()
                .map(|c| match c {
                    'L' => Dir::Left,
                    'R' => Dir::Right,
                    _ => !panic!("invalid char {}", c),
                })
                .collect();

            lines.next();
            let mut map: HashMap<u32, (u32, u32)> = HashMap::new();
            let mut ghost_start: Vec<u32> = Vec::new();
            let mut ghost_end: Vec<u32> = Vec::new();
            loop {
                let line = lines.next();
                if line.is_none() { break; }
                let line = line.unwrap().unwrap();
                
                let tmpid = str_to_id(&line[0..3]);
                map.insert(tmpid, (str_to_id(&line[7..10]), str_to_id(&line[12..15])));
                if &line[2..3] == "A" { 
                    ghost_start.push(tmpid);  
                    //println!("{}", &line[0..3]); 
                }
                if &line[2..3] == "Z" { 
                    ghost_end.push(tmpid); 
                    //println!("{}", &line[0..3]); 
                }
            }

            return Map { map, instruct, ghost_start, ghost_end };
        } else {
            panic!("cannot read file");
        }
    }

    fn solve(&self, target: &str) -> u32 {
        let mut current_pos: u32 = str_to_id("AAA"); // AAA = 0;
        let target: u32 = str_to_id(target);
        let mut n = 0;
                                                 
        loop {
            for i in &self.instruct {
                n+=1;
                match *i {
                    Dir::Left => current_pos = self.map.get(&current_pos).unwrap().0,
                    Dir::Right => current_pos = self.map.get(&current_pos).unwrap().1,
                }
                if current_pos == target { return n; }
            }
        }
    }

    fn solve_2(&self) -> u64 {
        let mut current_pos: Vec<u32> = self.ghost_start.clone();
        let mut end_points_dist: Vec<u32> = Vec::new();

        let mut n = 0;

        loop {
            for i in &self.instruct {
                n+=1;
                current_pos = current_pos.iter().map(|pos| 
                    match *i {
                    Dir::Left => self.map.get(&pos).unwrap().0,
                    Dir::Right => self.map.get(&pos).unwrap().1,
                    }
                ).collect();

                current_pos.retain(|pos| {
                    if self.ghost_end.contains(&pos) {
                        end_points_dist.push(n);
                        return false;
                    } else {
                        return true;
                    }
                });

                if current_pos.is_empty() {
                    //println!("{:?}", end_points_dist);

                    let mut lcm = end_points_dist.pop().unwrap() as u64;
                    while let Some(val) = end_points_dist.pop() {
                        lcm = find_lcm(lcm, val as u64);
                    }


                    return lcm;
                }
            }
        }
    }
}

fn find_gcd(a: u64, b: u64) -> u64 {
    if a == b { return a; }

    let mut lowest = if a > b { b } else { a };
    let mut highest = if a < b { b } else { a };

    while lowest > 0 {
        let remainder = highest % lowest;
        highest = lowest;
        lowest = remainder;
    }
    return highest;
}

fn find_lcm(a: u64, b: u64) -> u64 {
    return a * (b / find_gcd(a, b) );
}

fn task1(file: &Path) -> u32 {
    let map: Map = Map::new(file);

    return map.solve("ZZZ");
}

fn task2(file: &Path) -> u64 {
    let map: Map = Map::new(file);

    return map.solve_2();
}
