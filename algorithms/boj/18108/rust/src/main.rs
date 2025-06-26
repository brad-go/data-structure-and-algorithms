use std::io::{self, BufRead};

fn main() {
    let mut reader = create_reader();
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();

    let buddha_era = input.trim().parse::<i32>().unwrap();
    let anno_domini = to_anno_domini(buddha_era);

    println!("{}", anno_domini);
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

fn to_anno_domini(buddha_era: i32) -> i32 {
    buddha_era - 543
}