use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_input(file_path: &str) -> Result<Vec<Vec<i16>>, Box<dyn Error>> {
    let mut reports = Vec::new();

    let file = File::open(file_path)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        let split = line.split_whitespace();
        reports.push(
            split
                .map(|x| x.parse::<i16>())
                .collect::<Result<Vec<i16>, _>>()?,
        );
    }

    Ok(reports)
}

fn is_valid_report(v: &[i16]) -> bool {
    let mut diffs = v.windows(2).map(|w| w[1] - w[0]);
    diffs.clone().all(|d| (1..=3).contains(&d)) || diffs.all(|d| (-3..=-1).contains(&d))
}

fn gradual_increase_or_decrease(reports: &Vec<Vec<i16>>) -> u32 {
    reports
        .iter()
        .map(|report| u32::from(is_valid_report(&report)))
        .sum()
}

fn gradual_increase_or_decrease_with_removal(reports: &Vec<Vec<i16>>) -> u32 {
    reports
        .iter()
        .map(|report| {
            if is_valid_report(report) {
                return 1;
            }

            for i in 0..report.len() {
                let modified: Vec<_> = report
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, x)| *x)
                    .collect();

                if is_valid_report(&modified) {
                    return 1;
                }
            }
            0
        })
        .sum()
}

fn main() {
    let input_file_path = "src/bin/day02/input.txt";

    let input = parse_input(input_file_path);

    let Ok(reports) = input else {
        panic!("{}", input.err().unwrap());
    };

    println!("{}", gradual_increase_or_decrease(&reports));
    println!("{}", gradual_increase_or_decrease_with_removal(&reports));
}
