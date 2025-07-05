use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut input = buffer
        .split_ascii_whitespace()
        .flat_map(str::parse::<usize>);

    let n = input.next().unwrap();

    solve(n);
}

fn solve(n: usize) {
    for i in 1..10 {
        println!("{} * {} = {}", n, i, n * i);
    }
}