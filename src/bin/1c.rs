#[macro_use]
mod input;

fn main() {
    input!{
        a: usize,
        b: usize,
    }
    println!("{} {}", a * b, a * 2 + b * 2);
}
