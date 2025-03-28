use std::io::{stdin, stdout, Write};
use rand::Rng;

enum Difficulties{
    Easy,
    Medium,
    Hard,
    VeryHard
}

fn welcome_message(){
    println!("== Welcome to the number guessing game ==");
    print!("\n");
    println!("- The game will have 3 difficulties, which will influence the");
    print!("\n");
    println!("- number of chances you have to guess the secret number");
}

fn options_message(){
    println!("(1) - Easy\n(2) - Medium\n(3) - Hard\n(4) - VeryHard");
    println!("- Obs: If you choose an option outside the scope, the 'easy' difficulty will be the default.");
    print!("\n");
    println!("- What is the desired difficulty?")
}

fn analyze_guess(player_guess: usize, secret_num: usize){
    if player_guess > secret_num {
        println!("Your number is greater than the secret number");
    } else {
        println!("Your number is smaller than the secret number");
    }
}

fn main() {

    let mut maximum_guess: u32 = 12;
    let mut rng = rand::rng();
    let mut secret_num = rng.random_range(1..=100);
    let difficulties = vec!(Difficulties::Easy, Difficulties::Medium, Difficulties::Hard, Difficulties::VeryHard);
    let mut player_error_quantity: u32 = 0;

    welcome_message();
    options_message();

    print!("> ");
    stdout().flush().unwrap();

    let mut player_choosing: String = String::new();
    stdin()
        .read_line(&mut player_choosing)
        .expect("input error");

    let player_choosing: usize = player_choosing.trim().parse::<usize>().unwrap_or_else(|_| 1);


    if player_choosing > 0 && player_choosing <= difficulties.len() {
        match difficulties.get(player_choosing - 1) {
            Some(Difficulties::Easy) => { maximum_guess = 12; }
            Some(Difficulties::Medium) => { maximum_guess = 8; }
            Some(Difficulties::Hard) => { maximum_guess = 5; }
            Some(Difficulties::VeryHard) => { maximum_guess = 3; }
            _ => {
                println!("Erro")
            }
        }
    } else {
        maximum_guess = 12;
        println!("Invalid option. Easy difficulty set as default")
    }

    loop {

        let mut replay = false;
        let mut replay_option:String = String::new();

        if player_error_quantity <= maximum_guess {
            print!("Enter your guess: ");
            stdout()
                .flush()
                .unwrap();

            let mut player_guess: String = String::new();
            stdin()
                .read_line(&mut player_guess)
                .expect("Error writing your guess");

            match player_guess.trim().parse() {
                Ok(num) => {
                    if num == secret_num{
                        println!("Winner!!");
                        println!("\nAttempts: {}", player_error_quantity);

                        println!("This is a secret number: {}", secret_num);
                        println!(" ");
                    } else {
                        analyze_guess(num, secret_num);
                        player_error_quantity += 1;
                        continue;
                    }
                }
                Err(_) => println!("Insert only positive numbers")
            }

        } else {
            println!("This is a secret number: {}", secret_num);
            println!(" ");
        }

        println!("Do you want to play again?: (yes) (no)");

        print!("> ");
        stdout()
            .flush()
            .unwrap();
        stdin()
            .read_line(&mut replay_option)
            .expect("Choice error");

        replay = if replay_option.trim().to_lowercase() == "yes" { true } else { false };

        if replay {
            player_error_quantity = 0;
            secret_num = rng.random_range(1..=100);
            replay = false;
            continue;
        }
        println!("Game over, Thanks for playing.");
        break;
    }

}
