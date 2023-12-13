use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Junction {
    name: String,
    left: String,
    right: String
}

impl Junction {
    fn from_string(input: &str)->Self{

        Junction { name: String::from(&input[0..2]), left: String::from(&input[6..8]), right: String::from(&input[11..13]) }
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
                for line in reader {
                    if let Ok(line) = line {
                        if line.is_empty() {
                            continue;
                        }
                        println!("{:?}", line)
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
