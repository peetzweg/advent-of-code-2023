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
            part_two(&file_path);
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

const ONE_WORD: &str = "one";
const TWO_WORD: &str = "two";
const THREE_WORD: &str = "three";
const FOUR_WORD: &str = "four";
const FIVE_WORD: &str = "five";
const SIX_WORD: &str = "six";
const SEVEN_WORD: &str = "seven";
const EIGHT_WORD: &str = "eight";
const NINE_WORD: &str = "nine";
const ONE_DIGIT: &str = "1";
const TWO_DIGIT: &str = "2";
const THREE_DIGIT: &str = "3";
const FOUR_DIGIT: &str = "4";
const FIVE_DIGIT: &str = "5";
const SIX_DIGIT: &str = "6";
const SEVEN_DIGIT: &str = "7";
const EIGHT_DIGIT: &str = "8";
const NINE_DIGIT: &str = "9";
// put all in an array
const ALL: [&str; 18] = [
    ONE_WORD,
    TWO_WORD,
    THREE_WORD,
    FOUR_WORD,
    FIVE_WORD,
    SIX_WORD,
    SEVEN_WORD,
    EIGHT_WORD,
    NINE_WORD,
    ONE_DIGIT,
    TWO_DIGIT,
    THREE_DIGIT,
    FOUR_DIGIT,
    FIVE_DIGIT,
    SIX_DIGIT,
    SEVEN_DIGIT,
    EIGHT_DIGIT,
    NINE_DIGIT,
];
// ALL as uint128 values
const ALL_VALUES: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "1", "2", "3", "4", "5", "6", "7", "8", "9",
];

fn part_two(file_path: &str) {
    if let Ok(reader) = read_lines(file_path) {
        let mut accumulator = 0;
        for line in reader {
            if let Ok(line) = line {
                // for each all find in line
                let value = get_line_value(line.clone());
                accumulator = accumulator + value;
                println!("{} = {}, {}", line, value, accumulator);
            }
        }
    }
}

fn get_line_value(line: String) -> u128 {
    let mut finds: Vec<(usize, &str)> = vec![];

    for (i, pat) in ALL.iter().enumerate() {
        let mut butcher = line.clone();
        while let Some(index) = butcher.find(pat) {
            finds.push((index, ALL_VALUES[i]));
            butcher.replace_range(index..(index + pat.len()), &" ".repeat(pat.len()));
        }
    }

    finds.sort_by(|a, b| a.0.cmp(&b.0));
    println!("{:?}", finds);
    let combined = match (finds.first(), finds.last()) {
        (Some(first), Some(last)) => format!("{}{}", first.1, last.1),
        (Some(first), None) => format!("{}{}", first.1, first.1),
        _ => "0".to_string(),
    };

    combined.parse::<u128>().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn custom_values() {
        assert_eq!(get_line_value("eighthree".into()), 83);
        assert_eq!(get_line_value("sevenine".into()), 79);
        assert_eq!(get_line_value("1sevenine".into()), 19);
        assert_eq!(get_line_value("nine".into()), 99);
        assert_eq!(get_line_value("oneight".into()), 18);
        assert_eq!(get_line_value("one2one".into()), 11);
        assert_eq!(
            get_line_value("fiveckftrzxhmtwobrlgzeightkscfxzqqvm3five".into()),
            55
        );
    }
}

fn part_one(file_path: &str) {
    if let Ok(reader) = read_lines(file_path) {
        let mut accumulator = 0;
        for line in reader {
            if let Ok(line) = line {
                // fine first digit int string
                let mut digits = line.chars().filter(|c| c.is_digit(10));

                let first = digits.next();
                let last = digits.last();
                // join first and last
                let first_last = match (first, last) {
                    (Some(first), Some(last)) => format!("{}{}", first, last),
                    (Some(first), None) => format!("{}{}", first, first),
                    _ => "0".to_string(),
                };
                // cast first_last to int128
                let first_last = first_last.parse::<u128>().unwrap();
                // add to accumulator
                accumulator = accumulator + first_last;

                println!("{}, {}", first_last, accumulator);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
