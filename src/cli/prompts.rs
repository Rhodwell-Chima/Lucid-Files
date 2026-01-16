use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};

pub fn prompt_line(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    let _ = stdout().flush();
    input.clear();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn prompt_path(prompt: &str, must_exist: bool) -> PathBuf {
    loop {
        let s = prompt_line(prompt);
        if s.is_empty() {
            println!("Input cannot be empty. Please try again.");
            continue;
        }
        let p = Path::new(&s).to_path_buf();
        if must_exist {
            if p.exists() {
                return p;
            } else {
                println!("Path does not exist. Please enter an existing path.");
                continue;
            }
        } else {
            if p.exists() {
                return p;
            } else {
                let mut yn = prompt_line("Destination does not exist. Create it? (y/n): ");
                yn.make_ascii_lowercase();
                if yn == "y" || yn == "yes" {
                    if let Err(e) = fs::create_dir_all(&p) {
                        println!("Failed to create directory: {}. Try again.", e);
                        continue;
                    }
                    return p;
                } else {
                    println!("Please enter a different destination.");
                    continue;
                }
            }
        }
    }
}

pub fn prompt_choice(prompt: &str, min: u8, max: u8) -> u8 {
    loop {
        let s = prompt_line(prompt);
        match s.parse::<u8>() {
            Ok(n) if n >= min && n <= max => return n,
            _ => {
                println!("Invalid choice. Enter a number between {} and {}.", min, max);
            }
        }
    }
}