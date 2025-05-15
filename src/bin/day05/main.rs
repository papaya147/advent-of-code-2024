use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_input(file_path: &str) -> Result<(HashMap<(u8, u8), bool>, Vec<Vec<u8>>), Box<dyn Error>> {
    let mut page_orders = HashMap::new();
    let mut pages = Vec::new();

    let file = File::open(file_path)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        if line.contains("|") {
            let mut nums = line.split("|");
            let (Ok(pre), Ok(post)) = (nums.next().unwrap().parse(), nums.next().unwrap().parse())
            else {
                panic!("not enough numbers");
            };
            page_orders.insert((pre, post), true);
        } else if line.contains(",") {
            pages.push(line.split(",").map(|x| x.parse().unwrap()).collect());
        }
    }

    Ok((page_orders, pages))
}

fn valid_page(page_orders: &HashMap<(u8, u8), bool>, page: &Vec<u8>) -> u8 {
    if page
        .iter()
        .enumerate()
        .map(|(i, p)| {
            page.iter()
                .skip(i + 1)
                .fold(true, |f, x| f & page_orders.contains_key(&(*p, *x)))
        })
        .fold(true, |f, x| f & x)
    {
        *page.get(page.len() / 2).unwrap()
    } else {
        0
    }
}

fn invalid_page_mid_value(page_orders: &HashMap<(u8, u8), bool>, page: &Vec<u8>) -> u8 {
    page.iter()
        .map(|x| {
            (
                x,
                page.iter()
                    .map(|y| u8::from(page_orders.contains_key(&(*y, *x))))
                    .sum::<u8>(),
            )
        })
        .filter(|(_, pos)| *pos as usize == page.len() / 2)
        .map(|(x, _)| *x)
        .sum()
}

fn main() {
    let input_file_path = "src/bin/day05/input.txt";

    let input = parse_input(input_file_path);

    let Ok((page_orders, pages)) = input else {
        panic!("{}", input.err().unwrap());
    };

    println!(
        "{}",
        pages
            .iter()
            .map(|page| valid_page(&page_orders, page) as u32)
            .sum::<u32>()
    );
    println!(
        "{}",
        pages
            .iter()
            .map(|page| {
                if valid_page(&page_orders, page) == 0 {
                    invalid_page_mid_value(&page_orders, page) as u32
                } else {
                    0
                }
            })
            .sum::<u32>()
    )
}
