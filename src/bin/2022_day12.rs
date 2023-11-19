use std::collections::{self, VecDeque};
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
    max_x: usize,
    max_y: usize,
    first_a: Option<(usize, usize)>,
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

        return Map {
            max_x: map.len(),
            max_y: map[0].len(),
            map,
            distances,
            start,
            end,
            first_a: None,
        };
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
            y = 0;
        }
        println!("start: {:?}, end: {:?}", self.start, self.end);
    }

    fn print_distance(&self) {
        for row in &self.distances {
            for char in row {
                if *char > 999 {
                    print!("|99+");
                } else if *char > 99 {
                    print!("|{}", char);
                } else if *char > 9 {
                    print!("|0{}", char);
                } else {
                    print!("|00{}", char);
                }
            }
            print!("|\n");
            for _ in 0..row.len() {
                print!("+---");
            }
            print!("|\n");
        }
        print!("\n\n");
    }

    fn print_both(&self) {
        for x in 0..self.map.len() {
            for char in &self.map[x] {
                if *char > 999 {
                    print!("|99+");
                } else if *char > 99 {
                    print!("|{}", char);
                } else if *char > 9 {
                    print!("|0{}", char);
                } else {
                    print!("|00{}", char);
                }
            }
            print!("|\n");
            for char in &self.distances[x] {
                if *char > 999 {
                    print!("|99+");
                } else if *char > 99 {
                    print!("|{}", char);
                } else if *char > 9 {
                    print!("|0{}", char);
                } else {
                    print!("|00{}", char);
                }
            }
            print!("|\n");
            for _ in 0..self.distances[0].len() {
                print!("+---");
            }
            print!("|\n");
        }
        println!("\n");
    }

    fn is_valid(&self, x: usize, y: usize, height: u32) -> bool {
        if x >= self.max_x || y >= self.max_y {
            return false;
        };
        let this_height = self.map[x][y];
        if self.distances[x][y] != std::u32::MAX {
            return false;
        };
        if this_height <= height + 1 {
            return true;
        };
        return false;
    }

    fn is_valid2(&mut self, x: usize, y: usize, height: u32) -> bool {
        if x >= self.max_x || y >= self.max_y {
            return false;
        };
        let this_height = self.map[x][y];

        if self.distances[x][y] != std::u32::MAX {
            return false;
        };
        if this_height >= height - 1 {
            if this_height == 1 && self.first_a == None {
                self.first_a = Some((x, y));
            }

            return true;
        };
        return false;
    }

    fn solve_distances(&mut self) {
        let mut to_solve: std::collections::VecDeque<(usize, usize)> = VecDeque::new();

        self.distances[self.start.0][self.start.1] = 0;
        to_solve.push_back(self.start);

        loop {
            let coords = to_solve.pop_front();
            match coords {
                None => break,
                Some((x, y)) => {
                    let height = self.map[x][y];
                    let distance = self.distances[x][y];

                    if x != 0 {
                        if self.is_valid(x - 1, y, height) {
                            self.distances[x - 1][y] = distance + 1;
                            to_solve.push_back((x - 1, y));
                        }
                    }
                    if self.is_valid(x + 1, y, height) {
                        self.distances[x + 1][y] = distance + 1;
                        to_solve.push_back((x + 1, y));
                    }
                    if y != 0 {
                        if self.is_valid(x, y - 1, height) {
                            self.distances[x][y - 1] = distance + 1;
                            to_solve.push_back((x, y - 1));
                        }
                    }
                    if self.is_valid(x, y + 1, height) {
                        self.distances[x][y + 1] = distance + 1;
                        to_solve.push_back((x, y + 1));
                    }
                }
            }
        }
    }

    fn solve_distances2(&mut self) {
        let mut to_solve: std::collections::VecDeque<(usize, usize)> = VecDeque::new();

        self.distances[self.end.0][self.end.1] = 0;
        to_solve.push_back(self.end);

        loop {
            let coords = to_solve.pop_front();
            match coords {
                None => break,
                Some((x, y)) => {
                    let height = self.map[x][y];
                    let distance = self.distances[x][y];

                    if x != 0 {
                        if self.is_valid2(x - 1, y, height) {
                            self.distances[x - 1][y] = distance + 1;
                            to_solve.push_back((x - 1, y));
                        }
                    }
                    if self.is_valid2(x + 1, y, height) {
                        self.distances[x + 1][y] = distance + 1;
                        to_solve.push_back((x + 1, y));
                    }
                    if y != 0 {
                        if self.is_valid2(x, y - 1, height) {
                            self.distances[x][y - 1] = distance + 1;
                            to_solve.push_back((x, y - 1));
                        }
                    }
                    if self.is_valid2(x, y + 1, height) {
                        self.distances[x][y + 1] = distance + 1;
                        to_solve.push_back((x, y + 1));
                    }
                }
            }
        }
    }

    fn get_sol(&self) -> u32 {
        return self.distances[self.end.0][self.end.1];
    }

    fn get_sol2(&self) -> u32 {
        match self.first_a {
            None => panic!("no first a!"),
            Some(a) => return self.distances[a.0][a.1],
        }
    }
}

fn task1(file: &Path) -> u32 {
    let mut map: Map = Map::new(file);
    map.solve_distances();
    return map.get_sol();
}

fn task2(file: &Path) -> u32 {
    let mut map: Map = Map::new(file);
    map.solve_distances2();

    return map.get_sol2();
}
