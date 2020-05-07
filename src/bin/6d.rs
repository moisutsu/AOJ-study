#[macro_use]
mod input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[i32; m]; n],
        b: [i32; m],
    }
    for i in 0..n {
        let mut total = 0;
        for j in 0..m {
            total += a[i][j] * b[j];
        }
        println!("{}", total);
    }
}
