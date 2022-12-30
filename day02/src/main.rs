use std::fs;

enum Hand {
    Rock,
    Paper,
    Scissor,
}

enum Game {
    Win,
    Lose,
    Draw,
}

fn main() {
    // let file = fs::read_to_string("../input.txt").expect("Error getting input");

    // TODO: calculate scores
    // TODO: add clippy rust 
    
    let game: (Hand, Hand) = (Hand::Rock, Hand::Scissor);
    match game {
        (Hand::Rock, Hand::Rock) => Game::Draw,
        (Hand::Rock, Hand::Paper) => Game::Lose,
        (Hand::Rock, Hand::Scissor) => todo!(),
        (Hand::Paper, Hand::Rock) => todo!(),
        (Hand::Paper, Hand::Paper) => todo!(),
        (Hand::Paper, Hand::Scissor) => todo!(),
        (Hand::Scissor, Hand::Rock) => todo!(),
        (Hand::Scissor, Hand::Paper) => todo!(),
        (Hand::Scissor, Hand::Scissor) => todo!(),
    }
}
