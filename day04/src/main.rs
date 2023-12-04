use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

struct ScratchCard {
    card: u128,
    winning: Vec<u128>,
    scratch: Vec<u128>,
}

fn parse_numbers(s: &str) -> Vec<u128> {
    s.split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.trim()
                .parse::<u128>()
                .expect("Winning Number is not a number")
        })
        .collect::<Vec<u128>>()
}

impl FromStr for ScratchCard {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(":");
        let card_string = s.next().unwrap();

        let card = card_string
            .replace("Card", "")
            .trim()
            .parse::<u128>()
            .expect("Invalid Card Number");

        let mut card_content = s.next().unwrap().split("|");

        let winning = parse_numbers(card_content.next().unwrap());

        let scratch: Vec<u128> = parse_numbers(card_content.next().unwrap());

        Ok(ScratchCard {
            winning,
            scratch,
            card,
        })
    }
}

impl ScratchCard {
    fn value(&self) -> u128 {
        self.scratch.iter().fold(0, |acc, x| {
            if self.winning.contains(x) {
                if acc == 0 {
                    1
                } else {
                    acc * 2
                }
            } else {
                acc
            }
        })
    }
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    match std::env::args().nth(1) {
        None => {
            println!("no filename, please provide as first argument");
        }
        Some(file_path) => {
            if let Ok(reader) = read_lines(file_path) {
                let mut accumulator = 0u128;
                for line in reader {
                    if let Ok(line) = line {
                        println!("{:?}", line);
                        let card = ScratchCard::from_str(&line).expect("ScratchCard not parsed");
                        println!("{} => {}", card.card, card.value());
                        accumulator += card.value();
                    }
                }
                println!("Total: {}", accumulator);
            }
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
