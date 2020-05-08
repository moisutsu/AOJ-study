#[macro_use]
mod input;

fn main() {
    input! {
        x1: f32, y1: f32, x2: f32, y2: f32,
    }
    println!("{}", ((x1 - x2).powi(2) + (y1 - y2).powi(2)).powf(0.5));
}
