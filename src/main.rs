use colored::*;
use rand::prelude::*;

fn main() {
    let range = get_range();
    let number = rand::rng().random_range(1..range);
    let mut i = 5;
    if range <= 10 {
        i = 3;
    }
    while i > 0 {
        let mut line = String::new();
        println!("[{} {}]", "Attempt".blue(), i.to_string().green().bold());
        std::io::stdin().read_line(&mut line).expect("Failed to read line");
        match line.trim().parse::<i32>() {
            Ok(x) => {
                if x == number {
                    println!("{} 🎉: {}", "Congratulations!".green().italic(), "You guessed the number!");
                    break;
                } else if x < number {
                    println!("{}: {}", "Hint".bright_yellow().bold(), "The number is greater than your guess.".italic());
                    i -= 1;
                } else if x > number {
                    println!("{}: {}", "Hint".bright_yellow().bold(), "The number is less than your guess.".italic());
                    i -= 1;
                } else if i == 1 {
                    println!("{}: {}", "Game over!".red().bold(), "You ran out of attempts.".red());
                    break;
                }
            }
            Err(_) => { eprintln!("{}", "Invalid number — please try again.".red()); }
        }
    }
}

fn get_range() -> i32 {
    loop {
        let mut line = String::new();
        println!("{} Enter the range: ", "?".blue());
        std::io::stdin().read_line(&mut line).expect("Failed to read line");
        match line.trim().parse::<i32>() {
            Ok(range) => {
                if range <= 1 {
                    eprintln!("{}", "Invalid range — please try again.".red());
                    continue;
                }
                println!("{{ Range: {} }}", range.to_string().blue().bold());
                return range;
            }
            Err(_) => { eprintln!("{}", "Invalid number — please try again.".red()); }
        }
    }
}