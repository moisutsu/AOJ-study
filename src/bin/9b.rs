fn main() {
    loop {
        let mut w: String = read();
        if w == "-" {
            break;
        }
        let m = read();
        for _ in 0..m {
            let h = read();
            let front = &w[..h];
            let back = &w[h..];
            w = format!("{}{}", back.to_string(), front.to_string());
        }
        println!("{}", w);
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
