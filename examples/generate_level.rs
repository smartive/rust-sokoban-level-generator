use sokoban_level_generator::{generate_level, pretty_print_level};

fn main() {
    let level = generate_level(4, 4, 3);
    println!("Print level");
    println!("{}", pretty_print_level(&level));
}
