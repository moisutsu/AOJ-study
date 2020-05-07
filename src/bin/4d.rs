#[macro_use]
mod input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    println!(
        "{} {} {}",
        a.iter().min().unwrap(),
        a.iter().max().unwrap(),
        a.iter().sum::<i32>(),
    );
}
