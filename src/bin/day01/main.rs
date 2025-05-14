use std::error::Error as StdError;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Error {
    InputLengthMismatchErr,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InputLengthMismatchErr => write!(f, "input length mismatch"),
        }
    }
}

impl StdError for Error {}

fn parse_input(file_path: &str) -> Result<[Vec<u32>; 2], Box<dyn StdError>> {
    let mut locs = [Vec::new(), Vec::new()];

    let file = File::open(file_path)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        let mut split = line.split_whitespace();
        let (Some(loc_1), Some(loc_2)) = (split.next(), split.next()) else {
            return Err(Box::new(Error::InputLengthMismatchErr));
        };
        locs[0].push(loc_1.parse()?);
        locs[1].push(loc_2.parse()?);
    }

    Ok(locs)
}

// part 1
fn summed_min_dist(locs: &[Vec<u32>; 2]) -> u32 {
    locs[0]
        .iter()
        .zip(locs[1].iter())
        .map(|(x, y)| if x > y { x - y } else { y - x })
        .sum()
}

// part 2
fn similarity(locs: &[Vec<u32>; 2]) -> u32 {
    locs[0]
        .iter()
        .map(|&x| locs[1].iter().filter(|&y| *y == x).count() as u32 * x)
        .sum()
}

fn main() {
    let input_file_path = "src/bin/day01/input.txt";

    let input = parse_input(input_file_path);

    let Ok(mut locs) = input else {
        panic!("{}", input.err().unwrap());
    };

    // summed_min_dist needs sorted, similarity doesn't care
    locs[0].sort();
    locs[1].sort();

    println!("{}", summed_min_dist(&locs));
    println!("{}", similarity(&locs));
}
