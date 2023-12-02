//!
//! Day 2
//! Part 1
//!
//!  - &Cherry, 12/1/2023
//!

mod macros;

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,

    picks: Vec<RGB>,
}

impl Game {
    fn from_line(line: &str) -> Self {
        let mut words = line.split_whitespace();

        // skip "Game"
        words.next();

        let id = {
            // skip the colon
            let mut it = words.next().unwrap().chars();
            it.next_back();

            it.as_str().parse::<u32>().unwrap()
        };

        // this has the side effect of removing all
        // the whitespace, which is good for us here.
        let counts = words.collect::<String>();
        let counts = counts.split(';');

        let picks = counts.fold(vec![], |mut res, count| {
            res.push(RGB::from_count(count));
            res
        });

        Self { id, picks }
    }
}

#[derive(Debug, PartialEq)]
struct RGB(u32, u32, u32);

impl RGB {
    fn from_count(count: &str) -> Self {
        let mut iter = count.split(',');
        let mut res = Self(0, 0, 0);

        while let Some(count) = iter.next() {
            let mut chars = count.chars();

            let cube_count = chars
                .by_ref()
                .take_while(|v| v.to_digit(10).is_some())
                .collect::<String>();
            let cube_count = cube_count.parse::<u32>().unwrap();

            let color = chars.collect::<String>();
            let color_ref = match color.as_str() {
                // think dumber, not harder :3
                "ed" => &mut res.0,
                "reen" => &mut res.1,
                "lue" => &mut res.2,
                x @ _ => panic!("wtf is a {}", x),
            };

            *color_ref = cube_count;
        }

        res
    }
}

fn main() {
    let input = inputfile!("2.txt");

    let _lines = input.lines().map(Game::from_line);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_works() {
        let line = "Game 1: 4 blue; 1 green, 2 red; 4 blue, 1 green, 6 red";
        let game = Game::from_line(line);

        assert_eq!(
            game,
            Game {
                id: 1,
                picks: vec![RGB(0, 0, 4), RGB(2, 1, 0), RGB(6, 1, 4)],
            }
        )
    }
}
