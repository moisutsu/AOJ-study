#[macro_use]
mod input;

fn main() {
    input! {
        n: usize,
        lines: [(usize, usize, usize, usize); n],
    }
    let mut buildings = [[[0; 10]; 3]; 4];
    for line in lines {
        buildings[line.0 - 1][line.1 - 1][line.2 - 1] = if line.3 > 0 { line.3 } else { 0 }
    }
    print_buildings(buildings);
}

fn print_buildings(buildings: [[[usize; 10]; 3]; 4]) {
    let wall = "#".to_string().repeat(20);
    for building in buildings.iter() {
        for line in building.iter() {
            let s = line
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("");
            println!("{}", s);
        }
        println!("{}", wall);
    }
}
