#[macro_use]
mod input;

fn main() {
    input! {
        v: [i32; 3],
    }
    let mut vm = v.clone();
    vm.sort();
    print_vec(vm);
}

fn print_vec(v: Vec<i32>) {
    for i in 0..v.len() - 1 {
        print!("{} ", v[i]);
    }
    println!("{}", v[v.len() - 1]);
}
