use std::time::Instant;
use std::path::{PathBuf, Path};
mod utils;

const YEAR: &str = "2022";
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

#[derive(Debug)]
enum Item {
    List(Vec<Item>),
    Num(u32),
}

impl Item {
    fn new(chars: &[char]) -> Self {
        let mut list: Vec<Item> = Vec::new();

        //print!("\n");
        //print!("inspecting: ");
        //for c in chars {
        //    print!("{}",c );
        //}
        //print!("\n");

        let chars_len = chars.len();

        let mut i = 0;
        loop {
            i += 1;
            match chars[i] {
                '[' => {
                    let mut end_sec = 0;
                    let mut depth = 0;
                    for n in i..chars_len {
                        match chars[n] {
                            '[' => depth += 1,
                            ']' => depth -= 1,
                            _ => (),
                        }
                        if depth == 0 { end_sec = n; break; }
                    };
                    list.push(Item::new(&chars[i..end_sec+1]));
                    i = end_sec;
                }
                '0'..='9' => {
                    if chars[i+1] == '0' {
                        list.push(Item::Num(10));
                        i += 1;
                    } else {
                        list.push(Item::Num(chars[i].to_digit(10).unwrap()));
                    }
                }
                ',' => (),
                ']' => {
                    ////println!("return: {:?}", list);
                    return Item::List(list);
                }
                _ => panic!("invalid value {}", chars[i]),

            }
        }
    }
}

fn task1(file: &Path) -> i32 {

    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {

        let mut line_iter = lines.into_iter();

        loop {
            let next = line_iter.next();
            let line1: Vec<char> = match next {
                Some(val) => val.unwrap().chars().collect(),
                _ => break,
            };
            let line1 = Item::new(&line1);

            let next = line_iter.next();
            let line2: Vec<char> = match next {
                Some(val) => val.unwrap().chars().collect(),
                _ => break,
            };
            let line2 = Item::new(&line2);


            //blank line
            line_iter.next();

        }
    }

    return n;
}

fn task2(file: &Path) -> i32 {

    return 32;
}
