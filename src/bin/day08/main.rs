use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_input(file_path: &str) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let mut antennas = Vec::new();

    let file = File::open(file_path)?;
    for line in BufReader::new(file).lines() {
        antennas.push(line?.chars().collect());
    }

    Ok(antennas)
}

fn antenna_positions(antennas: &Vec<Vec<char>>) -> HashMap<char, Vec<(i32, i32)>> {
    let mut positions = HashMap::new();

    for (i, antenna_row) in antennas.iter().enumerate() {
        for (j, antenna) in antenna_row.iter().enumerate() {
            if *antenna == '.' {
                continue;
            }
            positions
                .entry(*antenna)
                .or_insert_with(Vec::new)
                .push((i as i32, j as i32));
        }
    }

    positions
}

fn unique_pairs<T: Clone>(items: &Vec<T>) -> Vec<(T, T)> {
    items
        .iter()
        .enumerate()
        .flat_map(|(i, x)| items.iter().skip(i + 1).map(|y| (x.clone(), y.clone())))
        .collect()
}

fn antenna_translations(antennas: &Vec<Vec<char>>) -> u32 {
    let mut antennas = antennas.clone();
    let antinode = '*';

    for (_, positions) in antenna_positions(&antennas).iter() {
        for ((x1, y1), (x2, y2)) in unique_pairs(positions) {
            let x3 = 2 * x2 - x1;
            let y3 = 2 * y2 - y1;
            let x4 = 2 * x1 - x2;
            let y4 = 2 * y1 - y2;

            if x3 >= 0 && x3 < antennas.len() as i32 && y3 >= 0 && y3 < antennas[0].len() as i32 {
                antennas[x3 as usize][y3 as usize] = antinode;
            }

            if x4 >= 0 && x4 < antennas.len() as i32 && y4 >= 0 && y4 < antennas[0].len() as i32 {
                antennas[x4 as usize][y4 as usize] = antinode;
            }
        }
    }

    antennas
        .iter()
        .map(|row| row.iter().filter(|antenna| **antenna == antinode).count() as u32)
        .sum()
}

fn antenna_translations_with_harmony(antennas: &Vec<Vec<char>>) -> u32 {
    let mut antennas = antennas.clone();
    let antinode = '*';

    for (_, positions) in antenna_positions(&antennas).iter() {
        for ((x1, y1), (x2, y2)) in unique_pairs(positions) {
            let x3_step = x2 - x1;
            let y3_step = y2 - y1;
            let x4_step = x1 - x2;
            let y4_step = y1 - y2;

            let mut x3 = x2 + x3_step;
            let mut y3 = y2 + y3_step;
            let mut x4 = x1 + x4_step;
            let mut y4 = y1 + y4_step;

            while x3 >= 0 && x3 < antennas.len() as i32 && y3 >= 0 && y3 < antennas[0].len() as i32
            {
                antennas[x3 as usize][y3 as usize] = antinode;
                x3 += x3_step;
                y3 += y3_step;
            }

            while x4 >= 0 && x4 < antennas.len() as i32 && y4 >= 0 && y4 < antennas[0].len() as i32
            {
                antennas[x4 as usize][y4 as usize] = antinode;
                x4 += x4_step;
                y4 += y4_step;
            }
        }
    }

    antennas
        .iter()
        .map(|row| row.iter().filter(|antenna| **antenna != '.').count() as u32)
        .sum()
}

fn main() {
    let input_file_path = "src/bin/day08/input.txt";

    let input = parse_input(input_file_path);

    let Ok(antennas) = input else {
        panic!("{}", input.err().unwrap());
    };

    println!("{:?}", antenna_translations(&antennas));
    println!("{:?}", antenna_translations_with_harmony(&antennas));
}
