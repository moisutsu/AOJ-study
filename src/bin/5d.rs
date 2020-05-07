#[macro_use]
mod input;

fn main() {
    input! {
        n: usize,
    }
    for i in 1..n + 1 {
        if i % 3 == 0 {
            print!(" {}", i);
        } else if include_three(i) {
            print!(" {}", i);
        }
    }
    println!("");
}

fn include_three(n: usize) -> bool {
    let mut x = n;
    while x != 0 {
        if x % 10 == 3 {
            return true;
        }
        x /= 10;
    }
    false
}
