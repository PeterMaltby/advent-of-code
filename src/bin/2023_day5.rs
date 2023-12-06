use std::cell::RefCell;
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
    seeds: Vec<u32>,
    seed_ranges: Vec<(u32,u32)>,
    seed_to_soil: Rc<Vec<(u32, u32, u32)>>,
    soil_to_fetilizer: Rc<Vec<(u32, u32, u32)>>,
    fertilizer_to_water: Rc<Vec<(u32, u32, u32)>>,
    water_to_light: Rc<Vec<(u32, u32, u32)>>,
    light_to_temp: Rc<Vec<(u32, u32, u32)>>,
    temp_to_humidity: Rc<Vec<(u32, u32, u32)>>,
    humidity_to_location: Rc<Vec<(u32, u32, u32)>>,
}

impl Garden {
    fn new(file: &Path) -> Self {
        let seeds: Vec<u32>;
        let mut seed_ranges: Vec<(u32, u32)> = Vec::new();
        let seed_to_soil: Rc<RefCell<Vec<(u32, u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let soil_to_fetilizer: Rc<RefCell<Vec<(u32, u32, u32)>>> =
            Rc::new(RefCell::new(Vec::new()));
        let fertilizer_to_water: Rc<RefCell<Vec<(u32, u32, u32)>>> =
            Rc::new(RefCell::new(Vec::new()));
        let water_to_light: Rc<RefCell<Vec<(u32, u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let light_to_temp: Rc<RefCell<Vec<(u32, u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let temp_to_humidity: Rc<RefCell<Vec<(u32, u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let humidity_to_location: Rc<RefCell<Vec<(u32, u32, u32)>>> =
            Rc::new(RefCell::new(Vec::new()));

        if let Ok(lines) = utils::read_lines(file) {
            let mut lines = lines.into_iter();

            let line = lines.next();
            let line = line.unwrap().unwrap();

            let line: Vec<&str> = line.split_whitespace().collect();
            seeds = line[1..]
                .into_iter()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();

            let mut line = line.iter();
            line.next();

            let mut range = (0,0);
            loop {
                match line.next() {
                    Some(x) => range.0 = x.parse::<u32>().unwrap(),
                    None => break,
                }
                match line.next() {
                    Some(x) => {
                        range.1 = x.parse::<u32>().unwrap() + range.0;
                        seed_ranges.push(range);
                    },
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
                            let param: Vec<u32> = line
                                .split_whitespace()
                                .map(|i| i.parse::<u32>().unwrap())
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
            humidity_to_location: Rc::new(Rc::try_unwrap(humidity_to_location).unwrap().into_inner()),
        };
    }

    fn cal_locations(&self) -> Vec<u32> {
        let mut ret: Vec<u32> = Vec::new();

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

    fn translate(&self, input: u32, instruct: Rc<Vec<(u32, u32, u32)>>) -> u32 {
        for instruct in instruct.iter() {
            if (instruct.1..instruct.1 + instruct.2).contains(&input) {
                return input - (instruct.1 - instruct.0);
            }
        }
        return input;
    }
}

impl Display for Garden {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "{:?}", self.seeds)?;
        for r in &self.seed_ranges {
            writeln!(f, "{:?},", r)?;
        }
        return writeln!(f, "{:?}", self.humidity_to_location);
    }
}

fn task1(file: &Path) -> u32 {
    let garden = Garden::new(file);

    let mut locations = garden.cal_locations();

    locations.sort();

    return locations[0];
}

fn task2(file: &Path) -> u32 {

    let garden = Garden::new(file);

    println!("{}", garden);

    let mut locations = garden.cal_locations();

    locations.sort();

    return locations[0];
}
