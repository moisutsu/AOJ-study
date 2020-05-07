#[macro_use]
mod input;

use std::f32::consts::PI;

fn main() {
    input! {
        r: f32,
    }
    println!("{} {}", PI * r * r, 2.0 * PI * r);
}
