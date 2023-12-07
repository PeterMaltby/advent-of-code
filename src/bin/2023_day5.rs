use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Instant;
mod utils;

const YEAR: &str = "2023";
const DAY: &str = "5";

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

struct Garden {
    seeds: Vec<i64>,
    seed_ranges: VecDeque<(i64, i64)>,
    seed_to_soil: Rc<Vec<(i64, i64, i64)>>,
    soil_to_fetilizer: Rc<Vec<(i64, i64, i64)>>,
    fertilizer_to_water: Rc<Vec<(i64, i64, i64)>>,
    water_to_light: Rc<Vec<(i64, i64, i64)>>,
    light_to_temp: Rc<Vec<(i64, i64, i64)>>,
    temp_to_humidity: Rc<Vec<(i64, i64, i64)>>,
    humidity_to_location: Rc<Vec<(i64, i64, i64)>>,
}

impl Garden {
    fn new(file: &Path) -> Self {
        let seeds: Vec<i64>;
        let mut seed_ranges: VecDeque<(i64, i64)> = VecDeque::new();
        let seed_to_soil: Rc<RefCell<Vec<(i64, i64, i64)>>> = Rc::new(RefCell::new(Vec::new()));
        let soil_to_fetilizer: Rc<RefCell<Vec<(i64, i64, i64)>>> =
            Rc::new(RefCell::new(Vec::new()));
        let fertilizer_to_water: Rc<RefCell<Vec<(i64, i64, i64)>>> =
            Rc::new(RefCell::new(Vec::new()));
        let water_to_light: Rc<RefCell<Vec<(i64, i64, i64)>>> = Rc::new(RefCell::new(Vec::new()));
        let light_to_temp: Rc<RefCell<Vec<(i64, i64, i64)>>> = Rc::new(RefCell::new(Vec::new()));
        let temp_to_humidity: Rc<RefCell<Vec<(i64, i64, i64)>>> = Rc::new(RefCell::new(Vec::new()));
        let humidity_to_location: Rc<RefCell<Vec<(i64, i64, i64)>>> =
            Rc::new(RefCell::new(Vec::new()));

        if let Ok(lines) = utils::read_lines(file) {
            let mut lines = lines.into_iter();

            let line = lines.next();
            let line = line.unwrap().unwrap();

            let line: Vec<&str> = line.split_whitespace().collect();
            seeds = line[1..]
                .into_iter()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();

            let mut line = line.iter();
            line.next();

            let mut range = (0, 0);
            loop {
                match line.next() {
                    Some(x) => range.0 = x.parse::<i64>().unwrap(),
                    None => break,
                }
                match line.next() {
                    Some(x) => {
                        range.1 = x.parse::<i64>().unwrap() + range.0;
                        seed_ranges.push_front(range);
                    }
                    None => break,
                }
            }

            let mut current_vec = Rc::clone(&seed_to_soil);

            loop {
                if let Some(Ok(line)) = lines.next() {
                    //println!("{}", line);

                    match &line[..] {
                        "" => (),
                        "seed-to-soil map:" => current_vec = Rc::clone(&seed_to_soil),
                        "soil-to-fertilizer map:" => current_vec = Rc::clone(&soil_to_fetilizer),
                        "fertilizer-to-water map:" => current_vec = Rc::clone(&fertilizer_to_water),
                        "water-to-light map:" => current_vec = Rc::clone(&water_to_light),
                        "light-to-temperature map:" => current_vec = Rc::clone(&light_to_temp),
                        "humidity-to-location map:" => {
                            current_vec = Rc::clone(&humidity_to_location)
                        }
                        "temperature-to-humidity map:" => {
                            current_vec = Rc::clone(&temp_to_humidity)
                        }
                        _ => {
                            let param: Vec<i64> = line
                                .split_whitespace()
                                .map(|i| i.parse::<i64>().unwrap())
                                .collect();
                            current_vec
                                .borrow_mut()
                                .push((param[0], param[1], param[2]));
                        }
                    }
                } else {
                    break;
                }
            }
        } else {
            panic!("failed readign file");
        }
        return Garden {
            seeds,
            seed_ranges,
            seed_to_soil: Rc::new(Rc::try_unwrap(seed_to_soil).unwrap().into_inner()),
            soil_to_fetilizer: Rc::new(Rc::try_unwrap(soil_to_fetilizer).unwrap().into_inner()),
            fertilizer_to_water: Rc::new(Rc::try_unwrap(fertilizer_to_water).unwrap().into_inner()),
            water_to_light: Rc::new(Rc::try_unwrap(water_to_light).unwrap().into_inner()),
            light_to_temp: Rc::new(Rc::try_unwrap(light_to_temp).unwrap().into_inner()),
            temp_to_humidity: Rc::new(Rc::try_unwrap(temp_to_humidity).unwrap().into_inner()),
            humidity_to_location: Rc::new(
                Rc::try_unwrap(humidity_to_location).unwrap().into_inner(),
            ),
        };
    }

