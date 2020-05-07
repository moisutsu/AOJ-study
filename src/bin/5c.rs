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
        print_chessboard(h, w);
    }
}

fn print_chessboard(h: usize, w: usize) {
    let line = "#.".to_string().repeat(w / 2 + 1);
    for i in 0..h {
        let (s, e) = if i % 2 == 0 { (0, w) } else { (1, w + 1) };
        println!("{}", &line[s..e]);
    }
    println!("");
}
