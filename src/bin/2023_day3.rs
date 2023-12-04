use std::fmt::{self, Display};
use std::io::Write;
use std::iter::zip;
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::usize;
mod utils;

const YEAR: &str = "2023";
const DAY: &str = "3";

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

struct Schematic {
    map: Vec<Vec<char>>,
    part_numbers: Vec<u32>,
    gear: Vec<( u32, (usize,usize))>,
}

impl Schematic {
    fn new(file: &Path) -> Self {
        let mut map: Vec<Vec<char>> = Vec::new();
        if let Ok(lines) = utils::read_lines(file) {
            for line in lines {
                let line = line.unwrap();
                map.push(line.chars().collect());
            }
        }

        let part_numbers: Vec<u32> = Vec::new();
        let gear: Vec<(u32, (usize,usize))> = Vec::new();
        return Schematic { map, part_numbers, gear };
    }

    fn find_part(&mut self) {
        for mut x in 0..self.map.len() {
            let mut y = 0;
            loop {
                match self.map[x][y] {
                    '0'..='9' => {
                        let mut part_number: u32 = self.map[x][y].to_digit(10).unwrap();

                        let check_area_1: (usize, usize) = (
                            match x.checked_sub(1) {
                                None => 0,
                                Some(x) => x,
                            },
                            match y.checked_sub(1) {
                                None => 0,
                                Some(y) => y,
                            },
                        );

                        loop {
                            y += 1;
                            if y >= self.map[0].len() {
                                break;
                            }
                            match self.map[x][y] {
                                '0'..='9' => {
                                    part_number =
                                        part_number * 10 + self.map[x][y].to_digit(10).unwrap();
                                }
                                _ => break,
                            }
                        }

                        let check_area_2: (usize, usize) = (
                            if x + 1 >= self.map.len() { x } else { x + 1 },
                            if y >= self.map.len() { self.map.len() - 1 } else { y },
                        );

                        println!("found: {} - with check area: ({:?}, {:?}) ", part_number, check_area_1, check_area_2);

                        for x in check_area_1.0..=check_area_2.0 {
                            for y in check_area_1.1..=check_area_2.1 {
                                match self.map[x][y] {
                                    '.' => continue,
                                    '0'..='9' => continue,
                                    '!'..='/' => self.part_numbers.push(part_number),
                                    ':'..='@' => self.part_numbers.push(part_number),
                                    _ => panic!("unknown char: {}", self.map[x][y]),

                                }
                            }
                        }


                    }
                    _ => (),
                }
                y += 1;
                if y >= self.map[0].len() {
                    break;
                }
            }
        }
    }

    fn find_gear(&mut self) {
        for mut x in 0..self.map.len() {
            let mut y = 0;
            loop {
                match self.map[x][y] {
                    '0'..='9' => {
                        let mut part_number: u32 = self.map[x][y].to_digit(10).unwrap();

                        let check_area_1: (usize, usize) = (
                            match x.checked_sub(1) {
                                None => 0,
                                Some(x) => x,
                            },
                            match y.checked_sub(1) {
                                None => 0,
                                Some(y) => y,
                            },
                        );

                        loop {
                            y += 1;
                            if y >= self.map[0].len() {
                                break;
                            }
                            match self.map[x][y] {
                                '0'..='9' => {
                                    part_number =
                                        part_number * 10 + self.map[x][y].to_digit(10).unwrap();
                                }
                                _ => break,
                            }
                        }

                        let check_area_2: (usize, usize) = (
                            if x + 1 >= self.map.len() { x } else { x + 1 },
                            if y >= self.map.len() { self.map.len() - 1 } else { y },
                        );

                        println!("found: {} - with check area: ({:?}, {:?}) ", part_number, check_area_1, check_area_2);

                        for x in check_area_1.0..=check_area_2.0 {
                            for y in check_area_1.1..=check_area_2.1 {
                                match self.map[x][y] {
                                    '.' => continue,
                                    '0'..='9' => continue,
                                    '*' => { self.gear.push((part_number, (x,y))); },
                                    _ => (),

                                }
                            }
                        }


                    }
                    _ => (),
                }
                y += 1;
                if y >= self.map[0].len() {
                    break;
                }
            }
        }
    }

    fn cal_total(&self) -> u32 {
        return self.part_numbers.iter().sum();
    }

    fn cal_gear_ratios(&self) -> u32 {
        let mut n = 0;

        for item in &self.gear {
            for item_to_check in &self.gear {
                println!("gear pair {}: {:?}, {}: {:?}", item.0, item.1, item_to_check.0, item_to_check.1);
                if (item.1 == item_to_check.1 && item.0 != item_to_check.0) {
                    println!("found gear pair {}: {:?}, {}: {:?}", item.0, item.1, item_to_check.0, item_to_check.1);
                    n += (item.0 * item_to_check.0);
                }
            }
        }

        return n;
    }

}

impl Display for Schematic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for l in &self.map {
            for c in l {
                write!(f, "{}", c);
            }
            write!(f, "\n");
        }
        return write!(f, "");
    }
}

fn task1(file: &Path) -> u32 {
    let mut schematic = Schematic::new(file);

    println!("{}", schematic);

    schematic.find_part();

    return schematic.cal_total();
}

fn task2(file: &Path) -> u32 {
    let mut schematic = Schematic::new(file);

    println!("{}", schematic);

    schematic.find_gear();

    return schematic.cal_gear_ratios() / 2;

}
