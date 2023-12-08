use sokoban_level_generator::{generate_level, encode_level};

fn main() {
    let level = generate_level(2, 3, 2);
    println!("Encode level");
    println!("{}", encode_level(&level));
}
