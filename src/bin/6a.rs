#[macro_use]
mod input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut v = a.clone();
    v.sort();
    v.reverse();
    print_vec(v);
}

fn print_vec(v: Vec<i32>) {
    for i in 0..v.len() - 1 {
        print!("{} ", v[i]);
    }
    println!("{}", v[v.len() - 1]);
}
