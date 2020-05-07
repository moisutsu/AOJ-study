#[macro_use]
mod input;

fn main() {
    let mut c = 1;
    loop {
        input! {a: usize};
        if a == 0 {
            break;
        };
        println!("Case {}: {}", c, a);
        c += 1;
    }
}
