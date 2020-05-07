#[macro_use]
mod input;

fn main() {
    input! {
        r: usize,
        c: usize,
        input_table: [[i32; c]; r],
    }
    let mut output_table = vec![vec![0; c]; r + 1];
    for y in 0..r {
        for x in 0..c {
            output_table[y][x] = input_table[y][x];
        }
    }
    for x in 0..c {
        let mut total = 0;
        for y in 0..r {
            total += input_table[y][x];
        }
        output_table[r][x] = total;
    }
    print_table_sum(output_table);
}

fn print_table_sum(table: Vec<Vec<i32>>) {
    for line in table {
        let s = line
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        print!("{}", s);
        println!(" {}", line.iter().sum::<i32>())
    }
}
