#[macro_use]
mod input;

fn main() {
    input!{
        x: usize
    }
    println!("{}", x.pow(3));
}
