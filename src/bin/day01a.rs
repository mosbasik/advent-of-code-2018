use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let sum: i32 = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .sum();

    println!("{:}", sum);
}
