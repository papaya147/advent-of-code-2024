use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_input(file_path: &str) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let mut puzzle = Vec::new();

    let file = File::open(file_path)?;
    for line in BufReader::new(file).lines() {
        let mut temp_vec = vec![' '];
        temp_vec.extend(line?.chars());
        temp_vec.push(' ');
        puzzle.push(temp_vec);
    }

    // adding empty space padding to top and bottom
    puzzle.insert(0, vec![' '; puzzle.len() + 2]);
    puzzle.push(vec![' '; puzzle.len() + 1]);

    Ok(puzzle)
}

fn next_letter_xmas(c: char) -> char {
    match c {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        _ => ' ',
    }
}

fn check_xmas(
    expected_letter: char,
    (x, y): (usize, usize),
    (step_x, step_y): (i32, i32),
    puzzle: &Vec<Vec<char>>,
) -> bool {
    match puzzle.get(x).unwrap().get(y).unwrap() {
        c if *c == 'S' && *c == expected_letter => true,
        c if *c == expected_letter => check_xmas(
            next_letter_xmas(*c),
            ((x as i32 + step_x) as usize, (y as i32 + step_y) as usize),
            (step_x, step_y),
            puzzle,
        ),
        _ => false,
    }
}

fn check_x(puzzle: &Vec<Vec<char>>) -> u32 {
    let steps = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut count = 0;
    for (x, row) in puzzle.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if *cell == 'X' {
                for step in steps {
                    count += u32::from(check_xmas('X', (x, y), step, puzzle));
                }
            }
        }
    }
    count
}

fn check_mas(
    expected_letter: char,
    (x, y): (usize, usize),
    (step_x, step_y): (i32, i32),
    puzzle: &Vec<Vec<char>>,
) -> bool {
    *puzzle
        .get((x as i32 + step_x) as usize)
        .unwrap()
        .get((y as i32 + step_y) as usize)
        .unwrap()
        == expected_letter
}

fn check_a(puzzle: &Vec<Vec<char>>) -> u32 {
    let steps = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

    let mut count = 0;
    for (x, row) in puzzle.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if *cell == 'A' {
                let mut flag = false;
                for (step_x, step_y) in steps {
                    flag |= check_mas('M', (x, y), (step_x, step_y), puzzle)
                        & check_mas('S', (x, y), (-step_x, -step_y), puzzle)
                        & ((check_mas('M', (x, y), (step_x, -step_y), puzzle)
                            & check_mas('S', (x, y), (-step_x, step_y), puzzle))
                            | (check_mas('M', (x, y), (-step_x, step_y), puzzle)
                                & check_mas('S', (x, y), (step_x, -step_y), puzzle)));
                }
                if flag {
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    let input_file_path = "src/bin/day04/input.txt";

    let input = parse_input(input_file_path);

    let Ok(puzzle) = input else {
        panic!("{}", input.err().unwrap());
    };

    println!("{}", check_x(&puzzle));
    println!("{}", check_a(&puzzle));
}
