use std::time::Instant;
use std::path::{PathBuf, Path};
use std::collections::VecDeque;
mod utils;

const YEAR: &str = "2022";
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
    println!("task 2 answer: ");
    println!("{}", task2(&input_path));
    println!("execution took: {}μs", start.elapsed().as_micros());

}

fn task1(file: &Path) -> i32 {

    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {
        
        let mut cycles: VecDeque<Option<i32>> = VecDeque::new();
        for line in lines {
            let line = line.unwrap();
            let line: Vec<&str> = line.split(' ').collect();

            if line[0] == "noop" {
                cycles.push_back(None);
            } else {
                cycles.push_back(None);
                cycles.push_back(Some(line[1].parse().unwrap()));
            }
        }

        let mut clock = 0;
        let mut x = 1;
        loop {
            if cycles.front() == None { break; }
            clock += 1;
            if (20 + clock) % 40 == 0 {
                n += x * clock;
            }
            match cycles.pop_front() {
                None => break,
                Some(None) => continue,
                Some(Some(i)) => x += i,
            };
        }
    }

    return n;
}

fn task2(file: &Path) -> String {
    let mut ret: String = String::new();

    if let Ok(lines) = utils::read_lines(file) {
        
        let mut cycles: VecDeque<Option<i32>> = VecDeque::new();
        for line in lines {
            let line = line.unwrap();
            let line: Vec<&str> = line.split(' ').collect();

            if line[0] == "noop" {
                cycles.push_back(None);
            } else {
                cycles.push_back(None);
                cycles.push_back(Some(line[1].parse().unwrap()));
            }
        }

        let mut clock: i32 = 0;
        let mut x: i32 = 1;
        
        let mut row: usize = 0;
        let mut crt: [[char; 40]; 6] = [['.'; 40]; 6];
        loop {
            if cycles.front() == None { break; }
            clock += 1;
            
            let carat: usize= ((clock - 1)%40) as usize;
            if carat == 39 { row +=1 };
            
            let sprite = (carat as i32 - 1)..(carat as i32 + 2);
            if sprite.contains(&x) {
                crt[row][carat] = '#';
            }

            match cycles.pop_front() {
                None => break,
                Some(None) => continue,
                Some(Some(i)) => x += i,
            };
        }

        for scanline in crt {
            ret = format!("{}{}\n",ret, scanline.iter().collect::<String>());
        }
    }

    return ret;
}
