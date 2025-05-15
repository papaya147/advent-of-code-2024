use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_input(file_path: &str) -> Result<Vec<(i128, Vec<u128>)>, Box<dyn Error>> {
    let mut calibration = Vec::new();

    let file = File::open(file_path)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        let mut split = line.split(":");
        let res = split.next().unwrap().parse::<i128>().unwrap();
        let nums = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        calibration.push((res, nums));
    }

    Ok(calibration)
}

fn is_valid_calibration(res: i128, nums: &Vec<u128>, pos: usize) -> bool {
    if pos == 0 {
        return *nums.get(pos).unwrap() as i128 == res;
    }

    let last_num = *nums.get(pos).unwrap();
    let mut valid = is_valid_calibration(res + last_num as i128, nums, pos - 1)
        | is_valid_calibration(res - last_num as i128, nums, pos - 1)
        | is_valid_calibration(res * last_num as i128, nums, pos - 1);
    if res % last_num as i128 == 0 {
        valid |= is_valid_calibration(res / last_num as i128, nums, pos - 1);
    }

    valid
}

fn is_valid_calibration_with_concat(res: i128, acc: i128, nums: &Vec<u128>, pos: usize) -> bool {
    if pos == nums.len() {
        return acc == res;
    }

    let curr_num = *nums.get(pos).unwrap();

    let mut valid = false;
    match acc.checked_add(curr_num as i128) {
        Some(x) => valid |= is_valid_calibration_with_concat(res, x, nums, pos + 1),
        None => {}
    }
    match acc.checked_sub(curr_num as i128) {
        Some(x) => valid |= is_valid_calibration_with_concat(res, x, nums, pos + 1),
        None => {}
    }
    match acc.checked_mul(curr_num as i128) {
        Some(x) => valid |= is_valid_calibration_with_concat(res, x, nums, pos + 1),
        None => {}
    }
    if acc % curr_num as i128 == 0 {
        valid |= is_valid_calibration_with_concat(res, acc / curr_num as i128, nums, pos + 1);
    }

    match format!("{}{}", acc, curr_num).parse::<i128>() {
        Ok(x) => valid |= is_valid_calibration_with_concat(res, x, nums, pos + 1),
        Err(_) => {}
    }

    valid
}

fn main() {
    let input_file_path = "src/bin/day07/input.txt";

    let input = parse_input(input_file_path);

    let Ok(calibration) = input else {
        panic!("{}", input.err().unwrap());
    };

    let (mut total_calibration, mut total_calibration_with_concat) = (0, 0);
    for (res, nums) in calibration {
        if is_valid_calibration(res, &nums, nums.len() - 1) {
            total_calibration += res;
        }
        if is_valid_calibration_with_concat(res, nums[0] as i128, &nums, 1) {
            total_calibration_with_concat += res;
        }
    }

    println!("{}", total_calibration);
    println!("{}", total_calibration_with_concat);
}
