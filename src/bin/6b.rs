#[macro_use]
mod input;

fn main() {
    input! {
        n: usize,
        cards: [(char, usize); n],
    }
    for suit in vec!['S', 'H', 'C', 'D'] {
        for num in 1..14 {
            if !cards.contains(&(suit, num)) {
                println!("{} {}", suit, num);
            }
        }
    }
}
