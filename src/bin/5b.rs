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
        print_frame(h, w);
    }
}

fn print_frame(h: usize, w: usize) {
    let line_out = "#".to_string().repeat(w);
    let line_in = format!("#{}#", ".".to_string().repeat(w - 2));
    println!("{}", line_out);
    for _ in 0..h - 2 {
        println!("{}", line_in);
    }
    println!("{}\n", line_out);
}
