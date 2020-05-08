use std::collections::HashMap;
use std::char::from_u32;
use std::io::*;

#[macro_use]
mod input;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let mut map = HashMap::new();
    for c in s.chars() {
        let count = *map.entry(c.to_ascii_lowercase()).or_insert(0) + 1;
        map.insert(c, count);
    }
    for c_i in 'a' as u32..'z' as u32 + 1 {
        let c = from_u32(c_i).unwrap();
        println!("{} : {}", c, map.entry(c).or_insert(0));
    }
}
