#[macro_use]
mod input;

fn main() {
    loop {
        input! {
            n: usize,
            x: usize,
        }
        if n == 0 && x == 0 {
            break;
        }
        let mut count = 0;
        for i in 1..n - 1 {
            for j in i + 1..n {
                for k in j + 1..n + 1 {
                    if i + j + k == x {
                        count += 1;
                    }
                }
            }
        }
        println!("{}", count);
    }
}
