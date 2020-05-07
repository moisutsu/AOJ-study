#[macro_use]
mod input;

fn main() {
    loop {
        input! {
            x: usize,
            y: usize,
        };
        if x == 0 && y == 0 {
            break;
        };
        let s = if x <= y {
            format!("{} {}", x, y)
        } else {
            format!("{} {}", y, x)
        };
        println!("{}", s);
    }
}
