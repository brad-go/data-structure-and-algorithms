use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut input = buffer
        .split_ascii_whitespace()
        .flat_map(str::parse::<i32>);

    let h = input.next().unwrap();
    let m = input.next().unwrap();

    let (hour, minute) = solve(h, m);

    println!("{} {}", hour, minute);
}

fn solve(h: i32, m: i32) -> (i32, i32) {
    let total_minutes = h * 60 + m;
    let day_minutes = 1440;
    let adjusted = (total_minutes + day_minutes - 45) % day_minutes;

    let hour = adjusted / 60;
    let minute = adjusted % 60;

    (hour, minute)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve(10, 10), (9, 25));
    }

    #[test]
    fn test2() {
        assert_eq!(solve(0, 30), (23, 45));
    }

    #[test]
    fn test3() {
        assert_eq!(solve(23, 40), (22, 55));
    }
}