use std::io;

fn main() {
    loop {
        println!("Select a utility:");
        println!("1: Convert string to lowercase");
        println!("2: Convert string to uppercase");
        println!("3: Check for spaces in a string");
        println!("4: Find max in a list of numbers");
        println!("5: Find min in a list of numbers");
        println!("0: Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice = choice.trim();

        match choice {
            "1" => {
                let input = read_string("Enter a string:");
                println!("Lowercase: {}", input.to_lowercase());
            }
            "2" => {
                let input = read_string("Enter a string:");
                println!("Uppercase: {}", input.to_uppercase());
            }
            "3" => {
                let input = read_string("Enter a string:");
                if input.contains(' ') {
                    println!("The string contains spaces.");
                } else {
                    println!("The string does not contain spaces.");
                }
            }
            "4" => {
                let numbers = read_numbers("Enter numbers separated by spaces:");
                if let Some(max) = numbers.iter().cloned().max() {
                    println!("Max: {}", max);
                } else {
                    println!("No numbers provided.");
                }
            }
            "5" => {
                let numbers = read_numbers("Enter numbers separated by spaces:");
                if let Some(min) = numbers.iter().cloned().min() {
                    println!("Min: {}", min);
                } else {
                    println!("No numbers provided.");
                }
            }
            "0" => break,
            _ => println!("Invalid choice. Please select 1, 2, 3, 4, 5, or 0."),
        }
    }

    println!("Utility exited.");
}

fn read_string(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn read_numbers(prompt: &str) -> Vec<i32> {
    let input = read_string(prompt);
    input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}
