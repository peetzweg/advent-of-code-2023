use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Debug)]
struct DestinationToSource {
    destination_label: String,
    source_label: String,
    destination: Vec<u128>,
    source: Vec<u128>,
}

impl Display for DestinationToSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DestinationToSource")
    }
}

impl DestinationToSource {
    fn new(destination: String, source: String) -> Self {
        DestinationToSource {
            destination_label: destination,
            source_label: source,
            destination: vec![],
            source: vec![],
        }
    }

    fn add(&mut self, line: &str) {
        let mut s = line.split(" ");

        let destination_start = s
            .next()
            .expect("destination_start not available")
            .parse::<u128>()
            .expect("destination_start not a number");

        let source_start = s
            .next()
            .expect("source_start not available")
            .parse::<u128>()
            .expect("source_start not a number");

        let range_length = s
            .next()
            .expect("range_length not available")
            .parse::<u128>()
            .expect("range_length not a number");

        let destination = destination_start..(destination_start + range_length);
        let source = source_start..(source_start + range_length);

        self.destination.extend(destination);
        self.source.extend(source);
    }

    fn resolve(&self, source: &u128) -> u128 {
        let index = self.source.iter().position(|v| v == source);
        if index.is_some() {
            self.destination
                .get(index.unwrap())
                .expect("Destination for source index should be available")
                .to_owned()
        }
        // if not found the destination is the same as source
        else {
            source.to_owned()
        }
    }

    fn print(&self) {
        self.source
            .iter()
            .zip(self.destination.iter())
            .for_each(|(s, d)| println!("{}->{}", s, d))
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
                let mut maps: HashMap<String, (String, DestinationToSource)> = HashMap::new();

                let mut current_map: DestinationToSource =
                    DestinationToSource::new("dummy".to_string(), "dummy".to_string());
                let seeds = reader
                    .next()
                    .expect("Invalid format, should start with seeds")
                    .expect("invalid format");

                // Create Maps
                for (_, line) in reader.enumerate() {
                    if let Ok(line) = line {
                        // Init new Map
                        if line.ends_with("map:") {
                            let mut map_descriptor =
                                line.split(" ").next().expect("not valid input").split("-");
                            let source_label = map_descriptor.next().expect("No source_label");
                            let destination_label =
                                map_descriptor.skip(1).next().expect("No destination_label");
                            current_map = DestinationToSource::new(
                                destination_label.to_string(),
                                source_label.to_string(),
                            );
                        }
                        // Empty Line, finish map
                        else if line.is_empty() {
                            maps.insert(
                                current_map.source_label.clone(),
                                (current_map.destination_label.clone(), current_map.clone()),
                            );
                        }
                        // Not empty line
                        else {
                            current_map.add(&line);
                        }
                    }

                    maps.insert(
                        current_map.source_label.clone(),
                        (current_map.destination_label.clone(), current_map.clone()),
                    );
                }

                // Use Maps
                let seeds = seeds
                    .split(" ")
                    .skip(1)
                    .map(|v| v.parse::<u128>().expect("Seed should be numbers"))
                    .collect::<Vec<u128>>();

                let locations = seeds
                    .iter()
                    .map(|s| {
                        let mut map = maps.get("seed");
                        let mut source = s.to_owned();
                        println!("---Seed: {}---", source);
                        while map.is_some() {
                            let (destination, current_map) = map.unwrap();

                            println!(
                                "{}({})\t->\t{}({})",
                                current_map.source_label,
                                source,
                                current_map.destination_label,
                                current_map.resolve(&source)
                            );

                            source = current_map.resolve(&source);
                            map = maps.get(destination)
                        }
                        source
                    })
                    .collect::<Vec<u128>>();

                println!("{:?}", locations)
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
