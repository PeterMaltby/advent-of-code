use std::cmp::Ordering;
use std::iter::zip;
use std::path::{Path, PathBuf};
use std::time::Instant;
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

#[derive(Debug, Clone)]
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
                        if depth == 0 {
                            end_sec = n;
                            break;
                        }
                    }
                    list.push(Item::new(&chars[i..end_sec + 1]));
                    i = end_sec;
                }
                '0'..='9' => {
                    if chars[i + 1] == '0' {
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

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        let is_smaller = is_smaller(self, other);
        match is_smaller {
            Comp::False => return Ordering::Greater,
            Comp::Eq => return Ordering::Equal,
            Comp::True => return Ordering::Less,
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        let is_smaller = is_smaller(self, other);
        match is_smaller {
            Comp::False => return false,
            Comp::Eq => return true,
            Comp::True => false,
        }
    }
}

impl Eq for Item {}

#[derive(Debug)]
enum Comp {
    True,
    False,
    Eq,
}

fn is_smaller(item_a: &Item, item_b: &Item) -> Comp {
    match (item_a, item_b) {
        (Item::Num(x), Item::Num(y)) => {
            if x == y {
                return Comp::Eq;
            }
            if x < y {
                return Comp::True;
            } else {
                return Comp::False;
            }
        }
        (Item::List(x), Item::List(y)) => {
            let mut x = x.into_iter();
            let mut y = y.into_iter();

            loop {
                let vals = (x.next(), y.next());
                match vals {
                    (Some(o), Some(p)) => {
                        let state = is_smaller(o, p);
                        match state {
                            Comp::True => return Comp::True,
                            Comp::Eq => (),
                            Comp::False => return Comp::False,
                        }
                    }
                    (Some(_), None) => return Comp::False,
                    (None, Some(_)) => return Comp::True,
                    (None, None) => return Comp::Eq,
                }
            }
        }
        (Item::List(_), Item::Num(y)) => {
            let new_list_y: Item = Item::List(vec![Item::Num(*y)]);
            return is_smaller(item_a, &new_list_y);
        }
        (Item::Num(x), Item::List(_)) => {
            let new_list_x: Item = Item::List(vec![Item::Num(*x)]);
            return is_smaller(&new_list_x, item_b);
        }
    }
}

fn task1(file: &Path) -> i32 {
    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {
        let mut line_iter = lines.into_iter();

        let mut i = 0;

        loop {
            i += 1;
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

            let test = is_smaller(&line1, &line2);
            match test {
                Comp::False => (),
                Comp::Eq => n += i,
                Comp::True => n += i,
            }

            //blank line
            line_iter.next();
        }
    }

    return n;
}

fn task2(file: &Path) -> i32 {
    let mut n = 0;
    if let Ok(lines) = utils::read_lines(file) {
        let mut all_vals: Vec<Item> = Vec::new();
        for line in lines {
            let line = line.unwrap();

            if line.is_empty() {
                continue;
            }

            let line: Vec<char> = line.chars().collect();
            all_vals.push(Item::new(&line));
        }

        let devider_pack_1 = Item::List(vec![Item::List(vec![Item::Num(2)])]);
        let devider_pack_2 = Item::List(vec![Item::List(vec![Item::Num(6)])]);

        all_vals.push(devider_pack_1.clone());
        all_vals.push(devider_pack_2.clone());

        all_vals.sort();

        let mut i = 0;
        for x in all_vals {
            i += 1;
            if x == devider_pack_1 {
                n += i;
            }
            if x == devider_pack_2 {
                n *= i;
            }
        }
    }

    return n;
}
