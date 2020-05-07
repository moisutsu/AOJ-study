#[macro_use]
mod input;

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        a: [[i32; m]; n],
        b: [[i32; l]; m],
    }
    let mut out = vec![vec![0; l]; n];
    for a_i in 0..n {
        for b_i in 0..l {
            let mut total = 0;
            for i in 0..m {
                total += a[a_i][i] * b[i][b_i];
            }
            out[a_i][b_i] = total;
        }
    }
    print_matrix(out);
}

fn print_matrix(matrix: Vec<Vec<i32>>) {
    let s = matrix
        .iter()
        .map(|line| {
            line.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", s);
}
