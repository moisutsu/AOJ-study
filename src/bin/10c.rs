fn main() {
    loop {
        let n = read::<f32>();
        if n == 0.0 {
            break;
        }
        let s = read_vec::<f32>();
        let m = s.iter().sum::<f32>() / n;
        println!(
            "{}",
            (s.iter().map(|x| (x - m).powi(2)).sum::<f32>() / n).powf(0.5)
        );
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}
