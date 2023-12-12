use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    match std::env::args().nth(1) {
        None => {
            println!("no filename, please provide as first argument");
        }
        Some(file_path) => {
            if let Ok(reader) = read_lines(file_path) {
                let mut bids = Vec::new();

                for line in reader {

                    if let Ok(line) = line {
                        line.split(" ")

                    }
                }
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

fn hand_value(hand: &str) -> u8 {
    let mut cards = hand.clone().chars().collect::<Vec<char>>();

    while let Some(card) = cards.pop() {
        cards.remove(index)
    }

    4u8
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_hand_value() {

        assert_eq!(2, hand_value("32T3K"));


        let reader = read_lines("test.txt").unwrap();
        let mut lines = Vec::new();
        for line in reader {
            lines.push(line.unwrap());
        }
        assert_eq!(lines, vec!["hello", "world"]);
    }
}