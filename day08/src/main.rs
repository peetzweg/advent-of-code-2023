use core::fmt;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use rayon::iter::IntoParallelRefIterator;

#[derive(Debug, Clone)]
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
                let mut all_junctions = Vec::<Junction>::new();
                let mut currents = Vec::<&Junction>::new();

                for line in reader {
                    if let Ok(line) = line {
                        if line.is_empty() {
                            continue;
                        }
                        let j = Junction::from_string(&line);
                        println!("{}", &j);
                        mapping.insert(j.name.clone(), j.clone());

                        all_junctions.push(j.clone());
                    }
                }

                // find the starting junctions
                currents = all_junctions
                    .iter()
                    .filter(|j| j.name.ends_with("A"))
                    .collect();

                println!("starts: {:?}", currents);

                // walk the instructions

                let mut instruction_cycle = instructions.0.chars().cycle();
                let mut steps: u128 = 0;

                while !all_ended(&currents) {
                    let instruction = instruction_cycle.next().expect("Should yield instruction");
                    let updated_currents: Vec<_> = currents
                        .par_iter()
                        .map(|current| match instruction {
                            'L' => mapping.get(&current.left).unwrap(),
                            'R' => mapping.get(&current.right).unwrap(),
                            _ => panic!("Unknown instruction {}", instruction),
                        })
                        .collect();

                    currents = updated_currents.iter().map(|j| *j).collect();

                    steps += 1;
                    if steps % 10000 == 0 {
                        println!("Steps: {}", steps);
                    }
                }
                println!("Steps: {}", steps);
            }
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn all_ended(junctions: &Vec<&Junction>) -> bool {
    junctions
        .iter()
        .all(|junction| junction.name.ends_with("Z"))
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
