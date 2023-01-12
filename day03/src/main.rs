use std::{collections::HashSet, fs};

fn main() {
    part_one();
}

fn part_one() {
    let items = fs::read_to_string("../input.txt").unwrap();

    let items = items
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .collect::<Vec<(&str, &str)>>();

    println!("{:?}", items);
    // NOTE: Pour parser:
    // - https://stackoverflow.com/questions/52882267/how-to-find-if-two-strings-have-common-characters-in-rust
    // - https://doc.rust-lang.org/std/collections/struct.HashSet.html
    //
    // TODO: Faire la somme des priorités pour les types qui se chevauchent (a-z 1-26, A-Z 27-52)
}
