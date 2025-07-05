use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let year: i32 = input.trim().parse().expect("Invalid input");

    println!("{}", is_leap_year(year) as u8);
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(is_leap_year(2000));
    }

    #[test]
    fn test2() {
        assert!(!is_leap_year(1999));
    }

    #[test]
    fn test3() {
        assert!(is_leap_year(2012));
    }

    #[test]
    fn test4() {
        assert!(!is_leap_year(1900));
    }
}