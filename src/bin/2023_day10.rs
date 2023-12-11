use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::time::Instant;
mod utils;

const YEAR: &str = "2023";
const DAY: &str = "10";

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

enum Pipes {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl Display for Pipes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Pipes::NS => return write!(f, "│"),
            Pipes::EW => return write!(f, "─"),
            Pipes::NE => return write!(f, "└"),
            Pipes::NW => return write!(f, "┘"),
            Pipes::SW => return write!(f, "┐"),
            Pipes::SE => return write!(f, "┌"),
            Pipes::Ground => return write!(f, " "),
            Pipes::Start => return write!(f, "☺"),
        }
    }
}

#[derive(Debug, Clone)]
struct Coords {
    x: usize,
    y: usize,
}

impl PartialEq for Coords {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

struct Map {
    map: Vec<Vec<Pipes>>,
    start_pos: Coords,
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for v in &self.map {
            for c in v {
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        return write!(f, "start pos: {:?} \n", self.start_pos);
    }
}

impl Map {
    fn new(file: &Path) -> Self {
        let mut map: Vec<Vec<Pipes>> = Vec::new();
        let mut start_pos: Coords = Coords { x: 0, y: 0 };

        let mut x = 0;
        let mut y = 0;
        if let Ok(lines) = utils::read_lines(file) {
            for line in lines {
                let line = line.unwrap();
                let line = line.chars();
                let tmp_vec: Vec<Pipes> = line
                    .into_iter()
                    .map(|c| {
                        x += 1;
                        match c {
                            '|' => Pipes::NS,
                            '-' => Pipes::EW,
                            'L' => Pipes::NE,
                            'J' => Pipes::NW,
                            '7' => Pipes::SW,
                            'F' => Pipes::SE,
                            '.' => Pipes::Ground,
                            'S' => {
                                start_pos = Coords { x: x - 1, y };
                                return Pipes::Start;
                            }
                            _ => panic!("unrecognised tile: {}", c),
                        }
                    })
                    .collect();

                map.push(tmp_vec);
                y += 1;
                x = 0;
            }
        }
        return Map { map, start_pos };
    }


    fn loop_length(&self) -> i32 {
        let start_pos = &self.start_pos;
        // lil cheaty here ;)
        let mut current_pos = Coords { x: start_pos.x + 1 ,y: start_pos.y };
        let mut from = From::W;
        let mut count = 1;

        loop {
            match (from.clone(), &self.map[current_pos.y][current_pos.x]) {
                (_, Pipes::Start) => break,
                (From::N, Pipes::NW) => { current_pos = Coords { x: current_pos.x - 1, y: current_pos.y }; from = From::E },
                (From::N, Pipes::NE) => { current_pos = Coords { x: current_pos.x + 1, y: current_pos.y }; from = From::W },
                (From::N, Pipes::NS) => { current_pos = Coords { x: current_pos.x, y: current_pos.y + 1}; },
                (From::N, e) => panic!("From north but no connection: {}", e),
                (From::E, Pipes::NE) => { current_pos = Coords { x: current_pos.x, y: current_pos.y - 1 }; from = From::S },
                (From::E, Pipes::EW) => { current_pos = Coords { x: current_pos.x - 1, y: current_pos.y }; },
                (From::E, Pipes::SE) => { current_pos = Coords { x: current_pos.x, y: current_pos.y + 1}; from = From::N },
                (From::E, e) => panic!("From east but no connection: {}", e),
                (From::S, Pipes::SE) => { current_pos = Coords { x: current_pos.x + 1, y: current_pos.y }; from = From::W },
                (From::S, Pipes::SW) => { current_pos = Coords { x: current_pos.x - 1, y: current_pos.y }; from = From::E },
                (From::S, Pipes::NS) => { current_pos = Coords { x: current_pos.x, y: current_pos.y - 1 }; },
                (From::S, e) => panic!("From south but no connection: {}", e),
                (From::W, Pipes::NW) => { current_pos = Coords { x: current_pos.x, y: current_pos.y - 1 }; from = From::S },
                (From::W, Pipes::EW) => { current_pos = Coords { x: current_pos.x + 1, y: current_pos.y }; },
                (From::W, Pipes::SW) => { current_pos = Coords { x: current_pos.x, y: current_pos.y + 1}; from = From::N },
                (From::W, e) => panic!("From west but no connection: {}", e),
            }

            count +=1;
        }
        


        

        return count;
    }
}

#[derive(Clone)]
enum From {
    N,
    E,
    S,
    W
}

fn task1(file: &Path) -> i32 {
    let mut n = 0;

    let map: Map = Map::new(file);

    println!("{}", map);

    return map.loop_length() / 2;
}

fn task2(file: &Path) -> i32 {
    return 32;
}
