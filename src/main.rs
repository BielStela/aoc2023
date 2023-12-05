use reqwest::Error;
use reqwest::blocking::Client;
use reqwest::header::{COOKIE, HeaderValue};

mod day1;
mod day2;

fn main() {
    let day1_input = get_puzzle_input(1).unwrap();
    println!("Day 1.1: {}", day1::part1(&day1_input));
    println!("Day 1.2: {}", day1::part2(&day1_input));

    let day2_input = get_puzzle_input(2).unwrap();
    println!("Day 2.1: {}", day2::part1(&day2_input));
    println!("Day 2.2: {}", day2::part2(&day2_input));
}


fn get_puzzle_input(day: u8) -> Result<String, Error> {
    dotenv::dotenv().ok();
    let session_cookie = std::env::var("SESSION_COOKIE")
        .expect("SESSION_COOKIE not set");

    let url = format!("https://adventofcode.com/2023/day/{}/input", day);
    let client = Client::new();
    let cookie = HeaderValue::from_str(&session_cookie).unwrap();
    let response = client.get(&url)
        .header(COOKIE, cookie)
        .send()?
        .text()?;
    Ok(response)
}
