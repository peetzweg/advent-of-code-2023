use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Index;
use std::path::Path;

#[derive(PartialEq, Eq, Debug)]
enum Hand {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    fn value(&self) -> u8 {
        match self {
            Hand::FiveOfAKind => 7,
            Hand::FourOfAKind => 6,
            Hand::FullHouse => 5,
            Hand::ThreeOfAKind => 4,
            Hand::TwoPair => 3,
            Hand::OnePair => 2,
            Hand::HighCard => 1,
        }
    }

    fn from_string(input: &str) -> Self {
        hand_value(input)
    }
}

#[derive(Debug)]
struct Player {
    raw_hand: String,
    bid: usize,
    hand: Hand,
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let values = [self, other].map(|e| e.hand.value());
        // check value
        let outcome = match values {
            [a, b] if a > b => Ordering::Greater,
            [a, b] if b > a => Ordering::Less,
            _ => Ordering::Equal,
        };

        // check string
        if outcome == Ordering::Equal {


            let mut m = HashMap::new();
            m.insert('T', 1);
            m.insert('J',2);
            m.insert('Q',3);
            m.insert('K',4);
            m.insert('A',5);

            let pairs = self.raw_hand.chars().zip(other.raw_hand.chars());
            for pair in pairs {
                match pair {
                    (a,b) if a ==b =>{}
                    (a, b) if a.is_alphabetic() && b.is_numeric() => {
                        return Some(Ordering::Greater);
                    }
                    (a,b) if a.is_numeric() && b.is_alphabetic() =>{
                        return Some(Ordering::Less);
                    }
                    (a,b) if a.is_numeric() && b.is_numeric() =>{
                        return Some(a.cmp(&b));
                    }
                    (a,b) if a.is_alphabetic() && b.is_alphabetic() =>{


                        let [a,b] = [a,b].map(|e|m[&e]);
                        return Some(a.cmp(&b));
                    }
                    _ => {
                        panic!("Should not happen");
                    },
                }
            }
        }

        Some(outcome)
    }
}

impl Eq for Player {}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.raw_hand == other.raw_hand
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
                let mut game: Vec<Player> = vec![];

                for line in reader {
                    if let Ok(line) = line {
                        let mut line = line.split(" ");
                        let raw_hand = line.next().unwrap();
                        let bid = line.next().unwrap().parse::<usize>().expect("Bid not NaN");
                        let hand = Hand::from_string(raw_hand);

                        game.push(Player {
                            raw_hand: raw_hand.to_string(),
                            bid,
                            hand,
                        })
                    }
                }

                game.sort();
                for player in &game {
                    println!("{:?}", player);
                }

                let total_winnings = game.iter().enumerate().fold(0usize, |a,(rank,player)| (player.bid*(rank+1))+a);
                println!("total_winnings: {:?}",total_winnings);
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

fn hand_value(hand: &str) -> Hand {
    let cards = hand.chars().collect::<Vec<char>>();
    let mut map: HashMap<char, usize> = HashMap::new();

    for card in &cards {
        if !map.contains_key(&card) {
            let count = cards.iter().filter(|c| c == &card).count();
            map.insert(card.clone(), count);
        }
    }

    let mut identical = map
        .iter()
        .map(|(_, amount)| amount)
        .collect::<Vec<&usize>>();
    identical.sort();
    identical.reverse();

    let first_pair = (identical.get(0), identical.get(1));

    match first_pair {
        (Some(5), None) => Hand::FiveOfAKind,

        (Some(4), _) => Hand::FourOfAKind,

        (Some(3), Some(2)) => Hand::FullHouse,

        (Some(3), _) => Hand::ThreeOfAKind,

        (Some(2), Some(2)) => Hand::TwoPair,

        (Some(2), _) => Hand::OnePair,
        _ => Hand::HighCard,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hand_value() {
        assert_eq!(Hand::FiveOfAKind, hand_value("22222"));
        assert_eq!(Hand::FourOfAKind, hand_value("22223"));
        assert_eq!(Hand::FullHouse, hand_value("22233"));
        assert_eq!(Hand::ThreeOfAKind, hand_value("2223K"));
        assert_eq!(Hand::TwoPair, hand_value("3223K"));
        assert_eq!(Hand::OnePair, hand_value("32T3K"));
    }
}
