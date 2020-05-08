use std::io::*;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    println!(
        "{}",
        s.chars()
            .map(|c| if c.is_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c.to_ascii_uppercase()
            })
            .collect::<String>()
    );
}
