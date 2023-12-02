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
    fn from_count(count: &str) -> RGB {
        todo!()
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
                picks: vec![RGB(2, 1, 4), RGB(6, 1, 4)],
            }
        )
    }
}
