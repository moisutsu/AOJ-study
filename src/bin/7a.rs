#[macro_use]
mod input;

fn main() {
    loop {
        input! {
            m: i32,
            f: i32,
            r: i32,
        }
        if m == -1 && f == -1 && r == -1 {
            break;
        }
        let score = m + f;
        let s = match score {
            80..=100 => "A",
            65..=79 => "B",
            50..=64 => "C",
            30..=49 => {
                if r >= 50 {
                    "C"
                } else {
                    "D"
                }
            }
            _ => "F",
        };
        println!("{}", s);
    }
}
