use rand::Rng;
use std::process::Command;
use std::io::{stdin, stdout, Write};

enum Difficulties {
    Easy,
    Medium,
    Hard,
    VeryHard,
}

fn clear_terminal(){
    Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
}

fn secret_num_message(secret_num: usize) {
    println!(" ");
    println!("This is a secret number: {}", secret_num);
    println!(" ");
}
fn welcome_message() {
    println!("== Welcome to the number guessing game ==");
    println!(" ");
    println!("- The game will have 4 difficulties, which will influence the");
    println!("- number of chances you have to guess the secret number");
}

fn options_message() {
    println!(" ");
    println!("(1) - Easy (12 chances)\n(2) - Medium (8 chances)\n(3) - Hard (5 chances)\n(4) - VeryHard (3 chances)\n(0) - Quit");
    println!(" ");
    println!("- Obs: If you choose an option outside the scope, the 'easy' difficulty will be the default.");
    println!(" ");
    println!("- What is the desired difficulty?")
}

fn analyze_guess(player_guess: usize, secret_num: usize) {
    if player_guess > secret_num {
        println!("Your number is greater than the secret number");
        println!(" ");
    } else {
        println!("Your number is smaller than the secret number");
        println!(" ");
    }
}

fn show_score(score: usize) {
    println!("|🐤  Your score: {} 🐤|", score);
}
fn main() {
    let mut maximum_guess: u32 = 12;
    let mut rng = rand::rng();
    let mut secret_num = rng.random_range(1..=100);
    let difficulties = vec![
        Difficulties::Easy,
        Difficulties::Medium,
        Difficulties::Hard,
        Difficulties::VeryHard,
    ];
    let mut player_error_quantity: u32 = 0;
    let mut player_score: usize = 0;
    let mut reward: usize = 100;

    welcome_message();
    options_message();

    print!("> ");
    stdout().flush().unwrap();

    let mut player_choosing: String = String::new();
    stdin()
        .read_line(&mut player_choosing)
        .expect("input error");

    let player_choosing: usize = player_choosing
        .trim()
        .parse::<usize>()
        .unwrap_or_else(|_| 1);

    if player_choosing > 0 && player_choosing <= difficulties.len() {
        match difficulties.get(player_choosing - 1) {
            Some(Difficulties::Easy) => {
                maximum_guess = 12;
                reward = 100;
            }
            Some(Difficulties::Medium) => {
                maximum_guess = 8;
                reward = 250;
            }
            Some(Difficulties::Hard) => {
                maximum_guess = 5;
                reward = 400;
            }
            Some(Difficulties::VeryHard) => {
                maximum_guess = 3;
                reward = 550;
            }
            _ => {
                println!("Erro")
            }
        }
    } else if player_choosing == 0 {
        println!("Ending game....");
        return;
    } else {
        maximum_guess = 12;
        println!("Invalid option. Easy difficulty set as default")
    }

    let reward_copy = reward.clone();

    clear_terminal();

    loop {
        let mut replay_option: String = String::new();

        if player_error_quantity <= maximum_guess {
            print!("Enter your guess: ");
            stdout().flush().unwrap();

            let mut player_guess: String = String::new();
            stdin()
                .read_line(&mut player_guess)
                .expect("Error writing your guess");

            println!(" ");

            match player_guess.trim().parse() {
                Ok(num) => {
                    if num == secret_num {
                        println!("|🎂  Winner!! 🎂 |");
                        println!("\nAttempts: {}", player_error_quantity);
                        player_score += reward;
                        println!(" ");
                        println!("+{}", reward);
                        println!(" ");
                        show_score(player_score);

                        secret_num_message(secret_num);
                    } else {
                        analyze_guess(num, secret_num);
                        if reward!=0 {reward -= 5;}

                        player_error_quantity += 1;
                        continue;
                    }
                }
                Err(_) => { println!("Insert only positive numbers"); continue; }
            }
        } else {
            println!("|❌  You lost the game ❌ |");
            secret_num_message(secret_num);
            show_score(player_score);
            player_score = 0;
        }

        println!("> Do you want to play again?: (yes) (no)");

        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut replay_option).expect("Choice error");

        let replay: bool = if replay_option.trim().to_lowercase() == "yes" {
            true
        } else {
            false
        };

        if replay {
            clear_terminal();
            println!(" ");
            println!("Starting new round...");
            println!(" ");
            player_error_quantity = 0;
            secret_num = rng.random_range(1..=100);
            reward = reward_copy;
            continue;
        }
        clear_terminal();
        println!(" ");
        println!("🌞 Game over, Thanks for playing 🌞");
        break;
    }
}
