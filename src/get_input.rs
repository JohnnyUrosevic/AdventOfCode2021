use std::env;
use reqwest::header::COOKIE;

pub fn get_input(day: i8) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = format!("https://adventofcode.com/2021/day/{}/input", day);
    let session = env::var("SESSION").expect("Please set the SESSION environment variable");

    let client = reqwest::blocking::Client::new();
    let body = client.get(url)
        .header(COOKIE, format!("session={}", session))
        .send()?
        .text()
        .expect("Could not send request");

    Ok(body.lines().map(String::from).collect())
}
