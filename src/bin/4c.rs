#[macro_use]
mod input;

fn main() {
    loop {
        input! {
            a: isize,
            op: char,
            b: isize,
        }
        let ans = match op {
            '+' => a + b,
            '-' => a - b,
            '/' => a / b,
            '*' => a * b,
            _ => break,
        };
        println!("{}", ans);
    }
}
