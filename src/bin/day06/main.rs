use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_input(file_path: &str) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let mut map = Vec::new();

    let file = File::open(file_path)?;
    for line in BufReader::new(file).lines() {
        map.push(line?.chars().collect());
    }

    Ok(map)
}

fn turn((x, y): (i16, i16)) -> (i16, i16) {
    match (x, y) {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        _ => (x, y),
    }
}

fn guard_start(map: &Vec<Vec<char>>) -> (i16, i16) {
    let map = map.clone();

    // getting starting position
    let Some((curr_x, curr_y)) = map.iter().enumerate().find_map(|(i, row)| {
        row.iter().enumerate().find_map(|(j, cell)| {
            if *cell == '^' {
                Some((i as i16, j as i16))
            } else {
                None
            }
        })
    }) else {
        panic!("could not find starting position")
    };

    (curr_x, curr_y)
}

fn mark_guard(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let (mut curr_x, mut curr_y) = guard_start(map);
    let (mut step_x, mut step_y) = (-1, -0);

    let mut map = map.clone();

    // moving the guard
    loop {
        map[curr_x as usize][curr_y as usize] = 'X'; // mark curr as 'X'

        if curr_x == 0
            || curr_x == (map.len() - 1) as i16
            || curr_y == 0
            || curr_y == (map[0].len() - 1) as i16
        {
            break;
        }

        match map
            .get((step_x + curr_x) as usize)
            .unwrap()
            .get((step_y + curr_y) as usize)
            .unwrap()
        {
            '#' => (step_x, step_y) = turn((step_x, step_y)), // turn
            '.' | 'X' => (curr_x, curr_y) = (curr_x + step_x, curr_y + step_y), // move ahead
            _ => {}
        }
    }

    map
}

fn is_guard_cycle(map: &Vec<Vec<char>>) -> bool {
    let (mut curr_x, mut curr_y) = guard_start(map);
    let (mut step_x, mut step_y) = (-1, -0);

    let mut map = map.clone();

    let mut pos_hist = HashMap::new();

    // moving the guard
    loop {
        match pos_hist.get(&((curr_x, curr_y), (step_x, step_y))) {
            Some(_) => return true,
            None => {}
        }

        map[curr_x as usize][curr_y as usize] = 'X'; // mark curr as 'X'
        pos_hist.insert(((curr_x, curr_y), (step_x, step_y)), true); // been in this position facing this particular direction

        if curr_x == 0
            || curr_x == (map.len() - 1) as i16
            || curr_y == 0
            || curr_y == (map[0].len() - 1) as i16
        {
            break;
        }

        match map
            .get((step_x + curr_x) as usize)
            .unwrap()
            .get((step_y + curr_y) as usize)
            .unwrap()
        {
            '#' => (step_x, step_y) = turn((step_x, step_y)),
            '.' | 'X' => (curr_x, curr_y) = (curr_x + step_x, curr_y + step_y), // move ahead
            _ => {}
        }
    }

    false
}

fn main() {
    let input_file_path = "src/bin/day06/input.txt";

    let input = parse_input(input_file_path);

    let Ok(map) = input else {
        panic!("{}", input.err().unwrap());
    };

    let marked_map = mark_guard(&map);

    println!(
        "{}",
        marked_map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| if *cell == 'X' { 1 } else { 0 })
                    .sum::<u32>()
            })
            .sum::<u32>()
    );

    let mut guard_cycles = 0;
    map.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, cell)| match cell {
            '.' => {
                let mut mod_map = map.clone();
                mod_map[i][j] = '#';
                guard_cycles += u32::from(is_guard_cycle(&mod_map));
            }
            _ => {}
        })
    });

    println!("{}", guard_cycles);
}
