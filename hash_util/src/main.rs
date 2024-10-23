use std::fs;
use std::hash::{Hash, Hasher};
use std::io::{self, Write};
use std::path::Path;
use sha2::{Sha256, Digest};

fn main() {
    loop {
        println!("Select an option:");
        println!("1: Hash a string");
        println!("2: Hash a file");
        println!("0: Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice = choice.trim();

        match choice {
            "1" => {
                let mut input_string = String::new();
                println!("Enter the string to hash:");
                io::stdin()
                    .read_line(&mut input_string)
                    .expect("Failed to read line");
                let hash = hash_string(input_string.trim());
                println!("Hash: {}", hash);
            }
            "2" => {
                let mut file_path = String::new();
                println!("Enter the file path:");
                io::stdin()
                    .read_line(&mut file_path)
                    .expect("Failed to read line");
                let file_path = file_path.trim();

                match hash_file(file_path) {
                    Ok(hash) => println!("Hash: {}", hash),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "0" => break,
            _ => println!("Invalid choice. Please select 1, 2, or 0."),
        }
    }

    println!("Utility exited.");
}

fn hash_string(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    format!("{:x}", hasher.finalize())
}

fn hash_file(file_path: &str) -> Result<String, String> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err("File does not exist.".to_string());
    }

    let contents = fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;
    let hash = hash_string(&String::from_utf8_lossy(&contents));
    Ok(hash)
}
