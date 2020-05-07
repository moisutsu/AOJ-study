#[macro_use]
mod input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let count = (a..b + 1).filter(|x| c % x == 0).count();
    println!("{}", count);
}
