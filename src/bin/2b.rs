#[macro_use]
mod input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }
    let s = if a < b && b < c { "Yes" } else { "No" };
    println!("{}", s);
}