    fn cal_locations(&self) -> Vec<i64> {
        let mut ret: Vec<i64> = Vec::new();

        for seed in &self.seeds {
            let out = self.translate(*seed, Rc::clone(&self.seed_to_soil));
            let out = self.translate(out, Rc::clone(&self.soil_to_fetilizer));
            let out = self.translate(out, Rc::clone(&self.fertilizer_to_water));
            let out = self.translate(out, Rc::clone(&self.water_to_light));
            let out = self.translate(out, Rc::clone(&self.light_to_temp));
            let out = self.translate(out, Rc::clone(&self.temp_to_humidity));
            let out = self.translate(out, Rc::clone(&self.humidity_to_location));
            //println!("{} -> {}", seed, out);
            ret.push(out);
        }

        return ret;
    }

    fn cal_ranges(&mut self) -> i64 {
        self.range_translate(Rc::clone(&self.seed_to_soil));
        //print!("\n");
        self.range_translate(Rc::clone(&self.soil_to_fetilizer));
        //print!("\n");
        self.range_translate(Rc::clone(&self.fertilizer_to_water));
        //print!("\n");
        self.range_translate(Rc::clone(&self.water_to_light));
        //print!("\n");
        self.range_translate(Rc::clone(&self.light_to_temp));
        //print!("\n");
        self.range_translate(Rc::clone(&self.temp_to_humidity));
        //print!("\n");
        self.range_translate(Rc::clone(&self.humidity_to_location));
        return self.part2_ans();
    }

    fn translate(&self, input: i64, instruct: Rc<Vec<(i64, i64, i64)>>) -> i64 {
        for instruct in instruct.iter() {
            //println!("input: {}, instruct: {:?}", input, instruct);
            if (instruct.1..instruct.1 + instruct.2).contains(&input) {
                return input - (instruct.1 - instruct.0);
            }
        }
        return input;
    }

    fn range_translate(&mut self, instruct: Rc<Vec<(i64, i64, i64)>>) {
        let mut ranges: VecDeque<(i64, i64)> = self.seed_ranges.clone();
        let mut modified: VecDeque<(i64, i64)> = VecDeque::new();

        for instr in instruct.iter() {
            let trans_range = (instr.1, instr.1 + instr.2);
            let modifier = instr.0 - instr.1;
            for _ in 0..ranges.len() {
                let range = ranges.pop_front().unwrap();
                //println!("{:?}", ranges);

                if range.1 <= trans_range.0 || range.0 > trans_range.1 {
                    //println!("{:?} does not overlap with {:?}", range, trans_range);
                    ranges.push_back(range);
                    continue;
                }

                if range.0 >= trans_range.0 && range.1 <= trans_range.1 {
                    //println!("{:?} contained by {:?} + {}", range, trans_range, modifier);
                    modified.push_back((range.0 + modifier, range.1 + modifier));
                    continue;
                }

                if range.0 <= trans_range.0 && range.1 >= trans_range.1 {
                    //println!("{:?} contains {:?} + {}", range, trans_range, modifier);
                    ranges.push_back((range.0, trans_range.0 ));
                    modified.push_back((trans_range.0 + modifier, trans_range.1 + modifier));
                    ranges.push_back((trans_range.1, range.1 ));
                    continue;
                }

                if range.1 < trans_range.1 {
                    //println!("{:?} overlap with {:?} + {}", range, trans_range, modifier);
                    ranges.push_back((range.0, trans_range.0 ));
                    modified.push_back((trans_range.0 + modifier, range.1 + modifier));
                    continue;
                }
                if range.0 >= trans_range.0 {
                    //println!("{:?} overlap with {:?} + {}", range, trans_range, modifier);
                    modified.push_back((range.0 + modifier, trans_range.1  + modifier));
                    ranges.push_back((trans_range.1, range.1));
                    continue;
                }

                panic!("got through: {:?} , range {:?} with mod {}", range, trans_range, modifier);

            }
        }

        for _ in 0..modified.len() {
            ranges.push_back(modified.pop_front().unwrap());
        }

        self.seed_ranges = ranges;
    }

    fn part2_ans(&self) -> i64 {
        let mut val: Vec<i64> = self.seed_ranges.iter().map(|i| i.0 ).collect();

        val.sort();

        return val[0];
    }
}

impl Display for Garden {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "{:?}", self.seeds)?;
        for r in &self.seed_ranges {
            writeln!(f, "{:?},", r)?;
        }
        return writeln!(f, "{:?}", self.seed_to_soil);
    }
}

fn task1(file: &Path) -> i64 {
    let garden = Garden::new(file);

    let mut locations = garden.cal_locations();

    locations.sort();

    return locations[0];
}

fn task2(file: &Path) -> i64 {
    let mut garden = Garden::new(file);

    //println!("{}", garden);

    return garden.cal_ranges();
}
