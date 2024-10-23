use rand::Rng;
use std::io::{self, Write};

fn main() {
    let length = get_length();
    let include_special = get_boolean_input("Include special characters? (y/n): ");
    let include_numbers = get_boolean_input("Include numbers? (y/n): ");

    let password = generate_password(length, include_special, include_numbers);
    println!("Generated password: {}", password);
}

fn get_length() -> usize {
    loop {
        let mut input = String::new();
        println!("Enter the desired password length (minimum 1):");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<usize>() {
            Ok(length) if length > 0 => return length,
            _ => println!("Please enter a valid positive number."),
        }
    }
}

fn get_boolean_input(prompt: &str) -> bool {
    loop {
        let mut input = String::new();
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "y" => return true,
            "n" => return false,
            _ => println!("Invalid input. Please enter 'y' or 'n'."),
        }
    }
}

fn generate_password(length: usize, include_special: bool, include_numbers: bool) -> String {
    let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let numbers: Vec<char> = "0123456789".chars().collect();
    let special_characters: Vec<char> = "!@#$%^&*()-_=+[]{}|;:,.<>?/`~".chars().collect();

    let mut characters = letters.clone();

    if include_numbers {
        characters.extend(&numbers);
    }
    if include_special {
        characters.extend(&special_characters);
    }

    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| characters[rng.gen_range(0..characters.len())])
        .collect()
}
