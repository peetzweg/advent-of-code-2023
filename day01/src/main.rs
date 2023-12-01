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
