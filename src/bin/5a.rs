#[macro_use]
mod input;

fn main() {
    loop {
        input! {
            h: usize,
            w: usize,
        }
        if h == 0 && w == 0 {
            break;
        }
        print_rectangle(h, w);
    }
}

fn print_rectangle(h: usize, w: usize) {
    let line = "#".to_string().repeat(w);
    for _ in 0..h {
        println!("{}", line);
    }
    println!("");
}
