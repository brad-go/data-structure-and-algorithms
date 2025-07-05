use std::io;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).unwrap();

    let n: usize = buffer.trim().parse().unwrap();

    solve(n);
}

fn solve(n: usize) {
    let mut result = String::with_capacity(n * (n + 1) / 2);

    for i in 1..n + 1 {
        result.push_str(&"*".repeat(i));

        if i < n {
            result.push('\n');
        }
    }

    println!("{}", result);
}
