#[macro_use]
mod input;

fn main() {
    loop {
        input! {
            x: i128,
        }
        if x == 0 {
            break;
        }
        let mut n = x;
        let mut total = 0;
        while n != 0 {
            total += n % 10;
            n /= 10;
        }
        println!("{}", total);
    }
}
