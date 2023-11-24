use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::Instant;
mod utils;

const YEAR: &str = "2022";
const DAY: &str = "15";

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

struct Coords {
    x: i32,
    y: i32,
}

impl std::fmt::Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "({}, {})",self.x, self.y);
    }
}

impl Coords {
    fn find_max(&mut self, other: &Coords) {
        if self.x < other.x {
            self.x = other.x;
        }
        if self.y < other.y {
            self.y = other.y;
        }
    }

    fn find_min(&mut self, other: &Coords) {
        if self.x > other.x {
            self.x = other.x;
        }
        if self.y > other.y {
            self.y = other.y;
        }
    }

    fn manhattan(&self, other: &Coords) -> i32 {
        return (self.x - other.x).abs() + ( self.y - other.y).abs();

    }
}

struct Beacon {
    location: Coords,
    beacon: Coords,
}

impl std::fmt::Display for Beacon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "location: {}; beacon: {})",self.location, self.beacon);
    }
}


impl Beacon {
    fn manhattan(&self) -> i32 {
        return self.location.manhattan(&self.beacon);
    }

    fn exclusion_at_y(&self, y: i32) -> Option<(i32, i32)> {
        let dist = self.manhattan();
        let y_deviation = (self.location.y - y).abs();

        let x_dist = dist - y_deviation;

        if x_dist < 0 { return None }

        return Some((self.location.x - x_dist, self.location.x + x_dist + 1));
    }
}

struct RangeStack {
    stack: Vec<(i32,i32)>,
}

impl RangeStack {

    fn new () -> Self {
        RangeStack { stack: Vec::new() }
    }
    fn push (&mut self, to_add: (i32, i32)) {

        let mut to_add = to_add;
        let mut new_stacks: Vec<(i32, i32)> = Vec::new();

        for s in &self.stack {
            if to_add.0 < s.0 && to_add.1 < s.0 || to_add.0 > s.1 && to_add.1 > s.1
            {
                new_stacks.push(s.clone());
            } else {
                let new_x = if s.0 > to_add.0 { to_add.0 } else { s.0 };
                let new_y = if s.1 < to_add.1 { to_add.1 } else { s.1 };
                to_add = ( new_x, new_y );
            }
        }

        new_stacks.push(to_add);
        self.stack = new_stacks;
    }

    fn size(&self) -> i32 {
        let mut n = 0;
        for s in &self.stack {
            n += s.1 - s.0;
        }
        return n;
    }

    fn gaps(&self ) -> Option<i32> {
        //println!("{:?}", self.stack);
        if self.stack.len() > 1 {
            if self.stack[0].0 > self.stack[1].0 {
                return Some(self.stack[1].1);
            } else {
                return Some(self.stack[0].1);
            }
        }
        return None;
    }
}

struct Map {
    beacons: Vec<Beacon>,
    min_x_y: Coords,
    max_x_y: Coords,
}

impl Map {
    pub fn new(input: &Path) -> Self {
        let mut beacons: Vec<Beacon> = Vec::new();
        let mut min_x_y = Coords{ x: i32::MAX, y: i32::MAX };
        let mut max_x_y = Coords{ x: i32::MIN, y: i32::MIN };
        if let Ok(lines) = utils::read_lines(input) {
            for line in lines {
                let line = line.unwrap();

                let line: Vec<&str>= line.split(" ").collect();
                
                let sensor_x = line[2].chars();
                let sensor_x = sensor_x.filter(|c| { ('-'..='9').contains(c) }).collect::<String>();
                let sensor_x = sensor_x.parse::<i32>().unwrap();
                let sensor_y = line[3].chars();
                let sensor_y = sensor_y.filter(|c| { ('-'..='9').contains(c) }).collect::<String>();
                let sensor_y = sensor_y.parse::<i32>().unwrap();

                let beacon_x = line[8].chars();
                let beacon_x = beacon_x.filter(|c| { ('-'..='9').contains(c) }).collect::<String>();
                let beacon_x = beacon_x.parse::<i32>().unwrap();
                let beacon_y = line[9].chars();
                let beacon_y = beacon_y.filter(|c| { ('-'..='9').contains(c) }).collect::<String>();
                let beacon_y = beacon_y.parse::<i32>().unwrap();

                let location = Coords { x: sensor_x, y: sensor_y };
                let beacon = Coords { x: beacon_x, y: beacon_y };

                min_x_y.find_min(&location);
                min_x_y.find_min(&beacon);
                max_x_y.find_max(&location);
                max_x_y.find_max(&beacon);

                beacons.push(Beacon { location , beacon });

            }
        }

        return Map { beacons, min_x_y, max_x_y };
    }

    fn solve_y(&self, y:i32) -> i32 {
        let mut y_ranges = RangeStack::new();
        let mut beacon_remove: HashSet<i32> = HashSet::new();
        for beacon in &self.beacons {
            let res = beacon.exclusion_at_y(y);
            if beacon.beacon.y == y {
                beacon_remove.insert(beacon.beacon.x);
            }

            match res {
                None => (),
                Some(x) => y_ranges.push(x),
            }
        }

        return y_ranges.size() - beacon_remove.len() as i32;
    }

    fn find_x(&self) -> (i32, i32) {
            for y in 0..4_000_000 {
                let mut y_ranges = RangeStack::new();
                for beascon in &self.beacons {
                    let res = beascon.exclusion_at_y(y);
                    match res {
                        None => (),
                        Some(l) => y_ranges.push(l),
                    }
                }
                let res = y_ranges.gaps();

                match res {
                    None => (),
                    Some(x) => return (x,y),
                    
                }
            }

            return (0, 0);
    }

}

fn task1(file: &Path) -> i32 {

    let map = Map::new(file);
    return map.solve_y(2000000);

}

fn task2(file: &Path) -> i64 {
    let map = Map::new(file);

    let res = map.find_x();
    //println!("{:?}", map.find_x());

    return (res.0 as i64) * 4000000 + (res.1 as i64);
}
