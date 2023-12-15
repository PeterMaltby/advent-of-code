use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::time::Instant;
mod utils;

const YEAR: &str = "2023";
const DAY: &str = "14";

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

#[derive(PartialEq)]
enum Tile {
    RoundRock,
    SqaureRock,
    Ground,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Ground => write!(f, "."),
            Tile::RoundRock => write!(f, "O"),
            Tile::SqaureRock => write!(f, "#"),
        }
    }
}

struct Mirror {
    map: Vec<Vec<Tile>>,
}

impl Display for Mirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f, "")?;
        }
        return Ok(());
    }
}

impl Mirror {
    fn new(file: &Path) -> Self {
        let mut map: Vec<Vec<Tile>> = Vec::new();

        if let Ok(lines) = utils::read_lines(file) {
            for line in lines {
                let line = line.unwrap();
                let row: Vec<Tile> = line.chars().into_iter().map(|c| match c {
                    '.' => Tile::Ground,
                    'O' => Tile::RoundRock,
                    '#' => Tile::SqaureRock,
                    _ => panic!("unknown tile: {}", c),
                }).collect();
                map.push(row);
            }
        }

        return Mirror { map };
    }

    fn shift_north(&mut self) -> usize {
        let mut n = 0;

        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                if self.map[y][x] == Tile::RoundRock {
                    self.map[y][x] = Tile::Ground;

                    let mut new_y = y;
                    for slide in 0..y+1 {
                        if self.map[y-slide][x] == Tile::Ground {
                            new_y = y-slide;
                        } else { break; }
                    }

                    self.map[new_y][x] = Tile::RoundRock;
                    n += self.map.len() - new_y;
                }
            }
        }
        return n;
    }

    fn shift_east(&mut self) -> usize {
        let mut n = 0;

        for x in (0..self.map[0].len()).rev() {
            for y in 0..self.map.len() {
                if self.map[y][x] == Tile::RoundRock {
                    self.map[y][x] = Tile::Ground;

                    let mut new_x = x;
                    for slide in 0..self.map.len() - x {
                        if self.map[y][x+slide] == Tile::Ground {
                            new_x = x+slide;
                        } else { break; }
                    }

                    self.map[y][new_x] = Tile::RoundRock;
                    n+= self.map.len() - y;
                }
            }
        }
        return n;
    }

    fn shift_south(&mut self) {
        for y in (0..self.map.len()).rev() {
            for x in 0..self.map[0].len() {
                if self.map[y][x] == Tile::RoundRock {
                    self.map[y][x] = Tile::Ground;

                    let mut new_y = y;
                    for slide in 0..self.map.len() - y {
                        if self.map[y+slide][x] == Tile::Ground {
                            new_y = y+slide;
                        } else { break; }
                    }

                    self.map[new_y][x] = Tile::RoundRock;
                }
            }
        }
    }

    fn shift_west(&mut self) {
        for x in 0..self.map[0].len() {
            for y in 0..self.map.len() {
                if self.map[y][x] == Tile::RoundRock {
                    self.map[y][x] = Tile::Ground;

                    let mut new_x = x;
                    for slide in 0..x+1 {
                        if self.map[y][x-slide] == Tile::Ground {
                            new_x = x-slide;
                        } else { break; }
                    }

                    self.map[y][new_x] = Tile::RoundRock;
                }
            }
        }
    }

}

fn task1(file: &Path) -> usize {
    let mut mirror: Mirror = Mirror::new(file);
    //print!("{}", mirror);
    return mirror.shift_north();
}

fn task2(file: &Path) -> usize {
    let mut mirror: Mirror = Mirror::new(file);
    //print!("{}", mirror);

    let mut loads: Vec<usize> = Vec::new();
    for i in 0..1000 {
        mirror.shift_north();
        mirror.shift_west();
        mirror.shift_south();
        loads.push(mirror.shift_east());

        //println!("{:?}", loads);

    }

    //println!("{:?}", loads);
    for i in 1..=100 {
        let a = &loads[loads.len()-i..loads.len()];
        let b = &loads[loads.len()-i*2..loads.len()-i];
        let c = &loads[loads.len()-i*3..loads.len()-i*2];

        //println!("a: {:?}", a);
        //println!("b: {:?}", b);
        //println!("c: {:?}", c);

        if a == b && b==c {
            return a[(1000000000 - loads.len() - 1) % i ];
        }
    }

    print!("failed");
    return 0;
}
