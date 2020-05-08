#[macro_use]
mod input;

fn main() {
    input! {
        n: usize,
        xs: [f32; n],
        ys: [f32; n],
    }
    println!("{}", (0..n).map(|i| (xs[i] - ys[i]).abs()).sum::<f32>());
    println!(
        "{}",
        (0..n)
            .map(|i| (xs[i] - ys[i]).powi(2))
            .sum::<f32>()
            .powf(0.5)
    );
    println!(
        "{}",
        (0..n)
            .map(|i| (xs[i] - ys[i]).abs().powi(3))
            .sum::<f32>()
            .powf(1.0 / 3.0)
    );
    println!(
        "{}",
        (0..n)
            .map(|i| (xs[i] - ys[i]).abs())
            .fold(0.0 / 0.0, |m, v| v.max(m))
    );
}
