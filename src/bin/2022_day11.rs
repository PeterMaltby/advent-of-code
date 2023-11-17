use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::{usize, vec};
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
enum Operation {
    #[default]
    Multi,
    Add,
}

#[derive(Default, Clone, Debug)]
enum OpParam {
    #[default]
    Sqr,
    I(f64),
}

#[derive(Default, Clone, Debug)]
struct Monkey {
    items: VecDeque<f64>,
    operation: Operation,
    op_param: OpParam,
    test_param: f64,
    true_result: usize,
    false_result: usize,
    inspect_counter: i32,
}

impl Monkey {
    fn turn(&mut self) {
        for i in 0..self.items.len() {
            self.inspect_counter += 1;
            self.items[i] = match (&self.operation, &self.op_param) {
                (Operation::Multi, OpParam::Sqr) => self.items[i] * self.items[i],
                (Operation::Multi, OpParam::I(x)) => self.items[i] * x,
                (Operation::Add, OpParam::I(x)) => self.items[i] + x,
                (Operation::Add, OpParam::Sqr) => self.items[i] + self.items[i],
            };

            self.items[i] = (self.items[i] / 3.0).floor();
        }
    }

    fn turn2(&mut self, high_factor: f64) {
        for i in 0..self.items.len() {
            self.inspect_counter += 1;
            self.items[i] = match (&self.operation, &self.op_param) {
                (Operation::Multi, OpParam::Sqr) => (self.items[i] * self.items[i]) % high_factor,
                (Operation::Multi, OpParam::I(x)) => (self.items[i] * x) % high_factor,
                (Operation::Add, OpParam::I(x)) => (self.items[i] + x) % high_factor,
                (Operation::Add, OpParam::Sqr) => (self.items[i] + self.items[i]) % high_factor,
            };

        }
    }
}

fn round(mut monkeys: Vec<Monkey>) -> Vec<Monkey> {
    for i in 0..monkeys.len() {
        monkeys[i].turn();
        for x in 0..monkeys[i].items.len() {
            let var = monkeys[i].items.pop_front().unwrap();
            if var % monkeys[i].test_param == 0.0 {
                let new_monkey_id = monkeys[i].true_result;
                monkeys[new_monkey_id].items.push_back(var);
            } else {
                let new_monkey_id = monkeys[i].false_result;
                monkeys[new_monkey_id].items.push_back(var);
            }
        }
    }

    return monkeys;
}

fn round2(mut monkeys: Vec<Monkey>, high_factor: f64) -> Vec<Monkey> {
    for i in 0..monkeys.len() {
        monkeys[i].turn2(high_factor);
        for x in 0..monkeys[i].items.len() {
            let var = monkeys[i].items.pop_front().unwrap();
            if var % monkeys[i].test_param == 0.0 {
                let new_monkey_id = monkeys[i].true_result;
                monkeys[new_monkey_id].items.push_back(var);
            } else {
                let new_monkey_id = monkeys[i].false_result;
                monkeys[new_monkey_id].items.push_back(var);
            }
        }
    }

    return monkeys;
}

fn task1(file: &Path) -> i32 {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {
        let mut monkey: Monkey = Monkey::default();

        for line in lines {
            let line = line.unwrap();
            let words: Vec<&str> = line.split_whitespace().collect();

            if words.is_empty() {
                monkeys.push(monkey.clone());
                continue;
            }

            match words[0] {
                "Monkey" => continue,
                "Starting" => {
                    let (_, items) = line.split_once(": ").unwrap();
                    let items: VecDeque<f64> = items
                        .split(", ")
                        .map(|x| x.parse::<f64>().unwrap())
                        .collect();
                    monkey.items = items;
                }
                "Operation:" => {
                    if words[4] == "+" {
                        monkey.operation = Operation::Add;
                    } else {
                        monkey.operation = Operation::Multi
                    }
                    if words[5] == "old" {
                        monkey.op_param = OpParam::Sqr;
                    } else {
                        monkey.op_param = OpParam::I(words[5].parse::<f64>().unwrap());
                    }
                }
                "Test:" => {
                    monkey.test_param = words[3].parse::<f64>().unwrap();
                }
                "If" => match words[1] {
                    "true:" => monkey.true_result = words[5].parse::<usize>().unwrap(),
                    "false:" => monkey.false_result = words[5].parse::<usize>().unwrap(),
                    _ => panic!("{}", words[1]),
                },
                _ => continue,
            }

            n += 1;
        }
        monkeys.push(monkey);

        for _ in 0..20 {
            monkeys = round(monkeys);
        }

    }
    let mut monkey_business: Vec<i32> = monkeys.iter().map(|m| m.inspect_counter).collect();
    monkey_business.sort();

    return monkey_business[monkey_business.len() -2] * monkey_business[monkey_business.len() - 1];
}

fn task2(file: &Path) -> i64 {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {
        let mut monkey: Monkey = Monkey::default();

        for line in lines {
            let line = line.unwrap();
            let words: Vec<&str> = line.split_whitespace().collect();

            if words.is_empty() {
                monkeys.push(monkey.clone());
                continue;
            }

            match words[0] {
                "Monkey" => continue,
                "Starting" => {
                    let (_, items) = line.split_once(": ").unwrap();
                    let items: VecDeque<f64> = items
                        .split(", ")
                        .map(|x| x.parse::<f64>().unwrap())
                        .collect();
                    monkey.items = items;
                }
                "Operation:" => {
                    if words[4] == "+" {
                        monkey.operation = Operation::Add;
                    } else {
                        monkey.operation = Operation::Multi
                    }
                    if words[5] == "old" {
                        monkey.op_param = OpParam::Sqr;
                    } else {
                        monkey.op_param = OpParam::I(words[5].parse::<f64>().unwrap());
                    }
                }
                "Test:" => {
                    monkey.test_param = words[3].parse::<f64>().unwrap();
                }
                "If" => match words[1] {
                    "true:" => monkey.true_result = words[5].parse::<usize>().unwrap(),
                    "false:" => monkey.false_result = words[5].parse::<usize>().unwrap(),
                    _ => panic!("{}", words[1]),
                },
                _ => continue,
            }

            n += 1;
        }
        monkeys.push(monkey);

        for monkey in &monkeys {
            println!("{:?}", monkey);
        }

        let high_factor: f64 = monkeys.iter().map(|m| m.test_param).product();

        for _ in 0..10000 {
            monkeys = round2(monkeys, high_factor);
        }

    }

    let mut monkey_business: Vec<i32> = monkeys.iter().map(|m| m.inspect_counter).collect();
    monkey_business.sort();

    return monkey_business[monkey_business.len() -2 ] as i64 * monkey_business[monkey_business.len() - 1] as i64;
}
