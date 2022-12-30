use std::fs;

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> i32 {
    // Read file and output to variable
    let file = fs::read_to_string("../input.txt").expect("Error in opening file");

    // Creates a vector of i32 vectors from the file
    let groups: Vec<Vec<i32>> = file
        .split("\n\n") // splits into a vec of strs
        .map(|e| {
            e.lines()
                .map(|x| x.parse().unwrap()) // Change type to i32
                .collect()
        })
        .collect();

    // Rearrange the groups to be the sums of all values in the groups
    let summed_groups: Vec<i32> = groups.iter().map(|e| e.iter().sum()).collect();

    // Gets the max of summed groups and return it
    let max = summed_groups.iter().max().expect("Error in getting max");
    *max
}

fn part_two() -> i32 {
    let file = fs::read_to_string("../input.txt").expect("Error in opening files");

    let mut groups: Vec<i32> = file
        .split("\n\n") // splits into a vec of strs
        .map(|e| {
            e.lines()
                .map(|x| x.parse::<i32>().unwrap()) // Change type to i32
                .sum()
        })
        .collect();

    groups.sort_by(|a, b| b.cmp(a));
    groups.truncate(3);
    groups.iter().sum()
}
