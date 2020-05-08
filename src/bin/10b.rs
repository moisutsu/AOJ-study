#[macro_use]
mod input;

use std::f32::consts::PI;

fn main() {
    input! {
        a: f32,
        b: f32,
        c: f32
    }
    let s = 0.5 * a * b * (c / 180.0 * PI).sin();
    println!("{}", s);
    println!(
        "{}",
        (a.powi(2) + b.powi(2) - 2.0 * a * b * (c / 180.0 * PI).cos()).powf(0.5) + a + b
    );
    println!("{}", s / a * 2.0);
}
