#![allow(dead_code, unused_imports)]
use regex::Regex;

fn main() {
    let re_spaces = Regex::new(r"(?m)>\s+?<").unwrap();
    let mut body = get_basic();

    body = re_spaces.replace_all(&body, "><").to_string();
    println!("{:?}", body);
}

fn get_basic() -> String {
    let body = reqwest::blocking::get("https://www.rust-lang.org")
        .expect("method get failed")
        .text()
        .expect("failed to parse")
        .trim()
        .to_string();

    body
}
