use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read line");

    let mut numbers = input
        .split_ascii_whitespace()
        .flat_map(str::parse::<i32>);

    let x = numbers.next().unwrap();
    let y = numbers.next().unwrap();

    let quadrant = solve(x, y);

    println!("{}", quadrant);
}

fn solve(x: i32, y: i32) -> i32 {
    let quadrant = if x > 0 && y > 0 {
        1
    } else if x < 0 && y > 0 {
        2
    } else if x < 0 && y < 0 {
        3
    } else {
        4
    };

    quadrant
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve(12, 5), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(9, -13), 4);
    }
}
