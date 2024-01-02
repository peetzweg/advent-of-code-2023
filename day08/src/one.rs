use core::fmt;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Junction {
    name: String,
    left: String,
    right: String,
}
struct Instructions(String);

impl Junction {
    fn from_string(input: &str) -> Self {
        Junction {
            name: String::from(&input[0..3]),
            left: String::from(&input[7..10]),
            right: String::from(&input[12..15]),
        }
    }
}

impl Display for Junction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{},{}", self.name, self.left, self.right)
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
            if let Ok(mut reader) = read_lines(file_path) {
                let instructions: Instructions = Instructions(
                    reader
                        .next()
                        .expect("No first line")
                        .expect("Not valid instructions"),
                );

                let mut mapping = HashMap::<String, Junction>::new();

                for line in reader {
                    if let Ok(line) = line {
                        if line.is_empty() {
                            continue;
                        }
                        let j = Junction::from_string(&line);

                        println!("{}", &j);
                        mapping.insert(j.name.clone(), j);
                    }
                }

                println!("{:?}", mapping);

                // walk the instructions
                let mut current = mapping.get("AAA").unwrap();
                let mut instruction_cycle = instructions.0.chars().cycle();
                let mut steps = 0;

                while current.name != "ZZZ" {
                    let instruction = instruction_cycle.next().expect("Should yield instruction");
                    match instruction {
                        'L' => {
                            current = mapping.get(&current.left).unwrap();
                        }
                        'R' => {
                            current = mapping.get(&current.right).unwrap();
                        }
                        _ => {
                            panic!("Unknown instruction {}", instruction);
                        }
                    }
                    steps += 1;
                }

                println!("Steps: {}", steps);
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
