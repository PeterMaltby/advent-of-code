use std::collections::HashMap;
use std::fmt::{self, Display};
use std::path::{Path, PathBuf};
use std::time::{Instant, self};
mod utils;

const YEAR: &str = "2022";
const DAY: &str = "16";

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

#[derive(Clone)]
struct Valve {
    name: String,
    flow_rate: i32,
    leads_to: Vec<(i32, String)>,
}

impl fmt::Display for Valve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} flow: {}, ",self.name, self.flow_rate);
        for l in &self.leads_to {
            write!(f, "{}:{}, ", l.1, l.0);
        }
        return write!(f, "");
    }
}

struct ValveMap {
    valves: Vec<Valve>,
    hashValues: HashMap<String, i32>,
}

impl ValveMap {
    fn new(file: &Path) -> Self {
        let mut valves: Vec<Valve> = Vec::new();
        let mut hashValues: HashMap<String, i32> = HashMap::new();
        if let Ok(lines) = utils::read_lines(file) {
            for line in lines {
                let line = line.unwrap();

                let input: Vec<&str> = line.split(" ").collect();

                let name = input[1].to_string();

                let flow_rate = input[4].chars();
                let flow_rate = flow_rate
                    .filter(|c| ('0'..='9').contains(c))
                    .collect::<String>();
                let flow_rate = flow_rate.parse::<i32>().unwrap();

                let leads_to_raw = input.into_iter().skip(9);

                let mut leads_to: Vec<(i32, String)> = Vec::new();
                for c in leads_to_raw {
                    leads_to.push((1, c.chars().filter(|k| ('A'..='Z').contains(k)).collect()));
                }

                valves.push(Valve {
                    name: name.clone(),
                    flow_rate,
                    leads_to,
                });

                hashValues.insert(name, flow_rate);
            }
        }
        return ValveMap { valves, hashValues};
    }

    fn cal_dist(&mut self) {
        for i in 0..self.valves.len() {
            let v = &self.valves[i].clone();
            if v.leads_to.len() == self.valves.len() {
                break;
            }
            let mut routes_toadd: Vec<(i32, String)> = Vec::new();
            for route in &v.leads_to {
                let length = route.0;
                let valve_routes = self.valves.iter().find(|f| f.name == route.1).unwrap();
                for added_route in &valve_routes.leads_to {
                    routes_toadd.push((length + added_route.0, added_route.1.clone()));
                }
            }

            loop {
                let new_route = match routes_toadd.pop() {
                    None => break,
                    Some(x) => x,
                };

                let old = self.valves[i].leads_to.iter().find(|f| f.1 == new_route.1);

                println!("old: {:?}, new: {:?}", old, new_route);

                match old {
                    None => self.valves[i].leads_to.push(new_route),
                    Some(o) => if o.0 > new_route.0 {
                        self.valves[i].leads_to.retain(|x| x.1 != new_route.1 );
                        self.valves[i].leads_to.push(new_route);
                        },
                }
            }
        }
    }

    fn trim(&mut self, keep: &str) {

        for i in 0..self.valves.len() {
            self.valves[i].leads_to.retain(|l| {
                let flow_rate = self.hashValues.get(&l.1).unwrap();
                return *flow_rate != 0;
            });
        }
        self.valves.retain(|v| v.flow_rate != 0 || v.name == keep);
    }

    fn find_max_from(&mut self, from: String, currentTime: i32) {
        let leads_to =  self.valves.iter().find(|c| c.name == from).unwrap();

        for route in leads_to.leads_to.iter() {
            let value = (30 - currentTime - route.0 - 1) * self.hashValues.get(&route.1).unwrap();
            println!("{} - dist: {}, flow_rate: {}, value: {}",route.1, route.0, self.hashValues.get(&route.1).unwrap(), value);
        }

        }

    fn cal_routes(&self, from: String, timeLimit: i32, mut no_visit: Vec<String>,mut score: i32 ) -> Route {
        let start_route = self.valves.iter().find(|v| v.name == from).unwrap();
        no_visit.push(from.to_string());

        score += start_route.flow_rate * timeLimit;

        let mut route = Route { visited: no_visit.clone(), score, time: timeLimit};
        //println!("current route: {:?}", route);

        let possible_route: Vec<Option<Route>> = start_route.leads_to.iter().map(|v| {
            let time = timeLimit - v.0 - 1;
            if no_visit.contains(&v.1) { return None; }
            if time < 0 { return None }
            return Some(self.cal_routes(v.1.clone(), time, no_visit.clone(), score));
        }).collect();

        for r in possible_route {
            match r {
                None => continue,
                Some(r) => {
                    if r.score >= route.score {
                        route = r;
                    }
                }
            };

        }
        //println!("best: {:?}", route);

        return route;


    }
}

#[derive(Debug)]
struct Route {
    visited: Vec<String>,
    score: i32,
    time: i32,
}

fn task1(file: &Path) -> i32 {
    let mut n = 0;

    let mut route: ValveMap = ValveMap::new(file);

    for i in 0..(route.valves.len() as f32).sqrt() as usize + 1 {
        route.cal_dist();
        println!("cal routes iter: {}", i);
    }

    print!("\n");
    for r in &route.valves {
        println!("{}", r);
    }

    route.trim("AA");

    print!("\n");
    for r in &route.valves {
        println!("{}", r);
    }

    let route = route.cal_routes("AA".to_string(), 30, vec![], 0);

    println!("{:?}", route);

    return route.score;
}

fn task2(file: &Path) -> i32 {
    return 32;
}
