use std::env;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut res = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        res.push(line.to_string())
    }
    res
}

fn count_first_and_last_digits(line: &str) -> u32 {
    let left = line.as_bytes().iter().find(|&c| c.is_ascii_digit());
    let right = line.as_bytes().iter().rev().find(|&c| c.is_ascii_digit());
    match left {
        Some(l) => match right {
            Some(r) => return ((l - 48) * 10 + (r - 48)).clone().into(),
            None => return ((l - 48) as u32 * 10 + (l - 48) as u32).clone().into(),
        },
        None => match left {
            Some(r) => return ((r - 48) as u32 * 10 + (r - 48) as u32).clone().into(),
            None => return 0,
        },
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("wrong amount of argument");
    }
    // let filename = File::open(&args[1]).unwrap();
    let a = &args[1];

    let mut count: u32 = 0;
    for line in read_lines(a) {
        count += count_first_and_last_digits(line.as_str());
    }
    println!("{count}")
}
