use crate::_1a::parse_line;
use std::process::exit;

type NumberKVPair = (&'static str, u32);
const NUMBERS: [NumberKVPair; 10] = [
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

pub fn run(input: &str) {
    let sum = input
        .lines()
        .map(preprocess_replace)
        .map(|v| parse_line(&v))
        .sum::<u32>();
    println!("{}", sum);
}

fn preprocess_replace(line: &str) -> String {
    let mut res = line.to_owned();

    for (k, v) in NUMBERS {
        res = res.replace(k, &v.to_string())
    }

    res
}
