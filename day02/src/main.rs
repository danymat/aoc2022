use std::fs;

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
fn calculate_score(game: &str) -> i32 {
    let (opponent, yours): (i32, i32) = match game {
        "A X" => (1, 1),
        "A Y" => (1, 2),
        "A Z" => (1, 3),
        "B X" => (2, 1),
        "B Y" => (2, 2),
        "B Z" => (2, 3),
        "C X" => (3, 1),
        "C Y" => (3, 2),
        "C Z" => (3, 3),
        _ => (0, 0),
    };
    let mut score = 0;
    score += yours;
    if opponent == yours {
        score += 3;
    } else if (yours - opponent) == 1 || (yours == 1 && opponent == 3) {
        score += 6;
    }
    score
}

fn part_one() -> i32 {
    let file = fs::read_to_string("../input.txt").expect("Error getting input");
    let score: i32 = file.lines().map(|line| calculate_score(line)).sum();
    score
}

fn part_two() -> i32 {
    let file = fs::read_to_string("../input.txt").expect("Could not read input file");
    // X --> Je dois perdre
    // Y --> draw
    // Z --> win
    // Je peux reutiliser la même fonction mais remplacer la deuxieme par

    let score: i32 = file
        .lines()
        .map(|line| {
            let edited_line: &str = match line {
                "A X" => "A Z",
                "A Y" => "A X",
                "A Z" => "A Y",
                "B X" => "B X",
                "B Y" => "B Y",
                "B Z" => "B Z",
                "C X" => "C Y",
                "C Y" => "C Z",
                "C Z" => "C X",
                _ => "",
            };
            calculate_score(edited_line)
        })
        .sum();
    score
}
