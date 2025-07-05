use std::io::{self, BufRead};

fn main() {
    let mut reader = create_reader();
    let mut input = String::new();

    reader
        .read_line(&mut input)
        .unwrap();

    let number: i32 = input
        .trim()
        .parse()
        .unwrap();

    let grade = if number >= 90 {
        "A"
    } else if number >= 80 {
        "B"
    } else if number >= 70 {
        "C"
    } else if number >= 60 {
        "D"
    } else {
        "F"
    };

    println!("{}", grade);
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