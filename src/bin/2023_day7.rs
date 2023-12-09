use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::time::Instant;
mod utils;

const YEAR: &str = "2023";
const DAY: &str = "7";

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

#[derive(Debug, Copy, Clone)]
enum Cards {
    Joker = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    Ten = 9,
    Jack = 10,
    Queen = 11,
    King = 12,
    Ace = 13,
}

struct CardHand {
    bid: i32,
    hand: [Cards; 5],
    strength: i32,
}

impl Display for CardHand {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        return write!(
            f,
            "{:?} bid: {}  strength: {}",
            self.hand, self.bid, self.strength
        );
    }
}

impl CardHand {
    fn score(&mut self) {
        let cards = self.hand;
        let mut score = 0;
        let mut jokers_count = 0;
        score += cards[4] as i32;
        score += cards[3] as i32 * 14;
        score += cards[2] as i32 * 196;
        score += cards[1] as i32 * 2744;
        score += cards[0] as i32 * 38416;

        for card in cards {
            if card as i32 == 0 { jokers_count +=1; }
        }

        let mut hash: HashMap<i32, i32> = HashMap::new();

        for card in cards {
            let card = card as i32;
            if card != 0 {
            match hash.get(&card) {
                None => {
                    hash.insert(card, 1);
                }
                Some(x) => {
                    hash.insert(card, x + 1);
                }
            }
            }
        }

        let mut counts: Vec<i32> = hash.values().cloned().collect();
        counts.sort();
        counts.reverse();

        //println!("{:?} {}", counts, jokers_count);

        // handle all cards joker
        if counts.is_empty() {
            counts.push(5);
        } else {
            counts[0] += jokers_count;
        }

        let multiplier = 537824;
        score += match counts[0] {
            5 => 6 * multiplier,
            4 => 5 * multiplier,
            3 => match counts[1] {
                2 => 4 * multiplier,
                1 => 3 * multiplier,
                _ => panic!("invalid card counts {:?}", counts),
            },
            2 => match counts[1] {
                2 => 2 * multiplier,
                1 => multiplier,
                _ => panic!("invalid card counts {:?}", counts),
            },
            1 => 0,
            _ => panic!("invalid card counts {:?}", counts),
        };

        //println!("{:?}", counts);

        self.strength = score;
    }
}

fn task1(file: &Path) -> i32 {
    let mut n = 0;
    let mut hands: Vec<CardHand> = Vec::new();
    if let Ok(lines) = utils::read_lines(file) {
        for line in lines {
            let line = line.unwrap();
            let line: Vec<&str> = line.split_whitespace().collect();

            let hand = line[0].chars();
            let hand: Vec<Cards> = hand
                .map(|c| match c {
                    'A' => Cards::Ace,
                    'K' => Cards::King,
                    'Q' => Cards::Queen,
                    'J' => Cards::Jack,
                    'T' => Cards::Ten,
                    '9' => Cards::Nine,
                    '8' => Cards::Eight,
                    '7' => Cards::Seven,
                    '6' => Cards::Six,
                    '5' => Cards::Five,
                    '4' => Cards::Four,
                    '3' => Cards::Three,
                    '2' => Cards::Two,
                    _ => panic!("invalid card {}", c),
                })
                .collect();

            let hand: [Cards; 5] = hand.try_into().unwrap();
            let bid = line[1];
            let bid = bid.parse::<i32>().unwrap();

            hands.push(CardHand {
                bid,
                hand,
                strength: 0,
            });
        }
    }

    for i in 0..hands.len() {
        hands[i].score();
    }

    hands.sort_unstable_by_key(|k| k.strength);

    let mut rank = 0;
    for hand in &hands {
        rank += 1;
        //println!("{}", hand);
        //println!("{} * {} = {}", hand.bid, rank, hand.bid * rank);
        n += hand.bid * rank;
    }

    return n;
}

fn task2(file: &Path) -> i32 {
    let mut n = 0;
    let mut hands: Vec<CardHand> = Vec::new();
    if let Ok(lines) = utils::read_lines(file) {
        for line in lines {
            let line = line.unwrap();
            let line: Vec<&str> = line.split_whitespace().collect();

            let hand = line[0].chars();
            let hand: Vec<Cards> = hand
                .map(|c| match c {
                    'A' => Cards::Ace,
                    'K' => Cards::King,
                    'Q' => Cards::Queen,
                    'J' => Cards::Joker,
                    'T' => Cards::Ten,
                    '9' => Cards::Nine,
                    '8' => Cards::Eight,
                    '7' => Cards::Seven,
                    '6' => Cards::Six,
                    '5' => Cards::Five,
                    '4' => Cards::Four,
                    '3' => Cards::Three,
                    '2' => Cards::Two,
                    _ => panic!("invalid card {}", c),
                })
                .collect();

            let hand: [Cards; 5] = hand.try_into().unwrap();
            let bid = line[1];
            let bid = bid.parse::<i32>().unwrap();

            hands.push(CardHand {
                bid,
                hand,
                strength: 0,
            });
        }
    }

    for i in 0..hands.len() {
        //print!("{}", hands[i]);
        hands[i].score();
    }

    hands.sort_unstable_by_key(|k| k.strength);

    let mut rank = 0;
    for hand in &hands {
        rank += 1;
        //println!("{}", hand);
        //println!("{} * {} = {}", hand.bid, rank, hand.bid * rank);
        n += hand.bid * rank;
    }

    return n;
}
