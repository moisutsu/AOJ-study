fn main() {
    let n: usize = read();
    let (mut taro, mut hanako) = (0, 0);
    for _ in 0..n {
        let cards: Vec<String> = read_vec();
        // String型は辞書順で比較できる
        if cards[0] < cards[1] {
            hanako += 3;
        } else if cards[0] > cards[1] {
            taro += 3;
        } else {
            taro += 1;
            hanako += 1;
        }
    }
    println!("{} {}", taro, hanako);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}
