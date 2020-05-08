use std::io::*;

fn main() {
    let mut w = String::new();
    stdin().read_line(&mut w).unwrap();
    w = w.trim().to_string();
    let mut count = 0;
    loop {
        let mut t = String::new();
        stdin().read_line(&mut t).unwrap();
        t = t.trim().to_string();
        if t == "END_OF_TEXT" {
            break;
        }
        count += t.match_indices(&w).count();
    }
    println!("{}", count);
}
