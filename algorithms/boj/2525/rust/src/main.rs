use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(& mut buffer)
        .unwrap();

    let mut input = buffer
        .split_ascii_whitespace()
        .flat_map(str::parse::<i32>);

    let start_hour = input.next().unwrap();
    let start_minute = input.next().unwrap();
    let cooking_time = input.next().unwrap();

    let (hour, minute) = solve(start_hour, start_minute, cooking_time);

    println!("{} {}", hour, minute);
}

fn solve(
    start_hour: i32, 
    start_minute: i32, 
    cooking_time: i32) -> (i32, i32) {
    let total_minutes = start_hour * 60 + start_minute + cooking_time;
    
    let hour = (total_minutes / 60) % 24;
    let minute = total_minutes % 60;

    (hour, minute)
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve(14, 30, 20), (14, 50));
    }

    #[test]
    fn test2() {
        assert_eq!(solve(17, 40, 80), (19, 0));
    }

    #[test]
    fn test3() {
        assert_eq!(solve(23, 48, 25), (0, 13));
    }
}
