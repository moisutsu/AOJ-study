#[macro_use]
mod input;

fn main() {
    input! {
        a: isize,
        b: isize,
    }
    println!("{} {} {:.5}", a / b, a % b, a as f32 / b as f32);
}
