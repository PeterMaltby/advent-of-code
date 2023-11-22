use core::fmt;
use std::fmt::write;
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::usize;
mod utils;

const YEAR: &str = "2022";
const DAY: &str = "14";

const WIDTH: usize = 1000;
const HEIGHT: usize = 200;

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

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    None,
    Stone,
    Sand,
    SandSpawn,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::None => write!(f, "."),
            Self::Stone => write!(f, "#"),
            Self::Sand => write!(f, "o"),
            Self::SandSpawn => write!(f, "@"),
        }
    }
}

struct Map {
    map: [[Tile; WIDTH]; HEIGHT],
    sand_spawn: Coords,
    sand_count: i32,
    highest_y: usize,
}

#[derive(Debug, Clone, Copy)]
struct Coords {
    x: usize,
    y: usize,
}

impl Map {
    fn new() -> Self {
        let map = [[Tile::None; WIDTH]; HEIGHT];

        return Map {
            map,
            sand_spawn: Coords { x: 0, y: 0 },
            sand_count: 0,
            highest_y: 0,
        };
    }

    fn add_wall(&mut self, walls: &Vec<(Coords)>) {
        let walls: Vec<(&(Coords), &(Coords))> = walls.iter().zip(walls.iter().skip(1)).collect();

        for wall in walls {
            let wall_a = wall.0;
            let wall_b = wall.1;

            let low_x = if wall_a.x > wall_b.x {
                wall_b.x
            } else {
                wall_a.x
            };
            let high_x = if wall_a.x < wall_b.x {
                wall_b.x
            } else {
                wall_a.x
            };

            let low_y = if wall_a.y > wall_b.y {
                wall_b.y
            } else {
                wall_a.y
            };
            let high_y = if wall_a.y < wall_b.y {
                wall_b.y
            } else {
                wall_a.y
            };

            if high_y > self.highest_y {
                self.highest_y = high_y
            };

            for x in low_x..=high_x {
                for y in low_y..=high_y {
                    self.map[y][x] = Tile::Stone;
                }
            }
        }
    }

    fn add_sand_spawn(&mut self, coords: Coords) {
        self.sand_spawn = Coords {
            x: coords.x,
            y: coords.y,
        };
        self.map[coords.y][coords.x] = Tile::SandSpawn;
    }

    fn spawn_sand(&mut self) -> bool {
        let mut current_pos = self.sand_spawn;

        loop {
            if current_pos.y == HEIGHT - 1 {
                return false;
            }
            if self.map[current_pos.y + 1][current_pos.x] == Tile::None {
                current_pos.y += 1;
                continue;
            }
            if self.map[current_pos.y + 1][current_pos.x - 1] == Tile::None {
                current_pos.y += 1;
                current_pos.x -= 1;
                continue;
            }
            if self.map[current_pos.y + 1][current_pos.x + 1] == Tile::None {
                current_pos.y += 1;
                current_pos.x += 1;
                continue;
            }
            if current_pos.y == self.sand_spawn.y {
                self.map[current_pos.y][current_pos.x] = Tile::Sand;
                self.sand_count +=1;
                return false;
            }

            self.map[current_pos.y][current_pos.x] = Tile::Sand;
            self.sand_count += 1;
            return true;
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.map {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

fn task1(file: &Path) -> i32 {
    let mut map = Map::new();
    if let Ok(lines) = utils::read_lines(file) {
        for line in lines {
            let line = line.unwrap();
            let line = line.split(' ');

            let mut wall: Vec<(Coords)> = Vec::new();
            for l in line.step_by(2) {
                let coords: Vec<&str> = l.split(',').collect();
                let coords = Coords {
                    x: coords[0].parse::<usize>().unwrap(),
                    y: coords[1].parse::<usize>().unwrap(),
                };
                wall.push(coords);
            }

            map.add_wall(&wall);
            map.add_sand_spawn(Coords { x: 500, y: 0 })
        }
    }

    loop {
        if !map.spawn_sand() {
            break;
        }
    }

    return map.sand_count;
}

fn task2(file: &Path) -> i32 {
    let mut map = Map::new();
    if let Ok(lines) = utils::read_lines(file) {
        for line in lines {
            let line = line.unwrap();
            let line = line.split(' ');

            let mut wall: Vec<(Coords)> = Vec::new();
            for l in line.step_by(2) {
                let coords: Vec<&str> = l.split(',').collect();
                let coords = Coords {
                    x: coords[0].parse::<usize>().unwrap(),
                    y: coords[1].parse::<usize>().unwrap(),
                };
                wall.push(coords);
            }

            map.add_wall(&wall);
        }
    }

    map.add_sand_spawn(Coords { x: 500, y: 0 });
    let mut wall: Vec<(Coords)> = Vec::new();

    wall.push(Coords {
        x: 0,
        y: map.highest_y + 2,
    });
    wall.push(Coords {
        x: WIDTH - 1,
        y: map.highest_y + 2,
    });

    map.add_wall(&wall);

    loop {
        if !map.spawn_sand() {
            break;
        }
    }

    return map.sand_count;
}
