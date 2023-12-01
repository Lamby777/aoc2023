//!
//! yippee yup yippers
//! uwu
//!
//!  - &Cherry, 12/1/2023
//!

use reqwest::blocking as reqwest;
mod _1a;

fn main() {
    let in_url = _1a::DATA_URL;
    let in_data = reqwest::get(in_url).unwrap().text().unwrap();
    _1a::run(in_data);
}
