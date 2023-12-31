use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::string;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    match std::env::args().nth(1) {
        None => {
            println!("no filename, please provide as first argument");
        }
        Some(file_path) => {
            if let Ok(reader) = read_lines(file_path) {
                let mut map: Vec<Vec<char>> = vec![];
                // ingest the map
                for line in reader {
                    if let Ok(line) = line {
                        println!("{:?}", line);
                        map.push(line.chars().collect());
                    }
                }

                println!("----------------------------");
                // wander the map

                let mut accumulator = 0u128;

                for (y, row) in map.clone().iter().enumerate() {
                    for (x, c) in row.iter().enumerate() {
                        match c {
                            // if c is a number
                            '0'..='9' => {}
                            '.' => {}
                            '*' => {
                                println!("Symbol: {},({},{})", c, x, y);
                                accumulator += get_adjacent_sum(&mut map, x, y);
                            }
                            _ => {}
                        }
                    }
                }
                println!("----------------------------");
                map.iter().for_each(|row| {
                    println!("{:?}", row);
                });

                println!("Accumulator: {}", accumulator);
            }
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn get_adjacent_sum(map: &mut Vec<Vec<char>>, x: usize, y: usize) -> u128 {
    let mut values: Vec<u128> = vec![];

    // top
    if let Some(top) = map.get_mut(y - 1) {
        // left
        values.push(consume(top, x - 1));

        // middle
        values.push(consume(top, x));

        // right
        values.push(consume(top, x + 1));
    }

    // middle
    if let Some(top) = map.get_mut(y) {
        // left
        values.push(consume(top, x - 1));

        // right
        values.push(consume(top, x + 1));
    }

    if let Some(bottom) = map.get_mut(y + 1) {
        // left
        values.push(consume(bottom, x - 1));

        // middle
        values.push(consume(bottom, x));

        // right
        values.push(consume(bottom, x + 1));
    }

    let values = values
        .iter()
        .filter(|v| v > &&0u128)
        .collect::<Vec<&u128>>();
    println!("{:?}", values);
    if values.len() == 2 {
        values[0] * values[1]
    } else {
        0u128
    }
}

fn consume(line: &mut Vec<char>, x: usize) -> u128 {
    let start = line.get(x).expect("START NOT AVAILABLE").clone();
    if !start.is_numeric() {
        return 0u128;
    }
    line[x] = '.';

    let mut backward = x;
    let mut before: Vec<char> = vec![];
    while backward > 0 {
        backward -= 1;
        let c = line.get(backward).expect("BACKWARD NOT AVAILABLE");
        if !c.is_numeric() {
            break;
        }
        before.push(c.clone());
        line[backward] = '.';
    }

    let mut forward = x;
    let mut after: Vec<char> = vec![];
    while forward < line.len() - 1 {
        forward += 1;
        let c = line.get(forward).expect("FORWARD NOT AVAILABLE");
        if !c.is_numeric() {
            break;
        }
        after.push(c.clone());
        line[forward] = '.';
    }

    before.reverse();
    println!("{:?},{},{:?}", before, start, after);
    println!("Line: {:?}", line);

    let value = format!(
        "{}{}{}",
        before.iter().collect::<String>(),
        start,
        after.iter().collect::<String>(),
    )
    .parse::<u128>()
    .expect("NOT A NUMBER?");

    return value;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_consume() {
        let mut line = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let value = consume(&mut line, 4);
        assert_eq!(value, 123456789);
        assert_eq!(line, vec!['.', '.', '.', '.', '.', '.', '.', '.', '.']);
    }

    #[test]
    fn test_consume_2() {
        let mut line = vec!['.', '2', '3', '4', '.', '.', '.', '.', '.'];
        let value = consume(&mut line, 4);
        assert_eq!(value, 0);
        assert_eq!(line, vec!['.', '2', '3', '4', '.', '.', '.', '.', '.']);
    }

    #[test]
    fn test_consume_3() {
        let mut line = vec!['.', '2', '3', '4', '5', '6', '7', '.', '.'];
        let value = consume(&mut line, 4);
        assert_eq!(value, 234567);
        assert_eq!(line, vec!['.', '.', '.', '.', '.', '.', '.', '.', '.']);
    }
}
