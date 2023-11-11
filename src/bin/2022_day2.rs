use std::time::Instant;
use std::path::{PathBuf, Path};
mod utils;

const YEAR: &str = "2022";
const DAY: &str = "2";

fn main() {
    println!("advent of code {}, day {}", YEAR, DAY);
    let mut input_path: PathBuf = Path::new("input").join(YEAR).join(DAY);
    input_path.set_extension("txt");

    println!("input: {}", input_path.display());

    let start = Instant::now();
    println!("task 1 answer: {}", task1(&input_path));
    println!("execution took: {}ms", start.elapsed().as_micros());

    let start = Instant::now();
    println!("task 2 answer: {}", task2(&input_path));
    println!("execution took: {}ms", start.elapsed().as_micros());

}

#[derive(Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

fn makeGame(input: &str) -> (RPS, RPS) {
    let input: Vec<char>= input.chars().collect();

    let opponent = match input[0] {
        'A' => RPS::Rock,
        'B' => RPS::Paper,
        'C' => RPS::Scissors,
        _ => panic!("inorrect char input: {}", input[0]),
    };

    let player = match input[2] {
        'X' => RPS::Rock,
        'Y' => RPS::Paper,
        'Z' => RPS::Scissors,
        _ => panic!("incorrect char input: {}", input[2]),
    };
    return (opponent, player);
}

fn makeGame2(input: &str) -> (RPS, RPS) {
    let input: Vec<char>= input.chars().collect();

    let opponent = match input[0] {
        'A' => RPS::Rock,
        'B' => RPS::Paper,
        'C' => RPS::Scissors,
        _ => panic!("inorrect char input: {}", input[0]),
    };

    let player = match (&opponent, input[2]) {
        (RPS::Rock,'X') => RPS::Scissors,
        (RPS::Paper,'X') => RPS::Rock,
        (RPS::Scissors,'X') => RPS::Paper,
        (val,'Y') => val.clone(),
        (RPS::Rock,'Z') => RPS::Paper,
        (RPS::Paper,'Z') => RPS::Scissors,
        (RPS::Scissors,'Z') => RPS::Rock,
        _ => panic!("incorrect char input: {}", input[2]),
    };
    return (opponent, player);
}

fn score(input: (RPS, RPS)) -> i32 {
    let mut score: i32 = match input.1 {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    };

    score += match input {
        (RPS::Rock, RPS::Scissors) => 0,
        (RPS::Rock, RPS::Paper) => 6,
        (RPS::Paper, RPS::Scissors) => 6,
        (RPS::Paper, RPS::Rock) => 0,
        (RPS::Scissors, RPS::Rock) => 6,
        (RPS::Scissors, RPS::Paper) => 0,
        (_,_) => 3,
    };

    return score;
}

fn task1(file: &Path) -> i32 {

    let mut total_score = 0;
    if let Ok(lines) = utils::read_lines(file) {

        for line in lines {
            let val = line.unwrap();
            total_score += score(makeGame(&val));
        }
    }

    return total_score;
}

fn task2(file: &Path) -> i32 {
    let mut total_score = 0;
    if let Ok(lines) = utils::read_lines(file) {

        for line in lines {
            let val = line.unwrap();
            total_score += score(makeGame2(&val));
        }
    }

    return total_score;
}
