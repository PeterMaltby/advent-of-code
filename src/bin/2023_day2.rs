use std::time::Instant;
use std::path::{PathBuf, Path};
mod utils;

const YEAR: &str = "2023";
const DAY: &str = "2";

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

fn task1(file: &Path) -> i32 {

    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {

        let mut game_id = 0;
        for line in lines {
            let line = line.unwrap();
            game_id += 1;

            //println!("{}", line);

            let mut cubes = line.split(" ");

            let mut tmp_num: i32 = 0;
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            while let Some(part) = cubes.next() {
                match part {
                    "Game" => { cubes.nth(0); },
                    "red" | "red," | "red;" => { max_red = if tmp_num > max_red { tmp_num } else { max_red };} ,
                    "green" | "green," | "green;" => { max_green = if tmp_num > max_green { tmp_num } else { max_green };} ,
                    "blue" | "blue," | "blue;" => { max_blue = if tmp_num > max_blue { tmp_num } else { max_blue };} ,
                    x => { tmp_num = x.parse::<i32>().unwrap(); }, 
                }
            }

            //println!("max red: {} , max green: {}, max blue {},", max_red, max_green, max_blue);

            if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
                //println!("game allowed");
                n+= game_id;
            }


        }
    }

    return n;
}

fn task2(file: &Path) -> i32 {

    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {

        let mut game_id = 0;
        for line in lines {
            let line = line.unwrap();
            game_id += 1;

            //println!("{}", line);

            let mut cubes = line.split(" ");

            let mut tmp_num: i32 = 0;
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            while let Some(part) = cubes.next() {
                match part {
                    "Game" => { cubes.nth(0); },
                    "red" | "red," | "red;" => { max_red = if tmp_num > max_red { tmp_num } else { max_red };} ,
                    "green" | "green," | "green;" => { max_green = if tmp_num > max_green { tmp_num } else { max_green };} ,
                    "blue" | "blue," | "blue;" => { max_blue = if tmp_num > max_blue { tmp_num } else { max_blue };} ,
                    x => { tmp_num = x.parse::<i32>().unwrap(); }, 
                }
            }

            //println!("max red: {} , max green: {}, max blue {},", max_red, max_green, max_blue);

            n += max_blue * max_green * max_red;

        }
    }

    return n;
}
