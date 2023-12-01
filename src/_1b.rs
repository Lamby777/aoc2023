use crate::_1a::parse_line;

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
    let chars = line.chars().collect::<Vec<_>>();

    // last "word" number was this many letters ago
    let mut debounce = 0usize;

    for i in 0..chars.len() {
        for (k, v) in NUMBERS {
            let k_chars = k.chars().collect::<Vec<_>>();

            let digit = if debounce == 0 && chars[i..].starts_with(&k_chars) {
                debounce = k.len();
                v
            } else if let Some(n) = chars[i].to_digit(10) {
                debounce = debounce.saturating_sub(1);
                n
            } else {
                debounce = debounce.saturating_sub(1);
                continue;
            };

            res.push(char::from_digit(digit, 10).unwrap());
        }
    }

    for (k, v) in NUMBERS {
        res = res.replace(k, &v.to_string())
    }

    res
}
