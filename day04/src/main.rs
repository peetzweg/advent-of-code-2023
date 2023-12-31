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

    fn winners(&self) -> Vec<u128> {
        self.scratch
            .iter()
            .filter(|x| self.winning.contains(x))
            .map(|x| *x)
            .collect::<Vec<u128>>()
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
                let mut cards: Vec<u128> = vec![];
                let mut value: Vec<u128> = vec![];

                for (index, line) in reader.enumerate() {
                    if let Ok(line) = line {
                        println!("{:?}", line);
                        let card = ScratchCard::from_str(&line).expect("ScratchCard not parsed");

                        // add initial card
                        if let Some(amount) = cards.get(index) {
                            cards[index] = amount + 1;
                        } else {
                            cards.push(1);
                        }

                        // current cards value
                        let all_cards_value = cards[index] * card.value();

                        // get won cards
                        let winners = card.winners();

                        let num_cards = cards[index];
                        for _ in 0..num_cards {
                            for (win_index, _) in winners.iter().enumerate() {
                                if let Some(amount) = cards.get(index + win_index + 1) {
                                    cards[index + win_index + 1] = amount + 1;
                                } else {
                                    cards.push(1);
                                }
                            }
                        }

                        println!("{} winners: {}", card.card, winners.len());
                        println!("{} total val: {}", card.card, all_cards_value);
                        println!("cards: {:?}", cards);
                        accumulator += all_cards_value;
                    }
                }
                println!("Total Value: {}", accumulator);
                println!("Total Cards: {}", cards.iter().sum::<u128>());
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
