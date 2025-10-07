use std::io::{self, BufRead};

fn main() {
    let mut reader = create_reader();
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();

    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let n = numbers[0];
    let k = numbers[1];

    let mut divisors: Vec<i32> = Vec::new();

    for i in 1..=n / 2 {
        if n % i == 0 {
            divisors.push(i);
        }
    }

    divisors.push(n);

    let result = if divisors.len() < k as usize {
        0
    } else {
        divisors[k as usize - 1]
    };

    println!("{}", result);
}

fn create_reader() -> Box<dyn BufRead> {
    let reader: Box<dyn BufRead> = if cfg!(debug_assertions) {
        let file = std::fs::File::open("../input.txt").expect("input.txt 파일을 열 수 없습니다.");
        Box::new(io::BufReader::new(file))
    } else {
        Box::new(io::BufReader::new(io::stdin()))
    };

    reader
}