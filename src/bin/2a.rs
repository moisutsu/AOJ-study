#[macro_use]
mod input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    let s = if a > b {
        "a > b"
    } else if a < b {
        "a < b"
    } else {
        "a == b"
    };
    println!("{}", s);
}
