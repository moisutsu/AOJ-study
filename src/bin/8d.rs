#[macro_use]
mod input;

fn main() {
    input! {
        s: String,
        p: String,
    }
    let mut ans = "No";
    for s_i in 0..s.len() {
        let mut count = 0;
        for p_i in 0..p.len() {
            let index = if s_i + p_i >= s.len() { s_i + p_i - s.len()} else { s_i + p_i};
            if s.chars().nth(index) == p.chars().nth(p_i) {
                count += 1;
            } else {
                break;
            }
        }
        if count == p.len() {
            ans = "Yes";
            break;
        }
    }
    println!("{}", ans);
}
