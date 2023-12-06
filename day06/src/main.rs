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
            let mut times: Vec<u128> = vec![];
            let mut best: Vec<u128> = vec![];
            if let Ok(reader) = read_lines(file_path) {
                for (index, line) in reader.enumerate() {
                    if let Ok(line) = line {
                        println!("{:?}", line);
                        if index == 0 {
                            let mut line = line.split(" ");
                            line.next();
                            times.extend(line.filter(|v| !v.is_empty()).map(|v| {
                                println!("v: {}", v);
                                v.trim().parse::<u128>().expect("Time not a number")
                            }))
                        } else {
                            let mut line = line.split(" ");
                            line.next();
                            best.extend(
                                line.filter(|v| !v.is_empty())
                                    .map(|v| v.trim().parse::<u128>().expect("Best not a number")),
                            )
                        }
                    }
                }
            }

            println!("times: {:?}", times);
            println!("best: {:?}", best);
            let ways = times
                .iter()
                .zip(best.iter())
                .map(|(t, b)| {
                    let time = t.clone();
                    (1u128..time)
                        .map(|p| p * (time - p))
                        .filter(|v| v > b)
                        .count()
                })
                .collect::<Vec<usize>>();
            println!("ways: {:?}", ways);
            let mul_ways = ways.iter().fold(1usize, |acc, v| acc * v);
            println!("mul_ways: {:?}", mul_ways);
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
