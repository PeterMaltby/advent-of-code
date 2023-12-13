use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::Instant;
mod utils;

const YEAR: &str = "2023";
const DAY: &str = "11";

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

#[derive(Debug)]
struct Coords {
    x: usize,
    y: usize,
}

struct Observation {
    galaxies: Vec<Coords>,
    expansion_x: Vec<usize>,
    expansion_y: Vec<usize>,
}

impl Observation {
    fn new(file: &Path) -> Self {
        let mut galaxies: Vec<Coords> = Vec::new();
        let mut expansion_y: Vec<usize> = Vec::new();
        let mut expansion_x: Vec<usize> = Vec::new();

        let mut tmp_x_len = 0;

        if let Ok(lines) = utils::read_lines(file) {
            for (y, line) in lines.enumerate() {
                let line = line.unwrap();

                //println!("{}", line);

                tmp_x_len = line.len();

                let tmp_count = galaxies.len();
                for (x, char) in line.chars().enumerate() {
                    match char {
                        '.' => (),
                        '#' => galaxies.push(Coords { x, y }),
                        x => panic!("unknown char: {}", x),
                    }
                }

                if galaxies.len() == tmp_count {
                    expansion_y.push(y)
                }
            }
        }

        let mut expansion_x_hash: HashSet<usize> = HashSet::new();
        for galaxy in &galaxies {
            expansion_x_hash.insert(galaxy.x);
        }

        for i in 0..tmp_x_len {
            if !expansion_x_hash.contains(&i) {
                expansion_x.push(i);
            }
        }

        //println!("Galaxies: {:?}", &galaxies);
        //println!("expansion_y: {:?},", &expansion_y);
        //println!("expansion_x: {:?}", &expansion_x);

        return Observation {
            galaxies,
            expansion_x,
            expansion_y,
        };
    }

    fn sum_all_length(&self) -> usize {
        //println!("total galaxies: {}", self.galaxies.len());
        let mut n = 0;
        for a in 0..self.galaxies.len()-1 {
            for b in a+1..self.galaxies.len() {
                n+=self.cal_distance(&self.galaxies[a], &self.galaxies[b]);
            }
        }
        return n;
    }

    fn cal_distance(&self, a: &Coords, b: &Coords) -> usize {
        //println!("{:?} -> {:?}", a, b);

        let lowest_x = if a.x > b.x { b.x } else { a.x };
        let highest_x = if a.x > b.x { a.x } else { b.x };
        let lowest_y = if a.y > b.y { b.y } else { a.y };
        let highest_y = if a.y > b.y { a.y } else { b.y };

        let mut modifier = 0;
        for x in &self.expansion_x {
            if *x > lowest_x && *x < highest_x { modifier += 1; }
        }
        for y in &self.expansion_y {
            if *y > lowest_y && *y < highest_y { modifier += 1; }
        }

        //println!("modifier: {}", modifier);

        let n = ((highest_x - lowest_x) + (highest_y - lowest_y)) + modifier;
        //println!("(({} - {}) + ({} - {})) + {} = {}", highest_x, lowest_x, highest_y, lowest_y, modifier, n);


        return n;
    }

    fn sum_all_length_million(&self) -> usize {
        let mut n = 0;
        for a in 0..self.galaxies.len()-1 {
            for b in a+1..self.galaxies.len() {
                n+=self.cal_distance_million(&self.galaxies[a], &self.galaxies[b]);
            }
        }
        return n;
    }

    fn cal_distance_million(&self, a: &Coords, b: &Coords) -> usize {
        let lowest_x = if a.x > b.x { b.x } else { a.x };
        let highest_x = if a.x > b.x { a.x } else { b.x };
        let lowest_y = if a.y > b.y { b.y } else { a.y };
        let highest_y = if a.y > b.y { a.y } else { b.y };

        let mut modifier = 0;
        for x in &self.expansion_x {
            if *x > lowest_x && *x < highest_x { modifier += 999_999; }
        }
        for y in &self.expansion_y {
            if *y > lowest_y && *y < highest_y { modifier += 999_999; }
        }

        let n = ((highest_x - lowest_x) + (highest_y - lowest_y)) + modifier;
        //println!("(({} - {}) + ({} - {})) + {} = {}", highest_y, lowest_x, highest_y, lowest_y, modifier, n);


        return n;
    }
}

fn task1(file: &Path) -> usize {
    let observation = Observation::new(file);
    return observation.sum_all_length();
}

fn task2(file: &Path) -> usize {
    let observation = Observation::new(file);
    return observation.sum_all_length_million();
}
