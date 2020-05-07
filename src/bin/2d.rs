#[macro_use]
mod input;

fn main() {
    input! {
        w: isize,
        h: isize,
        x: isize,
        y: isize,
        r: isize,
    }
    let ans = if x - r >= 0 && x + r <= w && y - r >= 0 && y + r <= h {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
