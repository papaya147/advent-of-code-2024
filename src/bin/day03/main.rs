use regex::Regex;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_input(file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut program = "".to_string();

    let file = File::open(file_path)?;
    for line in BufReader::new(file).lines() {
        program += &line?;
    }

    Ok(program)
}

fn sum_extracted_muls(program: &String) -> u32 {
    let r = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    r.captures_iter(&program)
        .map(|cap| {
            (
                cap.get(1)
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse::<u32>()
                    .unwrap(),
                cap.get(2)
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse::<u32>()
                    .unwrap(),
            )
        })
        .map(|(multiplicand, multiplier)| multiplicand * multiplier)
        .sum()
}

fn sum_extracted_muls_with_condition(program: &String) -> u32 {
    let r = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();

    r.captures_iter(&program)
        .fold((true, 0), |(flag, prod), cap| {
            match cap.get(0).unwrap().as_str() {
                "do()" => (true, prod),
                "don't()" => (false, prod),
                _ => {
                    if flag {
                        (
                            flag,
                            prod + cap
                                .get(1)
                                .unwrap()
                                .as_str()
                                .to_string()
                                .parse::<u32>()
                                .unwrap()
                                * cap
                                    .get(2)
                                    .unwrap()
                                    .as_str()
                                    .to_string()
                                    .parse::<u32>()
                                    .unwrap(),
                        )
                    } else {
                        (flag, prod)
                    }
                }
            }
        })
        .1
}

fn main() {
    let input_file_path = "src/bin/day03/input.txt";

    let input = parse_input(input_file_path);

    let Ok(program) = input else {
        panic!("{}", input.err().unwrap());
    };

    println!("{:?}", sum_extracted_muls(&program));
    println!("{:?}", sum_extracted_muls_with_condition(&program));
}
