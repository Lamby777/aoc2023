pub fn run(input: &str) {
    let values = input.lines().map(parse_line);
    let sum = values.sum::<u32>();
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
