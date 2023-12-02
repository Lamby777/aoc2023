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

pub fn main() {
    let input = include_str!("inputs/1.txt");

    let sum = input
        .lines()
        .map(preprocess_replace)
        .map(|v| parse_line(&v))
        .sum::<u32>();
    println!("{}", sum);
}

pub fn parse_line(line: &str) -> u32 {
    let mut chars = line.chars().filter(|v| v.is_digit(10));

    fn char_to_digit(c: char) -> u32 {
        c.to_digit(10).unwrap()
    }

    let n_first = chars.next().map(char_to_digit).unwrap();
    let n_last = chars.next_back().map(char_to_digit);
    let n_last = n_last.unwrap_or(n_first);

    (n_first * 10) + n_last
}

fn preprocess_replace(line: &str) -> String {
    let mut res = "".to_owned();
    let chars = line.chars().collect::<Vec<_>>();

    for i in 0..chars.len() {
        for (k, v) in NUMBERS {
            let k_chars = k.chars().collect::<Vec<_>>();

            let digit = if chars[i..].starts_with(&k_chars) {
                v
            } else if let Some(n) = chars[i].to_digit(10) {
                n
            } else {
                continue;
            };

            res.push(char::from_digit(digit, 10).unwrap());
        }
    }

    res
}
