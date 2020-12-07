#[macro_use]
extern crate lazy_static;

use regex::Regex;

lazy_static! {
    static ref POLICY: Regex = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<chr>.)$").unwrap();
}

fn is_valid(password: &str, policy: &str) -> bool {
    let caps = POLICY.captures(policy).unwrap();
    let min = caps["min"].parse::<usize>().unwrap();
    let max = caps["max"].parse::<usize>().unwrap();
    let chr = &caps["chr"].chars().next().unwrap();
    let cnt = password.chars().filter(|c| c == chr).count();
    cnt >= min && cnt <= max
}

fn is_valid_alt(password: &str, policy: &str) -> bool {
    let caps = POLICY.captures(policy).unwrap();
    let p1 = caps["min"].parse::<usize>().unwrap();
    let p2 = caps["max"].parse::<usize>().unwrap();
    let chr = &caps["chr"].chars().next().unwrap();
    let c1 = &password.chars().nth(p1 - 1).unwrap();
    let c2 = &password.chars().nth(p2 - 1).unwrap();
    c1 != c2 && (c1 == chr || c2 == chr)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("data/day2.txt")?;
    let n_valid = input
        .split('\n')
        .filter(|spec| {
            let mut parts = spec.split(": ");
            let policy = parts.next().unwrap();
            let password = parts.next().unwrap();
            is_valid_alt(password, policy)
        })
        .count();
    println!("{}", n_valid);
    Ok(())
}
