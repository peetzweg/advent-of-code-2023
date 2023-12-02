use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const MAX_ALLOWED: [u64; 3] = [12, 13, 14];

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    match std::env::args().nth(1) {
        None => {
            println!("no filename, please provide as first argument");
        }
        Some(file_path) => {
            if let Ok(reader) = read_lines(file_path) {
                let mut accumulator: u64 = 0;

                for line in reader {
                    if let Ok(line) = line {
                        let mut split = line.split(":");
                        let game_part = split.next().unwrap().trim();
                        let reveals = split.next().unwrap().trim().split(";");

                        let game_no = game_part
                            .replace("Game ", "")
                            .parse::<u64>()
                            .expect("Invalid game number");

                        let mut max_balls: [u64; 3] = [0; 3];
                        for (reveal_no, reveal) in reveals.enumerate() {
                            let mut balls: Vec<Vec<&str>> = reveal
                                .split(",")
                                .map(|v| v.trim().split(" ").collect())
                                .collect();

                            for result in balls {
                                let amount = result[0];
                                let color = result[1];
                                match color {
                                    "red" => {
                                        max_balls[0] = std::cmp::max(
                                            max_balls[0],
                                            amount.parse::<u64>().unwrap(),
                                        );
                                    }
                                    "green" => {
                                        max_balls[1] = std::cmp::max(
                                            max_balls[1],
                                            amount.parse::<u64>().unwrap(),
                                        );
                                    }
                                    "blue" => {
                                        max_balls[2] = std::cmp::max(
                                            max_balls[2],
                                            amount.parse::<u64>().unwrap(),
                                        );
                                    }
                                    _ => {}
                                }
                            }

                            // println!("{}.{} => {:?}", game_no, &reveal_no, balls)
                        }

                        // PART 1
                        // let possible = max_balls
                        //     .iter()
                        //     .zip(MAX_ALLOWED.iter())
                        //     .all(|(a, b)| a <= b);
                        // if possible {
                        //     accumulator = accumulator + game_no;
                        // }
                        // println!(
                        //     "{} => {:?} => {:?} => {:?}",
                        //     game_no, &max_balls, &possible, &accumulator
                        // );

                        // PART 2
                        let power = max_balls
                            .iter()
                            .fold(1u64, |a, b| a.to_owned() * b.to_owned());

                        accumulator = accumulator + power;

                        println!(
                            "{} => {:?} => {:?} => {:?}",
                            game_no, &max_balls, &power, &accumulator
                        );
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
