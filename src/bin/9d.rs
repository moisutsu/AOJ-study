fn main() {
    let mut w: String = read();
    let q: u32 = read();
    let commands = read_vec2::<String>(q);
    for command in commands {
        match command[0].as_str() {
            "print" => {
                let (a, b): (usize, usize) = (
                    command[1].parse().unwrap(),
                    command[2].parse().unwrap()
                );
                println!("{}", &w[a..b + 1]);
            },
            "reverse" => {
                let (a, b): (usize, usize) = (
                    command[1].parse().unwrap(),
                    command[2].parse().unwrap()
                );
                w = format!("{}{}{}",
                    &w[..a],
                    &w[a..b + 1].chars().rev().collect::<String>(),
                    &w[b + 1..]
                );
            },
            "replace" => {
                let (a, b, c): (usize, usize, &str) = (
                    command[1].parse().unwrap(),
                    command[2].parse().unwrap(),
                    &command[3]
                );
                w = format!("{}{}{}",
                    &w[..a],
                    c,
                    &w[b + 1..]
                );
            },
            _ => break,
        }
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
