use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher, Hash};
use std::path::{Path, PathBuf};
use std::time::Instant;
mod utils;

const YEAR: &str = "2023";
const DAY: &str = "15";

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

fn task1(file: &Path) -> u32 {
    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {
        for line in lines {
            let line = line.unwrap();
            let line = line.split(',');

            for s in line {
                //println!("{}, ", s);
                n += HASHMAP(s);
            }
        }
    }

    return n;
}

fn HASHMAP(s: &str) -> u32 {
    let mut tmp = 0;
    for c in s.chars() {
        let c = c as u32;
        //println!("{}", c);
        tmp += c;
        tmp *= 17;
        tmp %= 256;
    }
    return tmp;
}

#[derive(Clone, Debug)]
struct Lens {
    id: String,
    value: u32,
}

fn task2(file: &Path) -> u32 {
    let mut lens_boxes: Vec<Vec<Lens>> = vec![Vec::new() ;256];
    let mut n = 0;

    if let Ok(lines) = utils::read_lines(file) {
        for line in lines {
            let line = line.unwrap();
            let line = line.split(',');

            for s in line {
                if let Some((id, op)) = s.split_once('=') {
                    let boxid = HASHMAP(id) as usize;
                    let op = op.parse::<u32>().unwrap();

                    match lens_boxes[boxid].iter().position(|lb| lb.id == id) {
                        None => lens_boxes[boxid].push( Lens { id: id.to_string(), value: op }),
                        Some(i) => lens_boxes[boxid][i].value = op,
                    }

                } else {
                    let id = &s[0..s.len() - 1];
                    let boxid = HASHMAP(id) as usize;

                    match lens_boxes[boxid].iter().position(|lb| lb.id == id) {
                        None => (),
                        Some(i) => { lens_boxes[boxid].remove(i); },
                    }
                }
            }

            let mut box_n = 1;
            for lenses in &lens_boxes {
                //println!("{:?}", lenses);
                let mut slot_n = 1;
                for lens in lenses {
                    n += box_n * slot_n * lens.value;
                    slot_n += 1;
                }
                box_n += 1;
            }
        }
    }

    return n;
}
