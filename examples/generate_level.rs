use sokoban_level_generator::{generate_level, level_to_string};

fn main() {
    let level = generate_level(4, 4, 3);
    println!("Print level");
    println!("{}", level_to_string(&level));
}
