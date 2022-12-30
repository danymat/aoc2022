use std::fs;

fn main() {
    let file = fs::read_to_string("../input.txt").expect("Error getting input");
    println!("{}", file);
}
