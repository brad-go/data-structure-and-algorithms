use std::{io::{self, BufRead}};

fn main() {
    let mut reader = create_reader();
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    
    let t = input.trim().parse::<usize>().unwrap();

    for _i in 0..t {
        reader.read_line(&mut input).unwrap();
    }

    let numbers: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut result = String::new();

    for i in 1..numbers.len() {
        let n = numbers.get(i).unwrap();
        let binary = convert_to_binary(*n);
        let rev = binary.chars().rev().collect::<String>();

        for i in 0..rev.len() {
            if rev.chars().nth(i).unwrap() == '0' {
                continue;  
            } 
            
            result += i.to_string().as_str();  
            result += " ";
        }

        result += "\n";
    }
    
    println!("{}", result.trim());
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

fn convert_to_binary(number: usize) -> String {
    format!("{:b}", number)
}