use std::time::Instant;
use std::path::{PathBuf, Path};
use std::{vec, usize};
mod utils;

const YEAR: &str = "2022";
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

#[derive(Default, Clone, Debug)]
enum Operation{
    #[default]
    Multi,
    Add,
}

#[derive(Default, Clone, Debug)]
struct Monkey {
    items: Vec<f32>,
    operation: Operation,
    op_param: f32,
    test_param: f32,
    true_result: isize,
    false_result: isize,
}

fn task1(file: &Path) -> i32 {

    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {

        let mut monkey: Monkey = Monkey::default();

        for line in lines {
            let line = line.unwrap();
            println!("{}", line);
            let words: Vec<&str> = line.split_whitespace().collect();
            
            if words.is_empty() {  monkeys.push(monkey.clone()); continue; }

            match words[0] {
                "Monkey" => continue,
                "Starting" => {
                    let (_, items) = line.split_once(": ").unwrap();
                    let items: Vec<f32>= items.split(", ").map(|x| x.parse::<f32>().unwrap()).collect();
                    monkey.items = items;
                },
                "Operation" => {
                    if words[4] == "+" { monkey.operation = Operation::Add; }
                    else { monkey.operation = Operation::Multi }
                    monkey.op_param = words[5].parse::<f32>().unwrap();
                },
                "Test" => {
                    monkey.test_param = words[3].parse::<f32>().unwrap();
                }
                "If" => match words[1] {
                    "true:" => { monkey.true_result = words[5].parse::<isize>().unwrap() },
                    "false:" => { monkey.true_result = words[5].parse::<isize>().unwrap() },
                    _ => panic!("{}", words[1]),
                }
                _ => continue,
            }
            

            n += 1;
        }

        for monkey in monkeys {
            println!("{:?}", monkey);
        }
    }

    return n;
}

fn task2(file: &Path) -> i32 {

    return 32;
}
