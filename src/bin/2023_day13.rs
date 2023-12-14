use std::fmt::Display;
use std::time::Instant;
use std::path::{PathBuf, Path};
mod utils;

const YEAR: &str = "2023";
const DAY: &str = "13";

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

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Ash,
    Rock
}


impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Ash => return write!(f, "."),
            Tile::Rock => return write!(f, "#"),
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f,"")?;
        }
        return Ok(());
    }
}

struct Map {
    map: Vec<Vec<Tile>>,
}

impl Map {
    fn reflection(&self) -> usize {
        //print!("{}", self);

        for i in 1..self.map.len() {

            let mut tmp_reflects = true;
            for m in 1..=i {
                if i + m - 1 >= self.map.len() { break; }
                //print!("{} - {}",i - m ,i + m - 1);
                let a = &self.map[i-m];
                let b = &self.map[i+m-1];

                let res = a.into_iter().zip(b.into_iter()).filter(|&(a,b)| a == b).count() == self.map[0].len();
                if !res { tmp_reflects = false; break; };
            }

            if tmp_reflects {
                return i;
            }

        }

        return 0;
    }

    fn reflection2(&self) -> usize {
        //print!("\n{}", self);

        for i in 1..self.map.len() {

            let mut changes_req = 0;
            for m in 1..=i {
                if i + m - 1 >= self.map.len() { break; }
                //print!("{} - {}",i - m ,i + m - 1);
                let a = &self.map[i-m];
                let b = &self.map[i+m-1];

                changes_req += self.map[0].len() - a.into_iter().zip(b.into_iter()).filter(|&(a,b)| a == b).count();
                //if !res { tmp_reflects = false; break; };
            }

            //println!("chnages needed: {}", changes_req);
            if changes_req == 1 { return i};
        }

        return 0;
    }

    fn translate(&mut self) {
        let mut tmp_map: Vec<Vec<Tile>> = Vec::new();

        for col in 0..self.map[0].len() {
            let tmp: Vec<Tile> = self.map.iter().map(|c| c[col]).collect();
            tmp_map.push(tmp);
        }

        self.map = tmp_map;
    }
}

fn task1(file: &Path) -> usize {
    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {

        let mut map: Vec<Vec<Tile>> = Vec::new();
        let mut maps : Vec<Map> = Vec::new();
        for line in lines {
            let line = line.unwrap();

            if line.is_empty() {
                maps.push(Map { map });
                map = Vec::new();
                continue;
            }

            let tmp: Vec<Tile> = line.chars().into_iter().map(|c| match c {
                '.' => Tile::Ash,
                '#' => Tile::Rock,
                _ => panic!("unknown char: {}", c),
            }).collect();
            map.push(tmp);
        }

        maps.push(Map { map });



        for mut map in maps {
            n += map.reflection() * 100;
            map.translate();
            n += map.reflection();
        }
    }

    return n;
}

fn task2(file: &Path) -> usize {
    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {

        let mut map: Vec<Vec<Tile>> = Vec::new();
        let mut maps : Vec<Map> = Vec::new();
        for line in lines {
            let line = line.unwrap();

            if line.is_empty() {
                maps.push(Map { map });
                map = Vec::new();
                continue;
            }

            let tmp: Vec<Tile> = line.chars().into_iter().map(|c| match c {
                '.' => Tile::Ash,
                '#' => Tile::Rock,
                _ => panic!("unknown char: {}", c),
            }).collect();
            map.push(tmp);
        }

        maps.push(Map { map });



        for mut map in maps {
            n += map.reflection2() * 100;
            map.translate();
            n += map.reflection2();
        }
    }

    return n;
}
