use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::time::Instant;
mod utils;

const YEAR: &str = "2023";
const DAY: &str = "4";

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
struct Card {
    id: u32,
    winning_nums: Vec<i32>,
    nums: Vec<i32>,
}

impl Card {
    fn new(input: &String) -> Self {
        //println!("{}", input);
        let input: Vec<&str> = input.split_whitespace().collect();

        let id = &input[1][0..input[1].len() - 1].parse::<u32>().unwrap();

        let winning_nums: Vec<i32> = input[2..12]
            .iter()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let nums: Vec<i32> = input[13..]
            .iter()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        return Card {
            id: *id,
            winning_nums,
            nums,
        };
    }

    fn score_2(&self) -> usize {
        let mut score = 0;

        for n in &self.nums {
            for w in &self.winning_nums {
                if n == w {
                    score += 1;
                }
            }
        }

        return score;
    }

    fn score(&self) -> i32 {
        let mut score = 0;

        for n in &self.nums {
            for w in &self.winning_nums {
                if n == w {
                    if score == 0 {
                        score += 1;
                    } else {
                        score *= 2;
                    }
                }
            }
        }

        return score;
    }
}

fn task1(file: &Path) -> i32 {
    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {
        for line in lines {
            let line = line.unwrap();

            let card = Card::new(&line);

            //println!("{:?} -> {}", card, card.score());

            n += card.score();
        }
    }

    return n;
}

fn task2(file: &Path) -> i32 {
    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {
        let mut copies: VecDeque<i32> = VecDeque::from([1; 10]);
        for line in lines {
            let line = line.unwrap();

            let card = Card::new(&line);
            let score = card.score_2();

            let multiplier = match copies.pop_front() {
                None => panic!("multiplier que borked"),
                Some(x) => x,
            };
            
            //println!("{:?} -> {} - {}", copies,card.score_2(), card.score_2() as i32 * multiplier);

            copies.push_back(1);

            for i in 0..score {
                copies[i] += multiplier;
            }
            
            //println!("{:?} -> {} - {}", copies,card.score_2(), card.score_2() as i32 * multiplier);

            n += multiplier;
        }
    }

    return n;
}
