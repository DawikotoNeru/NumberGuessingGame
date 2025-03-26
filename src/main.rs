use rand;
use std::io::{stdin, stdout, Write};

enum Difficulties{
    Easy,
    Medium,
    Hard,
    VeryHard
}

fn welcome_message(){
    println!("Welcome to the number guessing game\n
    \n
    The game will have 3 difficulties, which will influence the \n
    number of chances you have to guess the secret number");
}

fn options_message(){
    println!("(1) - Easy\n(2) - Medium\n(3) - Hard\n(4) - VeryHard");
    println!("What is the desired difficulty?")
}

fn main() {

    let mut rng = rand::rng();
    let difficulties = vec!(Difficulties::Easy, Difficulties::Medium, Difficulties::Hard, Difficulties::VeryHard);


    welcome_message();
    options_message();

    print!("> ");
    stdout().flush().err();

    let mut player_choosing: String = String::new();
    stdin()
        .read_line(&mut player_choosing)
        .expect("input error");

    let player_choosing = player_choosing.trim().parse::<usize>().unwrap_or_else(|_| 0);


    match difficulties.get(player_choosing-1){
        Some(Difficulties::Easy) => {
            const MAXIMUM_GUESS: u32 = 12;
            println!("PRINT EASY");
        }

        Some(Difficulties::Medium) => {
            const MAXIMUM_GUESS: u32 = 8;
            println!("PRINT MEDIUM");
        }

        Some(Difficulties::Hard) => {
            const MAXIMUM_GUESS: u32 = 5;
            println!("PRINT HARD");
        }

        Some(Difficulties::VeryHard) => {
            const MAXIMUM_GUESS: u32 = 3;
            println!("PRINT VERY HARD");
        }

        _ => {
            println!("This option does not exist")
        }
    }

}
