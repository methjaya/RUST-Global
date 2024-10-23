use std::io;

fn main() {
    loop {
        // Read user input
        let mut input = String::new();
        println!("Enter an operation (e.g., 3 + 4) or 'exit' to quit:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Trim whitespace and check for exit command
        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        // Split the input into components
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 3 {
            println!("Invalid input. Please use the format: number1 operator number2");
            continue;
        }

        let num1: f64 = match parts[0].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number: {}", parts[0]);
                continue;
            }
        };
        
        let operator = parts[1];
        let num2: f64 = match parts[2].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number: {}", parts[2]);
                continue;
            }
        };

        // Perform the calculation
        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Error: Division by zero");
                    continue;
                }
                num1 / num2
            },
            _ => {
                println!("Invalid operator: {}", operator);
                continue;
            }
        };

        // Print the result
        println!("Result: {}", result);
    }

    println!("Calculator exited.");
}
